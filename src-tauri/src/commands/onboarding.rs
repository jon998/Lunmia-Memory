use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};
use tracing::warn;

use crate::db::queries;
use crate::AppState;

fn map_err<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstadoOnboarding {
    pub completado: bool,
    pub permiso_notificaciones: bool,
    pub permiso_accesibilidad: bool,
    pub captura_voz: bool,
    pub atajo: String,
}

#[tauri::command]
pub fn estado_onboarding(state: State<'_, AppState>) -> Result<EstadoOnboarding, String> {
    let db = &state.db;
    Ok(EstadoOnboarding {
        completado: queries::get_config(db, "onboarding_completado")
            .map_err(map_err)?
            .as_deref()
            == Some("1"),
        permiso_notificaciones: queries::get_config(db, "permiso_notificaciones")
            .map_err(map_err)?
            .as_deref()
            == Some("1"),
        permiso_accesibilidad: queries::get_config(db, "permiso_accesibilidad")
            .map_err(map_err)?
            .as_deref()
            == Some("1"),
        captura_voz: queries::get_config(db, "captura_voz_habilitada")
            .map_err(map_err)?
            .as_deref()
            == Some("1"),
        atajo: queries::get_config(db, "atajo_captura")
            .map_err(map_err)?
            .unwrap_or_else(|| "CmdOrCtrl+Shift+Space".to_string()),
    })
}

