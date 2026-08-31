use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::{
    models::{Entrada, PreguntaPendiente, Stats},
    queries,
};
use crate::AppState;

fn map_err<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TarjetaBandeja {
    pub entrada: Entrada,
    pub preguntas: Vec<PreguntaPendiente>,
}

#[tauri::command]
pub fn listar_bandeja(state: State<'_, AppState>) -> Result<Vec<TarjetaBandeja>, String> {
    let items = queries::list_preguntas_pendientes(&state.db).map_err(map_err)?;
    Ok(items
        .into_iter()
        .map(|(entrada, preguntas)| TarjetaBandeja { entrada, preguntas })
        .collect())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponderParams {
    pub pregunta_id: String,
    pub opcion: Option<i64>,
    pub texto: Option<String>,
}

#[tauri::command]
pub fn responder_pregunta(
    state: State<'_, AppState>,
    params: ResponderParams,
) -> Result<(), String> {
    queries::responder_pregunta(&state.db, &params.pregunta_id, params.opcion, params.texto.as_deref())
        .map_err(map_err)
}

#[tauri::command]
pub fn descartar_pregunta(state: State<'_, AppState>, pregunta_id: String) -> Result<(), String> {
    queries::descartar_pregunta(&state.db, &pregunta_id).map_err(map_err)
}

#[tauri::command]
pub fn obtener_stats(state: State<'_, AppState>) -> Result<Stats, String> {
    queries::calcular_stats(&state.db).map_err(map_err)
}
