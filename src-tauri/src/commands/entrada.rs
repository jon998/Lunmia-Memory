use std::sync::Arc;

use serde::Deserialize;
use tauri::{AppHandle, Manager, State};

use crate::ai::Pipeline;
use crate::db::{
    models::{Entrada, Tipo},
    queries,
};
use crate::AppState;

fn map_err<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListarParams {
    pub espacio_id: Option<String>,
    pub proyecto_id: Option<String>,
    pub limit: Option<i64>,
}

#[tauri::command]
pub fn listar_entradas(
    state: State<'_, AppState>,
    params: ListarParams,
) -> Result<Vec<Entrada>, String> {
    let limit = params.limit.unwrap_or(200);
    queries::list_entradas(
        &state.db,
        params.espacio_id.as_deref(),
        params.proyecto_id.as_deref(),
        limit,
    )
    .map_err(map_err)
}

#[tauri::command]
pub fn listar_tipos(state: State<'_, AppState>) -> Result<Vec<Tipo>, String> {
    queries::list_tipos(&state.db).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrearTipoParams {
    pub nombre: String,
}

#[tauri::command]
pub fn crear_tipo(state: State<'_, AppState>, params: CrearTipoParams) -> Result<Tipo, String> {
    let nombre = params.nombre.trim().to_lowercase();
    if nombre.is_empty() {
        return Err("El tipo no puede estar vacío.".into());
    }
    crate::limites::asegurar_max(&nombre, crate::limites::MAX_NOMBRE, "El tipo")?;
    let id = queries::upsert_tipo(&state.db, &nombre, None).map_err(map_err)?;
    queries::get_tipo(&state.db, &id)
        .map_err(map_err)?
        .ok_or_else(|| "no se pudo leer el tipo".into())
}

#[tauri::command]
pub fn obtener_entrada(state: State<'_, AppState>, entry_id: String) -> Result<Option<Entrada>, String> {
    queries::get_entrada(&state.db, &entry_id).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActualizarContenidoParams {
    pub entry_id: String,
    pub contenido: String,
}

#[tauri::command]
pub async fn actualizar_contenido(
    app: AppHandle,
    params: ActualizarContenidoParams,
) -> Result<(), String> {
    let state = app.state::<AppState>();
    crate::limites::asegurar_max(&params.contenido, crate::limites::MAX_ENTRADA, "El contenido")?;
    queries::update_entrada_contenido(&state.db, &params.entry_id, &params.contenido)
        .map_err(map_err)?;
    // Reclasificar en background (§5.3: se propone, no se aplica). Aquí sólo
    // regeneramos y guardamos: el frontend muestra el banner de propuesta.
    let pipeline = state.pipeline.clone();
    let id = params.entry_id.clone();
    tokio::spawn(async move {
        let _ = pipeline.clasificar(&id).await;
    });
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClasificarParams {
    pub entry_id: String,
    pub tipo_id: Option<String>,
    pub espacio_id: String,
    pub proyecto_id: String,
    pub estado: String,
    pub es_provisional: bool,
    pub confianza: Option<f64>,
    pub confianza_capa: Option<i64>,
}

#[tauri::command]
pub fn actualizar_clasificacion(
    state: State<'_, AppState>,
    params: ClasificarParams,
) -> Result<(), String> {
    queries::update_entrada_clasificacion(
        &state.db,
        &params.entry_id,
        params.tipo_id.as_deref(),
        &params.espacio_id,
        &params.proyecto_id,
        &params.estado,
        params.es_provisional,
        params.confianza,
        params.confianza_capa,
    )
    .map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EtiquetasParams {
    pub entry_id: String,
    pub etiquetas: Vec<String>,
}

#[tauri::command]
pub fn actualizar_etiquetas(state: State<'_, AppState>, params: EtiquetasParams) -> Result<(), String> {
    queries::set_etiquetas(&state.db, &params.entry_id, &params.etiquetas).map_err(map_err)
}

#[tauri::command]
pub fn eliminar_entrada(state: State<'_, AppState>, entry_id: String) -> Result<(), String> {
    queries::delete_entrada(&state.db, &entry_id).map_err(map_err)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoverEntradaParams {
    pub entry_id: String,
    pub espacio_id: String,
    pub proyecto_id: String,
}

#[tauri::command]
pub fn mover_entrada(state: State<'_, AppState>, params: MoverEntradaParams) -> Result<(), String> {
    queries::move_entrada(&state.db, &params.entry_id, &params.espacio_id, &params.proyecto_id)
        .map_err(map_err)
}

// Marker para tipo Pipeline en state
#[allow(dead_code)]
fn _pipeline_marker(_: Arc<Pipeline>) {}
