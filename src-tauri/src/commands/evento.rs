use serde::Deserialize;
use tauri::State;

use crate::db::{models::{Evento, Recordatorio}, queries};
use crate::AppState;

fn map_err<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrearEventoParams {
    pub entry_id: Option<String>,
    pub espacio_id: String,
    pub proyecto_id: Option<String>,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub inicio_at: String,
    pub fin_at: String,
    pub all_day: bool,
    pub ubicacion: Option<String>,
    pub rrule: Option<String>,
    pub color: Option<String>,
}

#[tauri::command]
pub fn crear_evento(state: State<'_, AppState>, params: CrearEventoParams) -> Result<String, String> {
    queries::create_evento(
        &state.db,
        queries::NewEvento {
            entry_id: params.entry_id.as_deref(),
            espacio_id: &params.espacio_id,
            proyecto_id: params.proyecto_id.as_deref(),
            titulo: &params.titulo,
            descripcion: params.descripcion.as_deref(),
            inicio_at: &params.inicio_at,
            fin_at: &params.fin_at,
            all_day: params.all_day,
            ubicacion: params.ubicacion.as_deref(),
            rrule: params.rrule.as_deref(),
            color: params.color.as_deref(),
        },
    )
    .map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListarEventosParams {
    pub desde: String,
    pub hasta: String,
    pub espacio_id: Option<String>,
}

#[tauri::command]
pub fn listar_eventos(
    state: State<'_, AppState>,
    params: ListarEventosParams,
) -> Result<Vec<Evento>, String> {
    queries::backfill_eventos_desde_recordatorios(&state.db).ok();
    queries::list_eventos_en_rango(
        &state.db,
        &params.desde,
        &params.hasta,
        params.espacio_id.as_deref(),
    )
    .map_err(map_err)
}

#[tauri::command]
pub fn obtener_evento(state: State<'_, AppState>, evento_id: String) -> Result<Option<Evento>, String> {
    queries::get_evento(&state.db, &evento_id).map_err(map_err)
}

#[tauri::command]
pub fn obtener_evento_por_entrada(
    state: State<'_, AppState>,
    entry_id: String,
) -> Result<Option<Evento>, String> {
    queries::get_evento_por_entrada(&state.db, &entry_id).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActualizarEventoParams {
    pub evento_id: String,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub inicio_at: String,
    pub fin_at: String,
    pub all_day: bool,
    pub espacio_id: String,
    pub proyecto_id: Option<String>,
    pub ubicacion: Option<String>,
    pub rrule: Option<String>,
    pub color: Option<String>,
}

#[tauri::command]
pub fn actualizar_evento(state: State<'_, AppState>, params: ActualizarEventoParams) -> Result<(), String> {
    queries::update_evento(
        &state.db,
        &params.evento_id,
        &params.titulo,
        params.descripcion.as_deref(),
        &params.inicio_at,
        &params.fin_at,
        params.all_day,
        &params.espacio_id,
        params.proyecto_id.as_deref(),
        params.ubicacion.as_deref(),
        params.rrule.as_deref(),
        params.color.as_deref(),
    )
    .map_err(map_err)
}

#[tauri::command]
pub fn eliminar_evento(state: State<'_, AppState>, evento_id: String) -> Result<(), String> {
    queries::delete_evento(&state.db, &evento_id).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FijarRecordatorioParams {
    pub entry_id: String,
    pub titulo: String,
    pub dispara_at: String,
}

#[tauri::command]
pub fn fijar_recordatorio(
    state: State<'_, AppState>,
    params: FijarRecordatorioParams,
) -> Result<(), String> {
    queries::reemplazar_recordatorio_entrada(
        &state.db,
        &params.entry_id,
        &params.titulo,
        &params.dispara_at,
    )
    .map(|_| ())
    .map_err(map_err)
}

#[tauri::command]
pub fn obtener_recordatorio_por_entrada(
    state: State<'_, AppState>,
    entry_id: String,
) -> Result<Option<Recordatorio>, String> {
    queries::get_recordatorio_activo_por_entrada(&state.db, &entry_id).map_err(map_err)
}
