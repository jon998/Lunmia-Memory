use serde::Deserialize;
use tauri::State;

use crate::db::{models::Proyecto, queries};
use crate::AppState;

fn map_err<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListarProyectosParams {
    pub espacio_id: Option<String>,
}

#[tauri::command]
pub fn listar_proyectos(
    state: State<'_, AppState>,
    params: ListarProyectosParams,
) -> Result<Vec<Proyecto>, String> {
    queries::list_proyectos(&state.db, params.espacio_id.as_deref()).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrearProyectoParams {
    pub espacio_id: String,
    pub nombre: String,
}

#[tauri::command]
pub fn crear_proyecto(
    state: State<'_, AppState>,
    params: CrearProyectoParams,
) -> Result<Proyecto, String> {
    crate::limites::asegurar_max(params.nombre.trim(), crate::limites::MAX_NOMBRE, "El nombre")?;
    queries::create_proyecto(&state.db, &params.espacio_id, &params.nombre).map_err(map_err)
}

#[tauri::command]
pub fn eliminar_proyecto(state: State<'_, AppState>, proyecto_id: String) -> Result<(), String> {
    queries::delete_proyecto(&state.db, &proyecto_id).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenombrarProyectoParams {
    pub proyecto_id: String,
    pub nombre: String,
}

#[tauri::command]
pub fn renombrar_proyecto(state: State<'_, AppState>, params: RenombrarProyectoParams) -> Result<(), String> {
    crate::limites::asegurar_max(params.nombre.trim(), crate::limites::MAX_NOMBRE, "El nombre")?;
    queries::rename_proyecto(&state.db, &params.proyecto_id, &params.nombre).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FijarProyectoParams {
    pub proyecto_id: String,
    pub fijado: bool,
}

#[tauri::command]
pub fn fijar_proyecto(state: State<'_, AppState>, params: FijarProyectoParams) -> Result<(), String> {
    queries::set_proyecto_fijado(&state.db, &params.proyecto_id, params.fijado).map_err(map_err)
}
