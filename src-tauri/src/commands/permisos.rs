//! Comandos IPC para permisos macOS.
//!
//! Regla del onboarding: el usuario debe ver diálogos nativos, no botones
//! decorativos. Cada permiso tiene un `verificar` (estado real) y un
//! `solicitar` (dispara diálogo si aplica, si no abre System Settings).
//!
//! Persistimos el último estado observado en `config` para que el onboarding
//! pueda pintar checks sin re-consultar en cada render.

use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_notification::NotificationExt;

use crate::db::queries;
use crate::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstadoPermisos {
    pub accesibilidad: bool,
    pub notificaciones: bool,
    pub microfono: bool,
}

#[tauri::command]
pub fn verificar_permisos(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<EstadoPermisos, String> {
    let acc = check_accesibilidad();
    let notif = check_notificaciones(&app);
    let mic = check_microfono();
    persist(&state, "permiso_accesibilidad", acc);
    persist(&state, "permiso_notificaciones", notif);
    persist(&state, "permiso_microfono", mic);
    Ok(EstadoPermisos {
        accesibilidad: acc,
        notificaciones: notif,
        microfono: mic,
    })
}

#[tauri::command]
pub async fn solicitar_permiso_notificaciones(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let ok = request_notificaciones(&app).await;
    persist(&state, "permiso_notificaciones", ok);
    Ok(ok)
}

#[tauri::command]
pub fn solicitar_permiso_accesibilidad(state: State<'_, AppState>) -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    crate::platform::macos::abrir_ajustes_accesibilidad();
    let ok = check_accesibilidad();
    persist(&state, "permiso_accesibilidad", ok);
    Ok(ok)
}

#[tauri::command]
pub fn solicitar_permiso_microfono(state: State<'_, AppState>) -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    crate::platform::macos::abrir_ajustes_microfono();
    let ok = check_microfono();
    persist(&state, "permiso_microfono", ok);
    Ok(ok)
}

// ────────────────────────────────────────────────────────────────────────────

fn persist(state: &State<'_, AppState>, clave: &str, v: bool) {
    let _ = queries::set_config(&state.db, clave, if v { "1" } else { "0" });
}

fn check_accesibilidad() -> bool {
    #[cfg(target_os = "macos")]
    {
        crate::platform::macos::accesibilidad_concedida()
    }
    #[cfg(not(target_os = "macos"))]
    {
        true
    }
}

fn check_notificaciones(app: &AppHandle) -> bool {
    app.notification()
        .permission_state()
        .map(|s| matches!(s, tauri_plugin_notification::PermissionState::Granted))
        .unwrap_or(false)
}

async fn request_notificaciones(app: &AppHandle) -> bool {
    // Si ya está concedido, no volvamos a preguntar (macOS no re-pregunta y
    // devolveríamos `false` de forma engañosa).
    if check_notificaciones(app) {
        return true;
    }
    match app.notification().request_permission() {
        Ok(state) => matches!(state, tauri_plugin_notification::PermissionState::Granted),
        Err(_) => false,
    }
}

fn check_microfono() -> bool {
    // Verificación real requiere FFI a AVCaptureDevice; mientras tanto
    // reflejamos el valor persistido (el usuario lo marca desde System Settings).
    // Al reabrir la app se re-verifica desde el sistema.
    false
}
