use serde::Deserialize;
use tauri::State;

use crate::db::{models::Espacio, queries};
use crate::AppState;

fn map_err<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

#[tauri::command]
pub fn listar_espacios(state: State<'_, AppState>) -> Result<Vec<Espacio>, String> {
    queries::list_espacios(&state.db).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrearEspacioParams {
    pub nombre: String,
    pub tipo: String,
    pub color: String,
}

#[tauri::command]
pub fn crear_espacio(state: State<'_, AppState>, params: CrearEspacioParams) -> Result<Espacio, String> {
    crate::limites::asegurar_max(params.nombre.trim(), crate::limites::MAX_NOMBRE, "El nombre")?;
    queries::create_espacio(&state.db, &params.nombre, &params.tipo, &params.color).map_err(map_err)
}

#[tauri::command]
pub fn eliminar_espacio(state: State<'_, AppState>, espacio_id: String) -> Result<(), String> {
    queries::delete_espacio(&state.db, &espacio_id).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenombrarEspacioParams {
    pub espacio_id: String,
    pub nombre: String,
}

#[tauri::command]
pub fn renombrar_espacio(state: State<'_, AppState>, params: RenombrarEspacioParams) -> Result<(), String> {
    crate::limites::asegurar_max(params.nombre.trim(), crate::limites::MAX_NOMBRE, "El nombre")?;
    queries::rename_espacio(&state.db, &params.espacio_id, &params.nombre).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FijarEspacioParams {
    pub espacio_id: String,
    pub fijado: bool,
}

#[tauri::command]
pub fn fijar_espacio(state: State<'_, AppState>, params: FijarEspacioParams) -> Result<(), String> {
    queries::set_espacio_fijado(&state.db, &params.espacio_id, params.fijado).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorEspacioParams {
    pub espacio_id: String,
    pub color: String,
}

#[tauri::command]
pub fn cambiar_color_espacio(state: State<'_, AppState>, params: ColorEspacioParams) -> Result<(), String> {
    queries::set_espacio_color(&state.db, &params.espacio_id, &params.color).map_err(map_err)
}
