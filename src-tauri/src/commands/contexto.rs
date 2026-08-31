use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::{
    models::{Contexto, Espacio, Proyecto},
    queries,
};
use crate::ai::context;
use crate::AppState;

fn map_err<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

#[tauri::command]
pub fn obtener_contexto_activo(state: State<'_, AppState>) -> Result<Option<Contexto>, String> {
    context::contexto_para_captura(&state.db).map_err(map_err)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinosCaptura {
    pub espacios: Vec<Espacio>,
    pub proyectos: Vec<Proyecto>,
    pub contexto: Option<Contexto>,
}

/// Un solo roundtrip a SQLite: lo que el flotante necesita para no depender del store de la ventana principal.
#[tauri::command]
pub fn destinos_captura(state: State<'_, AppState>) -> Result<DestinosCaptura, String> {
    let db = &state.db;
    Ok(DestinosCaptura {
        espacios: queries::list_espacios(db).map_err(map_err)?,
        proyectos: queries::list_proyectos(db, None).map_err(map_err)?,
        contexto: context::contexto_para_captura(db).map_err(map_err)?,
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FijarContextoParams {
    pub espacio_id: String,
    pub proyecto_id: String,
    pub origen: Option<String>,
}

#[tauri::command]
pub fn fijar_contexto(state: State<'_, AppState>, params: FijarContextoParams) -> Result<(), String> {
    let origen = params.origen.unwrap_or_else(|| "usuario".into());
    queries::set_contexto_activo(&state.db, &params.espacio_id, &params.proyecto_id, &origen)
        .map_err(map_err)?;
    context::aprender_contexto(&state.db, &params.espacio_id, &params.proyecto_id).map_err(map_err)
}