// ────────────────────────────────────────────────────────────────────────────
// Propuesta inicial (paso "conocerte")
// ────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EspacioPropuesto {
    pub nombre: String,
    pub tipo: String,
    pub color: String,
    #[serde(default)]
    pub razon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TipoPropuesto {
    pub nombre: String,
    #[serde(default)]
    pub razon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PropuestaSetup {
    pub espacios: Vec<EspacioPropuesto>,
    pub tipos: Vec<TipoPropuesto>,
    #[serde(default)]
    pub recomendaciones: Vec<String>,
    #[serde(default)]
    pub fuente: String, // "llm" | "fallback"
}

/// Respuesta cruda del LLM. `fuente` y `recomendaciones` las pone el backend.
#[derive(Debug, Deserialize)]
struct PropuestaLlm {
    #[serde(default)]
    espacios: Vec<EspacioPropuesto>,
    #[serde(default)]
    tipos: Vec<TipoPropuesto>,
}

const COLORES_PROPUESTA: &[&str] = &[
    "#4F46C9", "#C08A3E", "#4C8C68", "#9B4E7A", "#3E7CC0", "#C05F5F", "#6B4CA8",
];

fn propuesta_fallback() -> PropuestaSetup {
    PropuestaSetup {
        espacios: vec![
            EspacioPropuesto {
                nombre: "Trabajo".into(),
                tipo: "trabajo".into(),
                color: "#4F46C9".into(),
                razon: "Todo lo que pertenezca a tu trabajo principal.".into(),
            },
            EspacioPropuesto {
                nombre: "Personal".into(),
                tipo: "personal".into(),
                color: "#C08A3E".into(),
                razon: "Notas, recordatorios y cosas de la vida.".into(),
            },
        ],
        tipos: vec![
            TipoPropuesto { nombre: "idea".into(), razon: "Chispazos por atrapar.".into() },
            TipoPropuesto { nombre: "nota".into(), razon: "Fragmentos y contexto.".into() },
            TipoPropuesto { nombre: "recordatorio".into(), razon: "Cosas con hora o fecha.".into() },
            TipoPropuesto { nombre: "bug".into(), razon: "Fallos para no perder de vista.".into() },
            TipoPropuesto { nombre: "script".into(), razon: "Trozos de código o automatizaciones.".into() },
        ],
        recomendaciones: vec![
            "Usa ⌘⇧Space desde cualquier app para capturar sin cambiar de contexto.".into(),
            "Los espacios se ordenan por color en la barra lateral — elige colores que reconozcas al vuelo.".into(),
            "Todo lo que no encaje va a Inbox y luego lo triageamos.".into(),
        ],
        fuente: "fallback".into(),
    }
}

/// Propuesta inicial: espacios + tipos. Lo decide el LLM; si falla, una base genérica.
#[tauri::command]
pub async fn proponer_setup_inicial(
    state: State<'_, AppState>,
    oficio: String,
) -> Result<PropuestaSetup, String> {
    let oficio_l = oficio.trim().to_string();
    crate::limites::asegurar_max(&oficio_l, crate::limites::MAX_OFICIO, "El oficio")?;
    if oficio_l.is_empty() {
        return Ok(propuesta_fallback());
    }
    if !state.llm.ping().await {
        warn!("LLM no disponible para propuesta inicial, uso fallback");
        state.llm.log_skip(
            "onboarding_setup",
            "ping falló; se usa propuesta genérica de onboarding",
        );
        return Ok(propuesta_fallback());
    }
    let plantilla = r##"Eres el asistente de bienvenida de Lunmia Memory, una app de captura de ideas.
Un usuario acaba de contarte a qué se dedica:

"""__OFICIO__"""

Devuelve UN JSON con esta forma exacta (nada fuera del JSON):
{
  "espacios": [
    {"nombre": "...", "tipo": "trabajo|personal|cliente|estudio", "color": "#RRGGBB", "razon": "..."}
  ],
  "tipos": [
    {"nombre": "...", "razon": "..."}
  ]
}

Reglas:
- 2 a 4 espacios, ordenados por lo más útil primero según lo que cuente el usuario.
- Incluye "Trabajo" y/o "Personal" solo si encajan. Si está desempleado, entre estudios o todo es vida personal, no inventes un espacio de trabajo. Si sólo habla de trabajo y no de vida personal, no fuerces Personal.
- 4 a 7 tipos. Incluye siempre idea, nota y recordatorio; parte también de bug y script si el oficio encaja; añade 1-3 específicos.
- Nombres cortos (1-2 palabras), en español: tipos en minúsculas, espacios capitalizados.
- Colores hex sólo de esta paleta: __PALETA__.
"##;
    let prompt = plantilla
        .replace("__OFICIO__", &oficio_l)
        .replace("__PALETA__", &COLORES_PROPUESTA.join(", "));
    match state.llm.generate_json("onboarding_setup", &prompt).await {
        Ok(raw) => match serde_json::from_str::<PropuestaLlm>(&raw) {
            Ok(llm) => Ok(sanitizar(PropuestaSetup {
                espacios: llm.espacios,
                tipos: llm.tipos,
                recomendaciones: vec![],
                fuente: "llm".into(),
            })),
            Err(err) => {
                warn!(?err, raw = %raw, "respuesta LLM no parseable, uso fallback");
                Ok(propuesta_fallback())
            }
        },
        Err(err) => {
            warn!(?err, "LLM falló, uso fallback");
            Ok(propuesta_fallback())
        }
    }
}

fn sanitizar(mut p: PropuestaSetup) -> PropuestaSetup {
    p.espacios.retain(|e| !e.nombre.trim().is_empty());
    for e in p.espacios.iter_mut() {
        if !["trabajo", "personal", "cliente", "estudio"].contains(&e.tipo.as_str()) {
            e.tipo = "personal".into();
        }
        if !e.color.starts_with('#') || e.color.len() != 7 {
            e.color = "#4F46C9".into();
        }
    }
    p.tipos.retain(|t| !t.nombre.trim().is_empty());
    for t in p.tipos.iter_mut() {
        t.nombre = t.nombre.trim().to_lowercase();
    }
    if !p.tipos.iter().any(|t| t.nombre == "recordatorio") {
        p.tipos.insert(
            0,
            TipoPropuesto {
                nombre: "recordatorio".into(),
                razon: "Cosas con hora o fecha.".into(),
            },
        );
    }
    if p.espacios.len() > 4 { p.espacios.truncate(4); }
    if p.tipos.len() > 8 { p.tipos.truncate(8); }
    if p.recomendaciones.len() > 3 { p.recomendaciones.truncate(3); }
    p
}

// ────────────────────────────────────────────────────────────────────────────
// Aplicar setup: crea espacios/tipos elegidos y elimina los desmarcados.
// ────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalizarOnboardingParams {
    pub oficio: String,
    pub atajo: String,
    pub autostart: bool,
    pub captura_voz: bool,
    pub tipos: Vec<String>,
    pub espacios_nuevos: Vec<EspacioPropuesto>,
    pub espacios_a_eliminar: Vec<String>,
    #[serde(default)]
    pub zona_horaria: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalizarOnboardingResultado {
    pub espacios_creados: usize,
    pub tipos_creados: usize,
}

#[tauri::command]
pub fn finalizar_onboarding(
    app: AppHandle,
    state: State<'_, AppState>,
    params: FinalizarOnboardingParams,
) -> Result<FinalizarOnboardingResultado, String> {
    crate::limites::asegurar_max(params.oficio.trim(), crate::limites::MAX_OFICIO, "El oficio")?;
    for esp in &params.espacios_nuevos {
        crate::limites::asegurar_max(esp.nombre.trim(), crate::limites::MAX_NOMBRE, "El nombre del espacio")?;
    }
    for tipo in &params.tipos {
        crate::limites::asegurar_max(tipo.trim(), crate::limites::MAX_NOMBRE, "El tipo")?;
    }
    let db = &state.db;
    queries::set_config(db, "onboarding_completado", "1").map_err(map_err)?;
    queries::set_config(db, "atajo_captura", &params.atajo).map_err(map_err)?;
    queries::set_config(
        db,
        "captura_voz_habilitada",
        if params.captura_voz { "1" } else { "0" },
    )
    .map_err(map_err)?;
    if let Err(err) = crate::commands::sistema::aplicar_autostart(&app, db, params.autostart) {
        warn!(%err, "no se pudo aplicar autostart");
    }
    queries::set_config(db, "oficio_usuario", params.oficio.trim()).map_err(map_err)?;
    let zona = params.zona_horaria.trim();
    if !zona.is_empty() {
        queries::set_config(db, crate::zona::CLAVE, zona).map_err(map_err)?;
    }

    for id in &params.espacios_a_eliminar {
        if let Err(err) = queries::delete_espacio(db, id) {
            warn!(?err, id, "no se pudo eliminar espacio");
        }
    }

    let mut creados = 0usize;
    for esp in &params.espacios_nuevos {
        match queries::create_espacio(db, &esp.nombre, &esp.tipo, &esp.color) {
            Ok(_) => creados += 1,
            Err(err) => warn!(?err, "no se pudo crear espacio propuesto"),
        }
    }

    let mut tipos_creados = 0usize;
    for tipo in &params.tipos {
        if queries::upsert_tipo(db, tipo, None).is_ok() {
            tipos_creados += 1;
        }
    }

    // Contexto activo: primer espacio vivo + su proyecto por defecto.
    if queries::get_contexto_activo(db).map_err(map_err)?.is_none() {
        let espacios = queries::list_espacios(db).map_err(map_err)?;
        if let Some(primer) = espacios.first() {
            let proyectos = queries::list_proyectos(db, Some(&primer.espacio_id)).map_err(map_err)?;
            if let Some(proy) = proyectos.iter().find(|p| p.es_por_defecto).or_else(|| proyectos.first()) {
                queries::set_contexto_activo(db, &primer.espacio_id, &proy.proyecto_id, "usuario")
                    .map_err(map_err)?;
            }
        }
    }

    Ok(FinalizarOnboardingResultado {
        espacios_creados: creados,
        tipos_creados,
    })
}
