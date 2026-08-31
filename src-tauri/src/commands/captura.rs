use std::time::Instant;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};
use tracing::info;

use crate::ai::context;
use crate::db::queries::{self, NewEntrada};
use crate::AppState;

fn map_err<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturarTextoParams {
    pub contenido: String,
    pub espacio_id: Option<String>,
    pub proyecto_id: Option<String>,
    pub tipo_nombre: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturaConfirmada {
    pub entry_id: String,
    pub latencia_ms: i64,
}

/// Guarda primero, clasifica después (§6.3). El usuario nunca espera al modelo.
#[tauri::command]
pub async fn capturar_texto(
    app: AppHandle,
    params: CapturarTextoParams,
) -> Result<CapturaConfirmada, String> {
    let inicio = Instant::now();
    let state = app.state::<AppState>();
    crate::limites::asegurar_max(&params.contenido, crate::limites::MAX_CAPTURA, "El texto")?;

    let ctx = if params.espacio_id.is_some() {
        None
    } else {
        context::contexto_para_captura(&state.db)
            .map_err(map_err)?
            .or_else(|| context::primer_contexto_por_defecto(&state.db).ok().flatten())
    };
    let espacio_id = params
        .espacio_id
        .clone()
        .or_else(|| ctx.as_ref().map(|c| c.espacio_id.clone()))
        .ok_or_else(|| "no hay espacios creados todavía".to_string())?;
    let proyecto_id = params
        .proyecto_id
        .clone()
        .or_else(|| ctx.as_ref().map(|c| c.proyecto_id.clone()));

    let metadata_json = params
        .metadata
        .as_ref()
        .and_then(|v| serde_json::to_string(v).ok());

    let tipo_elegido = params
        .tipo_nombre
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase());
    if let Some(nombre) = tipo_elegido.as_deref() {
        crate::limites::asegurar_max(nombre, crate::limites::MAX_NOMBRE, "El tipo")?;
    }
    let tipo_id = match tipo_elegido.as_deref() {
        Some(nombre) => Some(queries::upsert_tipo(&state.db, nombre, None).map_err(map_err)?),
        None => None,
    };
    let tipo_usuario = tipo_id.is_some();

    let entry_id = queries::create_entrada(
        &state.db,
        NewEntrada {
            contenido: &params.contenido,
            contenido_original: None,
            espacio_id: &espacio_id,
            proyecto_id: proyecto_id.as_deref(),
            origen: "texto",
            metadata_captura: metadata_json.as_deref(),
            tipo_id: tipo_id.as_deref(),
            es_provisional: false,
            estado: if tipo_usuario { "activo" } else { "inbox" },
            confianza: if tipo_usuario { Some(1.0) } else { None },
            confianza_capa: if tipo_usuario { Some(0) } else { None },
        },
    )
    .map_err(map_err)?;

    let latencia_ms = inicio.elapsed().as_millis() as i64;
    let _ = queries::registrar_metrica_operacion(&state.db, "capturar", latencia_ms, None);

    // Clasificación en background — no bloquea la respuesta al usuario.
    let pipeline = state.pipeline.clone();
    let id_bg = entry_id.clone();
    let app_bg = app.clone();
    tokio::spawn(async move {
        let resultado = pipeline.clasificar(&id_bg).await;
        if let Err(err) = &resultado {
            tracing::warn!(?err, "pipeline en background");
        }
        let _ = app_bg.emit(
            "entrada:clasificada",
            serde_json::json!({
                "entryId": id_bg,
                "ok": resultado.is_ok(),
            }),
        );
    });

    info!(entry_id, latencia_ms, "capturado");
    Ok(CapturaConfirmada { entry_id, latencia_ms })
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturaOcrResultado {
    pub entry_id: String,
    pub texto: String,
    pub source_app: Option<String>,
}

/// Captura de pantalla — el panel de confirmación SÍ espera al usuario (§6.7).
#[tauri::command]
pub async fn capturar_pantalla(app: AppHandle) -> Result<CapturaOcrResultado, String> {
    let state = app.state::<AppState>();
    let ocr = state.ocr.capturar_pantalla().map_err(map_err)?;
    let texto = crate::limites::truncar(ocr.text.trim(), crate::limites::MAX_CAPTURA);
    if texto.is_empty() {
        return Err("El OCR no encontró texto legible (D4)".to_string());
    }
    let ctx = context::contexto_para_captura(&state.db).map_err(map_err)?;
    let espacio_id = ctx
        .as_ref()
        .map(|c| c.espacio_id.clone())
        .ok_or_else(|| "no hay espacio activo".to_string())?;
    let proyecto_id = ctx.as_ref().map(|c| c.proyecto_id.clone());

    let metadata = serde_json::json!({
        "source_app": ocr.source_app,
        "window_title": ocr.window_title,
        "ocr_confianza": ocr.confidence
    });

    let entry_id = queries::create_entrada(
        &state.db,
        NewEntrada {
            contenido: &texto,
            contenido_original: Some(&texto),
            espacio_id: &espacio_id,
            proyecto_id: proyecto_id.as_deref(),
            origen: "captura_pantalla",
            metadata_captura: serde_json::to_string(&metadata).ok().as_deref(),
            tipo_id: None,
            es_provisional: false,
            estado: "inbox",
            confianza: None,
            confianza_capa: None,
        },
    )
    .map_err(map_err)?;

    let pipeline = state.pipeline.clone();
    let id_bg = entry_id.clone();
    let app_bg = app.clone();
    tokio::spawn(async move {
        let resultado = pipeline.clasificar(&id_bg).await;
        let _ = app_bg.emit(
            "entrada:clasificada",
            serde_json::json!({
                "entryId": id_bg,
                "ok": resultado.is_ok(),
            }),
        );
    });

    Ok(CapturaOcrResultado {
        entry_id,
        texto,
        source_app: ocr.source_app,
    })
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstadoOcr {
    pub disponible: bool,
}

#[tauri::command]
pub fn estado_ocr(state: State<'_, AppState>) -> EstadoOcr {
    EstadoOcr {
        disponible: state.ocr.disponible(),
    }
}
