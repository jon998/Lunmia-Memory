//! Lunmia Memory — punto de entrada del backend Tauri.
//!
//! Arquitectura (§7.4):
//!  - `platform`: adaptadores de macOS detrás de traits
//!  - `db`      : SQLite (esquema completo §5)
//!  - `ai`      : cascada de clasificación (§6.2)
//!  - `ocr`     : Vision (macOS)
//!  - `scheduler`: recordatorios (§13 R1)
//!  - `commands`: IPC hacia la UI SvelteKit

pub mod ai;
pub mod commands;
pub mod db;
pub mod limites;
pub mod ocr;
pub mod platform;
pub mod scheduler;
pub mod zona;

use std::sync::Arc;

use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, RunEvent, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tracing::{info, warn};

use crate::ai::{llm::LlmClient, Pipeline};
use crate::db::Database;
use crate::ocr::OcrService;
use crate::platform::Platform;
use crate::scheduler::{Command as SchedCommand, Scheduler};

pub struct AppState {
    pub db: Arc<Database>,
    pub llm: Arc<LlmClient>,
    pub ocr: Arc<OcrService>,
    pub pipeline: Arc<Pipeline>,
    pub platform: Arc<Platform>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // .env en runtime: cwd (tauri dev), Contents/Resources del .app,
    // padres del binario, y Application Support. Si no hay archivo,
    // LlmClient usa las LUNMIA_* incrustadas al compilar (build.rs).
    // Un valor ya en el entorno gana.
    fn cargar_dotenv() {
        let mut dirs: Vec<std::path::PathBuf> = vec![".".into(), "..".into()];
        if let Ok(mut p) = std::env::current_exe() {
            // Contents/Resources del .app (por si se copia un .env al bundle).
            let mut resources = p.clone();
            resources.pop(); // MacOS
            resources.pop(); // Contents
            dirs.push(resources.join("Resources"));
            for _ in 0..8 {
                p.pop();
                if p.as_os_str().is_empty() {
                    break;
                }
                dirs.push(p.clone());
            }
        }
        if let Some(data) = dirs::data_local_dir() {
            dirs.push(data.join("Lunmia Memory"));
            dirs.push(data.join("Lunmia Memory (dev)"));
        }
        for d in dirs {
            let _ = dotenvy::from_path(d.join(".env.local"));
            let _ = dotenvy::from_path(d.join(".env"));
        }
    }

    cargar_dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let db_path = db::default_path();
    info!(?db_path, "abriendo SQLite");
    let db = Arc::new(Database::open(db_path).expect("no se pudo abrir la base de datos"));

    let platform = Arc::new(Platform::new());
    let llm = Arc::new(LlmClient::new());
    let ocr = Arc::new(OcrService::new(platform.clone()));
    let pipeline = Arc::new(Pipeline::new(db.clone(), llm.clone(), platform.clone()));

