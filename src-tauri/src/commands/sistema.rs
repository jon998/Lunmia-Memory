use serde::Serialize;
use tauri::{AppHandle, Emitter, LogicalPosition, LogicalSize, Manager, Monitor, State};

/// Tamaño lógico del cuadro de captura en modo input (§7.2).
/// El frontend puede pedir un tamaño distinto para el resumen; al reabrir
/// desde el atajo o el tray, volvemos a este tamaño por defecto.
const CAPTURA_ANCHO: f64 = 620.0;
const CAPTURA_ALTO: f64 = 200.0;
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_opener::OpenerExt;
use tracing::warn;

use crate::db::{queries, Database};
use crate::AppState;

// Cursor global en macOS. `CGEventGetLocation` devuelve puntos (unidades
// lógicas de AppKit), origen top-left del monitor primario. Es el mismo
// espacio que Tauri usa para `LogicalPosition`, así que sirve para elegir
// el monitor y posicionar la ventana sin ir por píxeles físicos.
#[cfg(target_os = "macos")]
mod cursor_macos {
    use core::ffi::c_void;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGPoint {
        x: f64,
        y: f64,
    }

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGEventCreate(source: *const c_void) -> *mut c_void;
        fn CGEventGetLocation(event: *mut c_void) -> CGPoint;
        fn CFRelease(cf: *mut c_void);
    }

    pub fn posicion_logica() -> Option<(f64, f64)> {
        unsafe {
            let event = CGEventCreate(core::ptr::null());
            if event.is_null() {
                return None;
            }
            let p = CGEventGetLocation(event);
            CFRelease(event);
            Some((p.x, p.y))
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod cursor_macos {
    pub fn posicion_logica() -> Option<(f64, f64)> {
        None
    }
}

fn monitor_bajo_cursor(app: &AppHandle) -> Option<Monitor> {
    if let Some((cx, cy)) = cursor_macos::posicion_logica() {
        if let Ok(monitors) = app.available_monitors() {
            for m in monitors {
                let scale = m.scale_factor();
                let x = m.position().x as f64 / scale;
                let y = m.position().y as f64 / scale;
                let w = m.size().width as f64 / scale;
                let h = m.size().height as f64 / scale;
                if cx >= x && cx < x + w && cy >= y && cy < y + h {
                    return Some(m);
                }
            }
        }
    }
    app.primary_monitor().ok().flatten()
}

/// Recoloca la ventana de captura en el monitor donde está el cursor,
/// centrada horizontal y a ~28% de la altura (estilo Spotlight).
/// Silencioso ante fallos: si el monitor o el tamaño no están disponibles,
/// deja la ventana donde estaba.
pub fn centrar_captura_en_cursor(app: &AppHandle) {
    let Some(win) = app.get_webview_window("captura") else {
        return;
    };
    let Some(monitor) = monitor_bajo_cursor(app) else {
        return;
    };
    let Ok(size) = win.outer_size() else {
        return;
    };

    let scale = monitor.scale_factor();
    let mx = monitor.position().x as f64 / scale;
    let my = monitor.position().y as f64 / scale;
    let mw = monitor.size().width as f64 / scale;
    let mh = monitor.size().height as f64 / scale;
    let ww = size.width as f64 / scale;
    let wh = size.height as f64 / scale;

    let x = mx + ((mw - ww) / 2.0).max(0.0);
    let y = my + ((mh - wh) * 0.28).max(0.0);

    let _ = win.set_position(LogicalPosition::new(x.round(), y.round()));
}

fn map_err<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoSistema {
    pub llm_activo: bool,
    /// Presente si el ping falló: key, red u Ollama caído.
    pub llm_aviso: Option<String>,
    pub llm_proveedor: String,
    pub llm_modelo: String,
    pub ocr_disponible: bool,
    pub version: String,
}

#[tauri::command]
pub async fn info_sistema(state: State<'_, AppState>) -> Result<InfoSistema, String> {
    let (llm_activo, llm_aviso) = state.llm.diagnostico().await;
    Ok(InfoSistema {
        llm_activo,
        llm_aviso,
        llm_proveedor: state.llm.provider_name().to_string(),
        llm_modelo: state.llm.classifier_model.clone(),
        ocr_disponible: state.ocr.disponible(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[tauri::command]
pub fn abrir_ventana_captura(app: AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("captura") {
        let _ = win.set_size(LogicalSize::new(CAPTURA_ANCHO, CAPTURA_ALTO));
        centrar_captura_en_cursor(&app);
        let _ = win.show();
        let _ = win.set_focus();
        let _ = app.emit("captura:abrir", ());
    }
    Ok(())
}

#[tauri::command]
pub fn cerrar_ventana_captura(app: AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("captura") {
        let _ = win.hide();
    }
    Ok(())
}

/// Vuelve a centrar la ventana de captura tras un cambio de tamaño del
/// frontend (paso de input a resumen). Sin esto la ventana crece desde su
/// esquina superior-izquierda y aparenta "bajarse" en cada apertura.
#[tauri::command]
pub fn recentrar_ventana_captura(app: AppHandle) -> Result<(), String> {
    centrar_captura_en_cursor(&app);
    Ok(())
}

#[tauri::command]
pub fn abrir_ventana_principal(app: AppHandle) -> Result<(), String> {
    crate::mostrar_ventana_principal(&app);
    Ok(())
}

/// Aplica el LaunchAgent según lo guardado. No toca el sistema si aún no hay preferencia.
pub fn sincronizar_autostart(app: &AppHandle, db: &Database) {
    match queries::get_config(db, "autostart_habilitado") {
        Ok(Some(v)) if v == "1" => {
            if let Err(err) = app.autolaunch().enable() {
                warn!(%err, "no se pudo habilitar autostart");
            }
        }
        Ok(Some(v)) if v == "0" => {
            if let Err(err) = app.autolaunch().disable() {
                warn!(%err, "no se pudo deshabilitar autostart");
            }
        }
        _ => {}
    }
}

pub fn autostart_habilitado_en_db(db: &Database) -> bool {
    queries::get_config(db, "autostart_habilitado")
        .ok()
        .flatten()
        .as_deref()
        == Some("1")
}

/// Persiste la preferencia y registra o quita el LaunchAgent de macOS.
pub fn aplicar_autostart(app: &AppHandle, db: &Database, habilitar: bool) -> Result<(), String> {
    queries::set_config(db, "autostart_habilitado", if habilitar { "1" } else { "0" })
        .map_err(map_err)?;
    if habilitar {
        app.autolaunch().enable().map_err(|e| e.to_string())
    } else {
        app.autolaunch().disable().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn fijar_autostart(
    app: AppHandle,
    state: State<'_, AppState>,
    habilitar: bool,
) -> Result<(), String> {
    aplicar_autostart(&app, &state.db, habilitar)
}

#[tauri::command]
pub fn get_config(state: State<'_, AppState>, clave: String) -> Result<Option<String>, String> {
    queries::get_config(&state.db, &clave).map_err(map_err)
}

#[tauri::command]
pub fn set_config(state: State<'_, AppState>, clave: String, valor: String) -> Result<(), String> {
    queries::set_config(&state.db, &clave, &valor).map_err(map_err)
}

#[tauri::command]
pub fn ruta_log_llm(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.llm.log_path().to_string_lossy().to_string())
}

#[tauri::command]
pub fn abrir_log_llm(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let path = state.llm.log_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if !path.exists() {
        let _ = std::fs::File::create(&path);
    }
    app.opener()
        .open_path(path.to_string_lossy().to_string(), None::<&str>)
        .map_err(map_err)
}
