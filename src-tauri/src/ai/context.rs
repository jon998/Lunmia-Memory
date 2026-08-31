//! Capa 0: contexto activo (§5.5) — determinista, sin IA.

use anyhow::Result;
use chrono::{Datelike, Local, Timelike};

use crate::db::{models::Contexto, queries, Database};

pub fn contexto_para_captura(db: &Database) -> Result<Option<Contexto>> {
    if let Some(ctx) = queries::get_contexto_activo(db)? {
        return Ok(Some(ctx));
    }
    let ahora = Local::now();
    let dia = (ahora.weekday().num_days_from_sunday()) as i64;
    let franja = ahora.hour() as i64;
    queries::proponer_contexto_por_hora(db, dia, franja)
}

/// Red de seguridad: primer espacio + su proyecto por defecto.
/// Se usa cuando no hay contexto activo ni aprendido (p.ej. sin onboarding).
pub fn primer_contexto_por_defecto(db: &Database) -> Result<Option<Contexto>> {
    let espacios = queries::list_espacios(db)?;
    let Some(esp) = espacios.first() else {
        return Ok(None);
    };
    let proyectos = queries::list_proyectos(db, Some(&esp.espacio_id))?;
    let Some(proy) = proyectos.iter().find(|p| p.es_por_defecto).or_else(|| proyectos.first())
    else {
        return Ok(None);
    };
    Ok(Some(Contexto {
        espacio_id: esp.espacio_id.clone(),
        espacio_nombre: esp.nombre.clone(),
        espacio_color: esp.color.clone(),
        proyecto_id: proy.proyecto_id.clone(),
        proyecto_nombre: proy.nombre.clone(),
        origen: "sesion".to_string(),
        etiqueta: format!("{} / {}", esp.nombre, proy.nombre),
    }))
}

pub fn aprender_contexto(db: &Database, espacio_id: &str, proyecto_id: &str) -> Result<()> {
    let ahora = Local::now();
    let dia = (ahora.weekday().num_days_from_sunday()) as i64;
    let franja = ahora.hour() as i64;
    queries::registrar_frecuencia_contexto(db, espacio_id, proyecto_id, dia, franja)
}