    let state = AppState {
        db,
        llm,
        ocr,
        pipeline,
        platform,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--silent"]),
        ))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state == ShortcutState::Pressed && es_atajo_captura(shortcut) {
                        toggle_ventana_captura(app.clone());
                    }
                })
                .build(),
        )
        .manage(state)
        .setup(|app| {
            // Vibrancy nativo (NSVisualEffectView) para la ventana de captura.
            // Sin esto, `transparent: true` deja el webview con un fondo gris
            // plano y no se ve efecto cristal alguno.
            #[cfg(target_os = "macos")]
            if let Some(win) = app.get_webview_window("captura") {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};
                if let Err(err) = apply_vibrancy(
                    &win,
                    NSVisualEffectMaterial::Sidebar,
                    Some(NSVisualEffectState::Active),
                    Some(14.0),
                ) {
                    warn!(?err, "no se pudo aplicar vibrancy a la ventana captura");
                }
            }

            // Atajo global: Cmd+Shift+Space por defecto (§7.2)
            if let Err(err) = registrar_atajo_predeterminado(app.handle()) {
                warn!(?err, "no se pudo registrar el atajo global");
            }
            // Icono en la barra de menú (§7.1)
            construir_tray(app.handle())?;
            // Scheduler de recordatorios (R1 crítico)
            let db_sched = app.state::<AppState>().db.clone();
            let sched = Scheduler::new(db_sched.clone(), app.handle().clone());
            let tx = sched.spawn();
            app.manage(SchedulerHandle(tx));

            commands::sistema::sincronizar_autostart(app.handle(), &db_sched);

            // Login item (--silent): bandeja + atajo, sin dashboard.
            // Cierre de ventanas: Accessory, sin icono en el Dock.
            if arranque_silencioso() {
                ir_a_segundo_plano(app.handle());
            } else {
                #[cfg(target_os = "macos")]
                {
                    let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
                }
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            // Cerrar el dashboard (o la captura) oculta la ventana. El proceso,
            // el tray y el atajo siguen vivos. Solo "Salir" del menú mata la app.
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                if window.label() == "main" {
                    ir_a_segundo_plano(window.app_handle());
                } else {
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::captura::capturar_texto,
            commands::captura::capturar_pantalla,
            commands::captura::estado_ocr,
            commands::entrada::listar_entradas,
            commands::entrada::listar_tipos,
            commands::entrada::crear_tipo,
            commands::entrada::obtener_entrada,
            commands::entrada::actualizar_contenido,
            commands::entrada::actualizar_clasificacion,
            commands::entrada::actualizar_etiquetas,
            commands::entrada::eliminar_entrada,
            commands::entrada::mover_entrada,
            commands::espacio::listar_espacios,
            commands::espacio::crear_espacio,
            commands::proyecto::listar_proyectos,
            commands::proyecto::crear_proyecto,
            commands::contexto::obtener_contexto_activo,
            commands::contexto::fijar_contexto,
            commands::contexto::destinos_captura,
            commands::inbox::listar_bandeja,
            commands::inbox::responder_pregunta,
            commands::inbox::descartar_pregunta,
            commands::inbox::obtener_stats,
            commands::onboarding::estado_onboarding,
            commands::onboarding::finalizar_onboarding,
            commands::onboarding::proponer_setup_inicial,
            commands::espacio::eliminar_espacio,
            commands::espacio::renombrar_espacio,
            commands::espacio::fijar_espacio,
            commands::espacio::cambiar_color_espacio,
            commands::proyecto::eliminar_proyecto,
            commands::proyecto::renombrar_proyecto,
            commands::proyecto::fijar_proyecto,
            commands::evento::crear_evento,
            commands::evento::listar_eventos,
            commands::evento::obtener_evento,
            commands::evento::obtener_evento_por_entrada,
            commands::evento::actualizar_evento,
            commands::evento::eliminar_evento,
            commands::evento::fijar_recordatorio,
            commands::evento::obtener_recordatorio_por_entrada,
            commands::sistema::info_sistema,
            commands::sistema::abrir_ventana_captura,
            commands::sistema::cerrar_ventana_captura,
            commands::sistema::recentrar_ventana_captura,
            commands::sistema::abrir_ventana_principal,
            commands::sistema::get_config,
            commands::sistema::set_config,
            commands::sistema::fijar_autostart,
            commands::sistema::ruta_log_llm,
            commands::sistema::abrir_log_llm,
            commands::permisos::verificar_permisos,
            commands::permisos::solicitar_permiso_notificaciones,
            commands::permisos::solicitar_permiso_accesibilidad,
            commands::permisos::solicitar_permiso_microfono,
        ])
        .build(tauri::generate_context!())
        .expect("error corriendo la aplicación")
        .run(|app, event| {
            match event {
                // Cmd+Q / menú Salir de macOS: code es None. Tray "Salir"
                // llama app.exit(0) y code es Some(0) — eso sí termina.
                RunEvent::ExitRequested { api, code, .. } => {
                    if code.is_none() {
                        api.prevent_exit();
                        ir_a_segundo_plano(app);
                    }
                }
                RunEvent::Reopen {
                    has_visible_windows, ..
                } => {
                    if !has_visible_windows {
                        mostrar_ventana_principal(app);
                    }
                }
                _ => {}
            }
        });
}

struct SchedulerHandle(#[allow(dead_code)] tokio::sync::mpsc::Sender<SchedCommand>);

fn arranque_silencioso() -> bool {
    std::env::args().any(|a| a == "--silent")
}

pub(crate) fn mostrar_ventana_principal(app: &AppHandle) {
    #[cfg(target_os = "macos")]
    {
        let _ = app.set_activation_policy(tauri::ActivationPolicy::Regular);
    }
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
}

pub(crate) fn ir_a_segundo_plano(app: &AppHandle) {
    for label in ["main", "captura"] {
        if let Some(w) = app.get_webview_window(label) {
            let _ = w.hide();
        }
    }
    #[cfg(target_os = "macos")]
    {
        let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    }
}

fn atajo_captura() -> Shortcut {
    Shortcut::new(
        Some(Modifiers::SUPER | Modifiers::SHIFT),
        Code::Space,
    )
}

fn es_atajo_captura(s: &Shortcut) -> bool {
    let esperado = atajo_captura();
    s.mods == esperado.mods && s.key == esperado.key
}

fn registrar_atajo_predeterminado(app: &AppHandle) -> anyhow::Result<()> {
    app.global_shortcut()
        .register(atajo_captura())
        .map_err(|e| anyhow::anyhow!("no se pudo registrar atajo: {e}"))
}

fn toggle_ventana_captura(app: AppHandle) {
    if let Some(win) = app.get_webview_window("captura") {
        let visible = win.is_visible().unwrap_or(false);
        if visible {
            let _ = win.hide();
        } else {
            let _ = win.set_size(tauri::LogicalSize::new(620.0, 200.0));
            commands::sistema::centrar_captura_en_cursor(&app);
            let _ = win.show();
            let _ = win.set_focus();
            let _ = app.emit("captura:abrir", ());
        }
    }
}

fn construir_tray(app: &AppHandle) -> tauri::Result<()> {
    let abrir = MenuItem::with_id(app, "abrir", "Abrir Lunmia", true, None::<&str>)?;
    let capturar = MenuItem::with_id(
        app,
        "capturar",
        "Capturar…\t⌘⇧Space",
        true,
        None::<&str>,
    )?;
    let bandeja = MenuItem::with_id(app, "bandeja", "Bandeja", true, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let autostart_on = app
        .try_state::<AppState>()
        .map(|s| commands::sistema::autostart_habilitado_en_db(&s.db))
        .unwrap_or(false);
    let autostart = CheckMenuItem::with_id(
        app,
        "autostart",
        "Abrir al iniciar sesión",
        true,
        autostart_on,
        None::<&str>,
    )?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let salir = MenuItem::with_id(app, "salir", "Salir", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&abrir, &capturar, &bandeja, &sep, &autostart, &sep2, &salir],
    )?;

    TrayIconBuilder::with_id("lunmia-tray")
        .icon(app.default_window_icon().cloned().unwrap())
        .icon_as_template(true)
        .tooltip("Lunmia Memory")
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event({
            let autostart_item = autostart.clone();
            move |app, event| match event.id.as_ref() {
                "abrir" => mostrar_ventana_principal(app),
                "capturar" => toggle_ventana_captura(app.clone()),
                "bandeja" => {
                    mostrar_ventana_principal(app);
                    let _ = app.emit("navegar", "/bandeja");
                }
                "autostart" => {
                    let checked = autostart_item.is_checked().unwrap_or(false);
                    if let Some(state) = app.try_state::<AppState>() {
                        if let Err(err) =
                            commands::sistema::aplicar_autostart(app, &state.db, checked)
                        {
                            warn!(%err, "no se pudo cambiar autostart");
                            let _ = autostart_item.set_checked(!checked);
                        }
                    }
                }
                "salir" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|_tray, _event| {
            // El menú se muestra automáticamente con show_menu_on_left_click(true).
            // El manejo del click derecho también lo cubre Tauri por defecto.
        })
        .build(app)?;
    Ok(())
}
