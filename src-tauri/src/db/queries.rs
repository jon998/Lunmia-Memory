use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

use super::models::*;
use super::Database;

pub fn now_iso() -> String {
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

pub fn new_uuid() -> String {
    Uuid::now_v7().to_string()
}

// ─────────────────────────── Espacios ─────────────────────────────
pub fn list_espacios(db: &Database) -> Result<Vec<Espacio>> {
    let conn = db.conn()?;
    let mut stmt = conn.prepare(
        "SELECT espacio_id, nombre, tipo, color, fijado, created_at, updated_at
         FROM espacios WHERE deleted_at IS NULL
         ORDER BY tipo ASC, nombre ASC",
    )?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Espacio {
                espacio_id: row.get(0)?,
                nombre: row.get(1)?,
                tipo: row.get(2)?,
                color: row.get(3)?,
                fijado: row.get::<_, i64>(4)? != 0,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

pub fn create_espacio(
    db: &Database,
    nombre: &str,
    tipo: &str,
    color: &str,
) -> Result<Espacio> {
    let conn = db.conn()?;
    let id = new_uuid();
    let now = now_iso();
    conn.execute(
        "INSERT INTO espacios (espacio_id, nombre, tipo, color, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)",
        params![id, nombre, tipo, color, now, now],
    )?;
    let proyecto_id = new_uuid();
    conn.execute(
        "INSERT INTO proyectos (proyecto_id, espacio_id, nombre, es_por_defecto, created_at, updated_at)
         VALUES (?, ?, 'General', 1, ?, ?)",
        params![proyecto_id, id, now, now],
    )?;
    let calendario_id = new_uuid();
    conn.execute(
        "INSERT INTO calendarios (calendario_id, espacio_id, nombre, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![calendario_id, id, format!("{nombre}"), now, now],
    )?;
    Ok(Espacio {
        espacio_id: id,
        nombre: nombre.to_string(),
        tipo: tipo.to_string(),
        color: color.to_string(),
        fijado: false,
        created_at: now.clone(),
        updated_at: now,
    })
}

// ─────────────────────────── Proyectos ─────────────────────────────
pub fn list_proyectos(db: &Database, espacio_id: Option<&str>) -> Result<Vec<Proyecto>> {
    let conn = db.conn()?;
    let (sql, args): (&str, Vec<String>) = match espacio_id {
        Some(eid) => (
            "SELECT proyecto_id, espacio_id, nombre, es_por_defecto, fijado, created_at, updated_at
             FROM proyectos WHERE deleted_at IS NULL AND espacio_id = ?
             ORDER BY es_por_defecto DESC, nombre ASC",
            vec![eid.to_string()],
        ),
        None => (
            "SELECT proyecto_id, espacio_id, nombre, es_por_defecto, fijado, created_at, updated_at
             FROM proyectos WHERE deleted_at IS NULL ORDER BY nombre ASC",
            vec![],
        ),
    };
    let mut stmt = conn.prepare(sql)?;
    let map = |row: &rusqlite::Row| -> rusqlite::Result<Proyecto> {
        Ok(Proyecto {
            proyecto_id: row.get(0)?,
            espacio_id: row.get(1)?,
            nombre: row.get(2)?,
            es_por_defecto: row.get::<_, i64>(3)? != 0,
            fijado: row.get::<_, i64>(4)? != 0,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    };
    let rows: Vec<Proyecto> = if args.is_empty() {
        stmt.query_map([], map)?.collect::<rusqlite::Result<_>>()?
    } else {
        stmt.query_map(rusqlite::params_from_iter(args), map)?
            .collect::<rusqlite::Result<_>>()?
    };
    Ok(rows)
}

pub fn create_proyecto(db: &Database, espacio_id: &str, nombre: &str) -> Result<Proyecto> {
    let conn = db.conn()?;
    let id = new_uuid();
    let now = now_iso();
    conn.execute(
        "INSERT INTO proyectos (proyecto_id, espacio_id, nombre, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![id, espacio_id, nombre, now, now],
    )?;
    Ok(Proyecto {
        proyecto_id: id,
        espacio_id: espacio_id.to_string(),
        nombre: nombre.to_string(),
        es_por_defecto: false,
        fijado: false,
        created_at: now.clone(),
        updated_at: now,
    })
}

/// Soft-delete de un espacio y de sus proyectos. Se cortan las FK con
/// deleted_at para no romper referencias históricas.
pub fn delete_espacio(db: &Database, espacio_id: &str) -> Result<()> {
    let conn = db.conn()?;
    let now = now_iso();
    conn.execute(
        "UPDATE espacios SET deleted_at = ?, updated_at = ? WHERE espacio_id = ?",
        params![now, now, espacio_id],
    )?;
    conn.execute(
        "UPDATE proyectos SET deleted_at = ?, updated_at = ? WHERE espacio_id = ?",
        params![now, now, espacio_id],
    )?;
    Ok(())
}

pub fn delete_proyecto(db: &Database, proyecto_id: &str) -> Result<()> {
    let conn = db.conn()?;
    let now = now_iso();
    conn.execute(
        "UPDATE proyectos SET deleted_at = ?, updated_at = ?
         WHERE proyecto_id = ? AND es_por_defecto = 0",
        params![now, now, proyecto_id],
    )?;
    Ok(())
}

pub fn rename_espacio(db: &Database, espacio_id: &str, nombre: &str) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE espacios SET nombre = ?, updated_at = ? WHERE espacio_id = ?",
        params![nombre, now_iso(), espacio_id],
    )?;
    Ok(())
}

pub fn rename_proyecto(db: &Database, proyecto_id: &str, nombre: &str) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE proyectos SET nombre = ?, updated_at = ? WHERE proyecto_id = ?",
        params![nombre, now_iso(), proyecto_id],
    )?;
    Ok(())
}

pub fn set_espacio_fijado(db: &Database, espacio_id: &str, fijado: bool) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE espacios SET fijado = ?, updated_at = ? WHERE espacio_id = ?",
        params![if fijado { 1 } else { 0 }, now_iso(), espacio_id],
    )?;
    Ok(())
}

pub fn set_proyecto_fijado(db: &Database, proyecto_id: &str, fijado: bool) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE proyectos SET fijado = ?, updated_at = ? WHERE proyecto_id = ?",
        params![if fijado { 1 } else { 0 }, now_iso(), proyecto_id],
    )?;
    Ok(())
}

pub fn set_espacio_color(db: &Database, espacio_id: &str, color: &str) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE espacios SET color = ?, updated_at = ? WHERE espacio_id = ?",
        params![color, now_iso(), espacio_id],
    )?;
    Ok(())
}

fn get_default_proyecto_for_espacio(db: &Database, espacio_id: &str) -> Result<String> {
    let conn = db.conn()?;
    let id: String = conn
        .query_row(
            "SELECT proyecto_id FROM proyectos
             WHERE espacio_id = ? AND es_por_defecto = 1 AND deleted_at IS NULL",
            [espacio_id],
            |row| row.get(0),
        )
        .context("proyecto por defecto del espacio")?;
    Ok(id)
}

// ─────────────────────────── Entradas ─────────────────────────────
pub struct NewEntrada<'a> {
    pub contenido: &'a str,
    pub contenido_original: Option<&'a str>,
    pub espacio_id: &'a str,
    pub proyecto_id: Option<&'a str>,
    pub origen: &'a str,
    pub metadata_captura: Option<&'a str>,
    pub tipo_id: Option<&'a str>,
    pub es_provisional: bool,
    pub estado: &'a str,
    pub confianza: Option<f64>,
    pub confianza_capa: Option<i64>,
}

pub fn create_entrada(db: &Database, entrada: NewEntrada) -> Result<String> {
    let conn = db.conn()?;
    let id = new_uuid();
    let now = now_iso();
    let contenido_original = entrada.contenido_original.unwrap_or(entrada.contenido);
    let proyecto_id = match entrada.proyecto_id {
        Some(pid) => pid.to_string(),
        None => get_default_proyecto_for_espacio(db, entrada.espacio_id)?,
    };
    conn.execute(
        "INSERT INTO entradas (
            entry_id, contenido_original, contenido, tipo_id, espacio_id, proyecto_id,
            es_provisional, estado, origen, metadata_captura, confianza, confianza_capa,
            created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            id,
            contenido_original,
            entrada.contenido,
            entrada.tipo_id,
            entrada.espacio_id,
            proyecto_id,
            if entrada.es_provisional { 1 } else { 0 },
            entrada.estado,
            entrada.origen,
            entrada.metadata_captura,
            entrada.confianza,
            entrada.confianza_capa,
            now,
            now
        ],
    )?;
    Ok(id)
}

pub fn list_entradas(
    db: &Database,
    espacio_id: Option<&str>,
    proyecto_id: Option<&str>,
    limit: i64,
) -> Result<Vec<Entrada>> {
    let conn = db.conn()?;
    let mut sql = String::from(
        "SELECT e.entry_id, e.contenido_original, e.contenido, e.tipo_id, t.nombre,
                e.espacio_id, es.nombre, es.color, e.proyecto_id, p.nombre,
                e.es_provisional, e.estado, e.origen, e.metadata_captura,
                e.confianza, e.confianza_capa, e.created_at, e.updated_at
         FROM entradas e
         JOIN espacios es ON es.espacio_id = e.espacio_id
         JOIN proyectos p ON p.proyecto_id = e.proyecto_id
         LEFT JOIN tipos t ON t.tipo_id = e.tipo_id
         WHERE e.deleted_at IS NULL",
    );
    let mut args: Vec<String> = vec![];
    if let Some(eid) = espacio_id {
        sql.push_str(" AND e.espacio_id = ?");
        args.push(eid.to_string());
    }
    if let Some(pid) = proyecto_id {
        sql.push_str(" AND e.proyecto_id = ?");
        args.push(pid.to_string());
    }
    sql.push_str(" ORDER BY e.created_at DESC LIMIT ?");
    args.push(limit.to_string());

    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(rusqlite::params_from_iter(args))?;
    let mut out = Vec::new();
    while let Some(row) = rows.next()? {
        let entry_id: String = row.get(0)?;
        let etiquetas = list_etiquetas_of(&conn, &entry_id).unwrap_or_default();
        out.push(Entrada {
            entry_id,
            contenido_original: row.get(1)?,
            contenido: row.get(2)?,
            tipo_id: row.get(3)?,
            tipo_nombre: row.get(4)?,
            espacio_id: row.get(5)?,
            espacio_nombre: row.get(6)?,
            espacio_color: row.get(7)?,
            proyecto_id: row.get(8)?,
            proyecto_nombre: row.get(9)?,
            es_provisional: row.get::<_, i64>(10)? != 0,
            estado: row.get(11)?,
            origen: row.get(12)?,
            metadata_captura: row.get(13)?,
            confianza: row.get(14)?,
            confianza_capa: row.get(15)?,
            etiquetas,
            created_at: row.get(16)?,
            updated_at: row.get(17)?,
        });
    }
    Ok(out)
}

pub fn get_entrada(db: &Database, entry_id: &str) -> Result<Option<Entrada>> {
    let items = list_entradas_by_ids(db, &[entry_id.to_string()])?;
    Ok(items.into_iter().next())
}

fn list_entradas_by_ids(db: &Database, ids: &[String]) -> Result<Vec<Entrada>> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let conn = db.conn()?;
    let placeholders = std::iter::repeat("?").take(ids.len()).collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT e.entry_id, e.contenido_original, e.contenido, e.tipo_id, t.nombre,
                e.espacio_id, es.nombre, es.color, e.proyecto_id, p.nombre,
                e.es_provisional, e.estado, e.origen, e.metadata_captura,
                e.confianza, e.confianza_capa, e.created_at, e.updated_at
         FROM entradas e
         JOIN espacios es ON es.espacio_id = e.espacio_id
         JOIN proyectos p ON p.proyecto_id = e.proyecto_id
         LEFT JOIN tipos t ON t.tipo_id = e.tipo_id
         WHERE e.deleted_at IS NULL AND e.entry_id IN ({placeholders})"
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(rusqlite::params_from_iter(ids))?;
    let mut out = Vec::new();
    while let Some(row) = rows.next()? {
        let entry_id: String = row.get(0)?;
        let etiquetas = list_etiquetas_of(&conn, &entry_id).unwrap_or_default();
        out.push(Entrada {
            entry_id,
            contenido_original: row.get(1)?,
            contenido: row.get(2)?,
            tipo_id: row.get(3)?,
            tipo_nombre: row.get(4)?,
            espacio_id: row.get(5)?,
            espacio_nombre: row.get(6)?,
            espacio_color: row.get(7)?,
            proyecto_id: row.get(8)?,
            proyecto_nombre: row.get(9)?,
            es_provisional: row.get::<_, i64>(10)? != 0,
            estado: row.get(11)?,
            origen: row.get(12)?,
            metadata_captura: row.get(13)?,
            confianza: row.get(14)?,
            confianza_capa: row.get(15)?,
            etiquetas,
            created_at: row.get(16)?,
            updated_at: row.get(17)?,
        });
    }
    Ok(out)
}

pub fn update_entrada_contenido(db: &Database, entry_id: &str, contenido: &str) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE entradas SET contenido = ?, updated_at = ? WHERE entry_id = ?",
        params![contenido, now_iso(), entry_id],
    )?;
    Ok(())
}

pub fn update_entrada_clasificacion(
    db: &Database,
    entry_id: &str,
    tipo_id: Option<&str>,
    espacio_id: &str,
    proyecto_id: &str,
    estado: &str,
    es_provisional: bool,
    confianza: Option<f64>,
    confianza_capa: Option<i64>,
) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE entradas SET tipo_id = ?, espacio_id = ?, proyecto_id = ?, estado = ?,
                            es_provisional = ?, confianza = ?, confianza_capa = ?, updated_at = ?
         WHERE entry_id = ?",
        params![
            tipo_id,
            espacio_id,
            proyecto_id,
            estado,
            if es_provisional { 1 } else { 0 },
            confianza,
            confianza_capa,
            now_iso(),
            entry_id
        ],
    )?;
    Ok(())
}

pub fn move_entrada(db: &Database, entry_id: &str, espacio_id: &str, proyecto_id: &str) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE entradas SET espacio_id = ?, proyecto_id = ?, updated_at = ? WHERE entry_id = ?",
        params![espacio_id, proyecto_id, now_iso(), entry_id],
    )?;
    Ok(())
}

pub fn delete_entrada(db: &Database, entry_id: &str) -> Result<()> {
    let conn = db.conn()?;
    let now = now_iso();
    conn.execute(
        "UPDATE entradas SET deleted_at = ?, updated_at = ? WHERE entry_id = ?",
        params![now, now, entry_id],
    )?;
    conn.execute(
        "UPDATE eventos SET deleted_at = ?, updated_at = ? WHERE entry_id = ? AND deleted_at IS NULL",
        params![now, now, entry_id],
    )?;
    conn.execute(
        "UPDATE recordatorios SET deleted_at = ?, cancelado_at = ?, updated_at = ? WHERE entry_id = ? AND deleted_at IS NULL",
        params![now, now, now, entry_id],
    )?;
    Ok(())
}

// ─────────────────────────── Etiquetas ─────────────────────────────
fn list_etiquetas_of(conn: &rusqlite::Connection, entry_id: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT et.nombre FROM etiquetas et
         JOIN entrada_etiquetas ee ON ee.etiqueta_id = et.etiqueta_id
         WHERE ee.entry_id = ? AND et.deleted_at IS NULL
         ORDER BY et.nombre",
    )?;
    let rows = stmt
        .query_map([entry_id], |row| row.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

pub fn set_etiquetas(db: &Database, entry_id: &str, etiquetas: &[String]) -> Result<()> {
    let mut conn = db.conn()?;
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM entrada_etiquetas WHERE entry_id = ?", [entry_id])?;
    for nombre in etiquetas {
        let nombre = nombre.trim().to_lowercase();
        if nombre.is_empty() {
            continue;
        }
        let etiqueta_id: String = match tx
            .query_row(
                "SELECT etiqueta_id FROM etiquetas WHERE nombre = ? AND deleted_at IS NULL",
                [&nombre],
                |row| row.get(0),
            )
            .optional()?
        {
            Some(id) => id,
            None => {
                let id = new_uuid();
                let now = now_iso();
                tx.execute(
                    "INSERT INTO etiquetas (etiqueta_id, nombre, created_at, updated_at)
                     VALUES (?, ?, ?, ?)",
                    params![id, nombre, now, now],
                )?;
                id
            }
        };
        let now = now_iso();
        tx.execute(
            "INSERT OR IGNORE INTO entrada_etiquetas (entry_id, etiqueta_id, created_at)
             VALUES (?, ?, ?)",
            params![entry_id, etiqueta_id, now],
        )?;
    }
    tx.commit()?;
    Ok(())
}

// ─────────────────────────── Tipos ─────────────────────────────
pub fn list_tipos(db: &Database) -> Result<Vec<Tipo>> {
    let conn = db.conn()?;
    let mut stmt = conn.prepare(
        "SELECT tipo_id, nombre, descripcion, usos_total, archivado, ultima_uso_at,
                created_at, updated_at
         FROM tipos WHERE deleted_at IS NULL AND archivado = 0
         ORDER BY usos_total DESC, nombre ASC",
    )?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Tipo {
                tipo_id: row.get(0)?,
                nombre: row.get(1)?,
                descripcion: row.get(2)?,
                usos_total: row.get(3)?,
                archivado: row.get::<_, i64>(4)? != 0,
                ultima_uso_at: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

pub const TIPOS_POR_DEFECTO: &[&str] = &["bug", "script", "idea", "nota", "recordatorio"];

pub fn asegurar_tipos_por_defecto(db: &Database) -> Result<()> {
    for nombre in TIPOS_POR_DEFECTO {
        upsert_tipo(db, nombre, None)?;
    }
    Ok(())
}

pub fn upsert_tipo(db: &Database, nombre: &str, descripcion: Option<&str>) -> Result<String> {
    let conn = db.conn()?;
    if let Some(id) = conn
        .query_row(
            "SELECT tipo_id FROM tipos WHERE nombre = ? AND deleted_at IS NULL",
            [nombre],
            |row| row.get::<_, String>(0),
        )
        .optional()?
    {
        return Ok(id);
    }
    let id = new_uuid();
    let now = now_iso();
    conn.execute(
        "INSERT INTO tipos (tipo_id, nombre, descripcion, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![id, nombre, descripcion, now, now],
    )?;
    Ok(id)
}

pub fn get_tipo(db: &Database, tipo_id: &str) -> Result<Option<Tipo>> {
    let conn = db.conn()?;
    conn.query_row(
        "SELECT tipo_id, nombre, descripcion, usos_total, archivado, ultima_uso_at,
                created_at, updated_at
         FROM tipos WHERE tipo_id = ? AND deleted_at IS NULL",
        [tipo_id],
        |row| {
            Ok(Tipo {
                tipo_id: row.get(0)?,
                nombre: row.get(1)?,
                descripcion: row.get(2)?,
                usos_total: row.get(3)?,
                archivado: row.get::<_, i64>(4)? != 0,
                ultima_uso_at: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    )
    .optional()
    .map_err(Into::into)
}

pub fn get_tipo_por_nombre(db: &Database, nombre: &str) -> Result<Option<Tipo>> {
    let conn = db.conn()?;
    conn.query_row(
        "SELECT tipo_id, nombre, descripcion, usos_total, archivado, ultima_uso_at,
                created_at, updated_at
         FROM tipos WHERE nombre = ? AND deleted_at IS NULL AND archivado = 0",
        [nombre],
        |row| {
            Ok(Tipo {
                tipo_id: row.get(0)?,
                nombre: row.get(1)?,
                descripcion: row.get(2)?,
                usos_total: row.get(3)?,
                archivado: row.get::<_, i64>(4)? != 0,
                ultima_uso_at: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    )
    .optional()
    .map_err(Into::into)
}

// ─────────────────────────── Preguntas ─────────────────────────────
pub fn create_pregunta(
    db: &Database,
    entry_id: &str,
    orden: i64,
    texto: &str,
    opciones: &[String],
) -> Result<String> {
    let conn = db.conn()?;
    let id = new_uuid();
    let now = now_iso();
    let opciones_json = serde_json::to_string(opciones).unwrap_or_else(|_| "[]".into());
    conn.execute(
        "INSERT INTO preguntas_pendientes (pregunta_id, entry_id, orden, texto, opciones, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![id, entry_id, orden, texto, opciones_json, now, now],
    )?;
    Ok(id)
}

pub fn list_preguntas_pendientes(db: &Database) -> Result<Vec<(Entrada, Vec<PreguntaPendiente>)>> {
    let conn = db.conn()?;
    let mut stmt = conn.prepare(
        "SELECT DISTINCT e.entry_id FROM entradas e
         JOIN preguntas_pendientes p ON p.entry_id = e.entry_id
         WHERE e.deleted_at IS NULL AND p.resuelta_at IS NULL AND p.descartada_at IS NULL
         ORDER BY e.created_at DESC",
    )?;
    let ids: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(stmt);
    let entradas = list_entradas_by_ids(db, &ids)?;
    let mut result = Vec::with_capacity(entradas.len());
    for entrada in entradas {
        let mut q = conn.prepare(
            "SELECT pregunta_id, entry_id, orden, texto, opciones, respuesta_opcion, respuesta_texto,
                    resuelta_at, descartada_at, created_at
             FROM preguntas_pendientes WHERE entry_id = ? ORDER BY orden ASC",
        )?;
        let preguntas: Vec<PreguntaPendiente> = q
            .query_map([&entrada.entry_id], |row| {
                let opciones: String = row.get(4)?;
                Ok(PreguntaPendiente {
                    pregunta_id: row.get(0)?,
                    entry_id: row.get(1)?,
                    orden: row.get(2)?,
                    texto: row.get(3)?,
                    opciones: serde_json::from_str(&opciones).unwrap_or_default(),
                    respuesta_opcion: row.get(5)?,
                    respuesta_texto: row.get(6)?,
                    resuelta: row.get::<_, Option<String>>(7)?.is_some(),
                    descartada: row.get::<_, Option<String>>(8)?.is_some(),
                    created_at: row.get(9)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        result.push((entrada, preguntas));
    }
    Ok(result)
}

pub fn responder_pregunta(
    db: &Database,
    pregunta_id: &str,
    opcion: Option<i64>,
    texto: Option<&str>,
) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE preguntas_pendientes
         SET respuesta_opcion = ?, respuesta_texto = ?, resuelta_at = ?, updated_at = ?
         WHERE pregunta_id = ?",
        params![opcion, texto, now_iso(), now_iso(), pregunta_id],
    )?;
    Ok(())
}

pub fn descartar_pregunta(db: &Database, pregunta_id: &str) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE preguntas_pendientes
         SET descartada_at = ?, updated_at = ?
         WHERE pregunta_id = ?",
        params![now_iso(), now_iso(), pregunta_id],
    )?;
    Ok(())
}

// ─────────────────────────── Recordatorios ─────────────────────────────
pub fn create_recordatorio(
    db: &Database,
    entry_id: Option<&str>,
    titulo: &str,
    cuerpo: Option<&str>,
    dispara_at: &str,
) -> Result<String> {
    let conn = db.conn()?;
    let id = new_uuid();
    let now = now_iso();
    conn.execute(
        "INSERT INTO recordatorios (recordatorio_id, entry_id, titulo, cuerpo, dispara_at, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![id, entry_id, titulo, cuerpo, dispara_at, now, now],
    )?;
    Ok(id)
}

pub fn reemplazar_recordatorio_entrada(
    db: &Database,
    entry_id: &str,
    titulo: &str,
    dispara_at: &str,
) -> Result<String> {
    let conn = db.conn()?;
    let now = now_iso();
    conn.execute(
        "UPDATE recordatorios SET deleted_at = ?, cancelado_at = ?, updated_at = ?
         WHERE entry_id = ? AND deleted_at IS NULL",
        params![now, now, now, entry_id],
    )?;
    drop(conn);
    create_recordatorio(db, Some(entry_id), titulo, None, dispara_at)
}

pub fn get_recordatorio_activo_por_entrada(
    db: &Database,
    entry_id: &str,
) -> Result<Option<Recordatorio>> {
    let conn = db.conn()?;
    Ok(conn.query_row(
        "SELECT recordatorio_id, entry_id, titulo, cuerpo, dispara_at, disparado_at, cancelado_at, created_at
         FROM recordatorios
         WHERE entry_id = ? AND deleted_at IS NULL AND cancelado_at IS NULL
         ORDER BY created_at DESC LIMIT 1",
        [entry_id],
        |row| {
            Ok(Recordatorio {
                recordatorio_id: row.get(0)?,
                entry_id: row.get(1)?,
                titulo: row.get(2)?,
                cuerpo: row.get(3)?,
                dispara_at: row.get(4)?,
                disparado: row.get::<_, Option<String>>(5)?.is_some(),
                cancelado: row.get::<_, Option<String>>(6)?.is_some(),
                created_at: row.get(7)?,
            })
        },
    )
    .optional()?)
}

pub fn recordatorios_por_disparar(db: &Database, ahora_iso: &str) -> Result<Vec<Recordatorio>> {
    let conn = db.conn()?;
    let mut stmt = conn.prepare(
        "SELECT recordatorio_id, entry_id, titulo, cuerpo, dispara_at, disparado_at, cancelado_at, created_at
         FROM recordatorios
         WHERE deleted_at IS NULL AND disparado_at IS NULL AND cancelado_at IS NULL
           AND dispara_at <= ?
         ORDER BY dispara_at ASC LIMIT 50",
    )?;
    let rows = stmt
        .query_map([ahora_iso], |row| {
            Ok(Recordatorio {
                recordatorio_id: row.get(0)?,
                entry_id: row.get(1)?,
                titulo: row.get(2)?,
                cuerpo: row.get(3)?,
                dispara_at: row.get(4)?,
                disparado: row.get::<_, Option<String>>(5)?.is_some(),
                cancelado: row.get::<_, Option<String>>(6)?.is_some(),
                created_at: row.get(7)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

pub fn marcar_recordatorio_disparado(db: &Database, recordatorio_id: &str) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE recordatorios SET disparado_at = ?, updated_at = ? WHERE recordatorio_id = ?",
        params![now_iso(), now_iso(), recordatorio_id],
    )?;
    Ok(())
}

// ─────────────────────────── Eventos (calendario) ─────────────────────────────
pub struct NewEvento<'a> {
    pub entry_id: Option<&'a str>,
    pub espacio_id: &'a str,
    pub proyecto_id: Option<&'a str>,
    pub titulo: &'a str,
    pub descripcion: Option<&'a str>,
    pub inicio_at: &'a str,
    pub fin_at: &'a str,
    pub all_day: bool,
    pub ubicacion: Option<&'a str>,
    pub rrule: Option<&'a str>,
    pub color: Option<&'a str>,
}

pub fn create_evento(db: &Database, evento: NewEvento) -> Result<String> {
    let conn = db.conn()?;
    let id = new_uuid();
    let now = now_iso();
    conn.execute(
        "INSERT INTO eventos (
            evento_id, entry_id, espacio_id, proyecto_id, titulo, descripcion,
            inicio_at, fin_at, all_day, ubicacion, rrule, color, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            id,
            evento.entry_id,
            evento.espacio_id,
            evento.proyecto_id,
            evento.titulo,
            evento.descripcion,
            evento.inicio_at,
            evento.fin_at,
            if evento.all_day { 1 } else { 0 },
            evento.ubicacion,
            evento.rrule,
            evento.color,
            now,
            now
        ],
    )?;
    Ok(id)
}

pub fn list_eventos_en_rango(
    db: &Database,
    desde_iso: &str,
    hasta_iso: &str,
    espacio_id: Option<&str>,
) -> Result<Vec<Evento>> {
    let conn = db.conn()?;
    let mut sql = String::from(
        "SELECT ev.evento_id, ev.entry_id, ev.espacio_id, es.nombre, es.color,
                ev.proyecto_id, p.nombre, ev.titulo, ev.descripcion,
                ev.inicio_at, ev.fin_at, ev.all_day, ev.ubicacion, ev.rrule, ev.color,
                ev.created_at, ev.updated_at
         FROM eventos ev
         JOIN espacios es ON es.espacio_id = ev.espacio_id
         LEFT JOIN proyectos p ON p.proyecto_id = ev.proyecto_id
         WHERE ev.deleted_at IS NULL
           AND ev.inicio_at < ? AND ev.fin_at > ?",
    );
    let mut args: Vec<String> = vec![hasta_iso.to_string(), desde_iso.to_string()];
    if let Some(eid) = espacio_id {
        sql.push_str(" AND ev.espacio_id = ?");
        args.push(eid.to_string());
    }
    sql.push_str(" ORDER BY ev.inicio_at ASC");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt
        .query_map(rusqlite::params_from_iter(args), evento_from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

pub fn get_evento(db: &Database, evento_id: &str) -> Result<Option<Evento>> {
    let conn = db.conn()?;
    let ev = conn
        .query_row(
            "SELECT ev.evento_id, ev.entry_id, ev.espacio_id, es.nombre, es.color,
                    ev.proyecto_id, p.nombre, ev.titulo, ev.descripcion,
                    ev.inicio_at, ev.fin_at, ev.all_day, ev.ubicacion, ev.rrule, ev.color,
                    ev.created_at, ev.updated_at
             FROM eventos ev
             JOIN espacios es ON es.espacio_id = ev.espacio_id
             LEFT JOIN proyectos p ON p.proyecto_id = ev.proyecto_id
             WHERE ev.evento_id = ? AND ev.deleted_at IS NULL",
            [evento_id],
            evento_from_row,
        )
        .optional()?;
    Ok(ev)
}

pub fn get_evento_por_entrada(db: &Database, entry_id: &str) -> Result<Option<Evento>> {
    let conn = db.conn()?;
    let ev = conn
        .query_row(
            "SELECT ev.evento_id, ev.entry_id, ev.espacio_id, es.nombre, es.color,
                    ev.proyecto_id, p.nombre, ev.titulo, ev.descripcion,
                    ev.inicio_at, ev.fin_at, ev.all_day, ev.ubicacion, ev.rrule, ev.color,
                    ev.created_at, ev.updated_at
             FROM eventos ev
             JOIN espacios es ON es.espacio_id = ev.espacio_id
             LEFT JOIN proyectos p ON p.proyecto_id = ev.proyecto_id
             WHERE ev.entry_id = ? AND ev.deleted_at IS NULL
             ORDER BY ev.created_at DESC LIMIT 1",
            [entry_id],
            evento_from_row,
        )
        .optional()?;
    Ok(ev)
}

fn evento_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Evento> {
    Ok(Evento {
        evento_id: row.get(0)?,
        entry_id: row.get(1)?,
        espacio_id: row.get(2)?,
        espacio_nombre: row.get(3)?,
        espacio_color: row.get(4)?,
        proyecto_id: row.get(5)?,
        proyecto_nombre: row.get(6)?,
        titulo: row.get(7)?,
        descripcion: row.get(8)?,
        inicio_at: row.get(9)?,
        fin_at: row.get(10)?,
        all_day: row.get::<_, i64>(11)? != 0,
        ubicacion: row.get(12)?,
        rrule: row.get(13)?,
        color: row.get(14)?,
        created_at: row.get(15)?,
        updated_at: row.get(16)?,
    })
}

pub fn update_evento(
    db: &Database,
    evento_id: &str,
    titulo: &str,
    descripcion: Option<&str>,
    inicio_at: &str,
    fin_at: &str,
    all_day: bool,
    espacio_id: &str,
    proyecto_id: Option<&str>,
    ubicacion: Option<&str>,
    rrule: Option<&str>,
    color: Option<&str>,
) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "UPDATE eventos SET titulo = ?, descripcion = ?, inicio_at = ?, fin_at = ?,
                             all_day = ?, espacio_id = ?, proyecto_id = ?, ubicacion = ?,
                             rrule = ?, color = ?, updated_at = ?
         WHERE evento_id = ?",
        params![
            titulo,
            descripcion,
            inicio_at,
            fin_at,
            if all_day { 1 } else { 0 },
            espacio_id,
            proyecto_id,
            ubicacion,
            rrule,
            color,
            now_iso(),
            evento_id,
        ],
    )?;
    Ok(())
}

pub fn delete_evento(db: &Database, evento_id: &str) -> Result<()> {
    let conn = db.conn()?;
    let now = now_iso();
    let entry_id: Option<String> = conn
        .query_row(
            "SELECT entry_id FROM eventos WHERE evento_id = ?",
            [evento_id],
            |row| row.get(0),
        )
        .optional()?
        .flatten();
    conn.execute(
        "UPDATE eventos SET deleted_at = ?, updated_at = ? WHERE evento_id = ?",
        params![now, now, evento_id],
    )?;
    if let Some(eid) = entry_id {
        conn.execute(
            "UPDATE recordatorios SET deleted_at = ?, cancelado_at = ?, updated_at = ? WHERE entry_id = ? AND deleted_at IS NULL",
            params![now, now, now, eid],
        )?;
    }
    Ok(())
}

/// Crea eventos de un día a partir de recordatorios que aún no tienen evento.
/// Así el calendario no queda vacío con capturas anteriores a este modelo.
pub fn backfill_eventos_desde_recordatorios(db: &Database) -> Result<()> {
    let conn = db.conn()?;
    let mut stmt = conn.prepare(
        "SELECT r.entry_id, r.dispara_at, e.contenido, e.espacio_id, e.proyecto_id
         FROM recordatorios r
         JOIN entradas e ON e.entry_id = r.entry_id
         WHERE r.deleted_at IS NULL
           AND r.entry_id IS NOT NULL
           AND e.deleted_at IS NULL
           AND NOT EXISTS (
             SELECT 1 FROM eventos ev
             WHERE ev.entry_id = r.entry_id AND ev.deleted_at IS NULL
           )",
    )?;
    let filas: Vec<(String, String, String, String, String)> = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();
    drop(stmt);
    drop(conn);
    for (entry_id, dispara_at, contenido, espacio_id, proyecto_id) in filas {
        let titulo = contenido
            .lines()
            .next()
            .unwrap_or(&contenido)
            .trim()
            .chars()
            .take(72)
            .collect::<String>();
        let titulo = if titulo.is_empty() {
            "Evento".to_string()
        } else {
            titulo
        };
        let fecha = dispara_at.get(..10).unwrap_or("1970-01-01");
        let inicio = format!("{fecha}T00:00:00.000Z");
        let fin = chrono::NaiveDate::parse_from_str(fecha, "%Y-%m-%d")
            .ok()
            .and_then(|d| d.succ_opt())
            .map(|d| format!("{}T00:00:00.000Z", d.format("%Y-%m-%d")))
            .unwrap_or_else(|| dispara_at.clone());
        let _ = create_evento(
            db,
            NewEvento {
                entry_id: Some(&entry_id),
                espacio_id: &espacio_id,
                proyecto_id: Some(&proyecto_id),
                titulo: &titulo,
                descripcion: Some(&contenido),
                inicio_at: &inicio,
                fin_at: &fin,
                all_day: true,
                ubicacion: None,
                rrule: None,
                color: None,
            },
        );
    }
    Ok(())
}

// ─────────────────────────── Contexto activo ─────────────────────────────
pub fn get_contexto_activo(db: &Database) -> Result<Option<Contexto>> {
    let conn = db.conn()?;
    let ctx = conn
        .query_row(
            "SELECT c.espacio_id, es.nombre, es.color, c.proyecto_id, p.nombre, c.origen
             FROM contexto_sesion c
             JOIN espacios es ON es.espacio_id = c.espacio_id
             JOIN proyectos p ON p.proyecto_id = c.proyecto_id
             WHERE c.activo = 1 ORDER BY c.created_at DESC LIMIT 1",
            [],
            |row| {
                let espacio_nombre: String = row.get(1)?;
                let proyecto_nombre: String = row.get(4)?;
                Ok(Contexto {
                    espacio_id: row.get(0)?,
                    espacio_nombre: espacio_nombre.clone(),
                    espacio_color: row.get(2)?,
                    proyecto_id: row.get(3)?,
                    proyecto_nombre: proyecto_nombre.clone(),
                    origen: row.get(5)?,
                    etiqueta: format!("{} / {}", espacio_nombre, proyecto_nombre),
                })
            },
        )
        .optional()?;
    Ok(ctx)
}

pub fn set_contexto_activo(
    db: &Database,
    espacio_id: &str,
    proyecto_id: &str,
    origen: &str,
) -> Result<()> {
    let mut conn = db.conn()?;
    let tx = conn.transaction()?;
    tx.execute("UPDATE contexto_sesion SET activo = 0", [])?;
    tx.execute(
        "INSERT INTO contexto_sesion (sesion_id, espacio_id, proyecto_id, origen, activo, created_at, updated_at)
         VALUES (?, ?, ?, ?, 1, ?, ?)",
        params![new_uuid(), espacio_id, proyecto_id, origen, now_iso(), now_iso()],
    )?;
    tx.commit()?;
    Ok(())
}

pub fn registrar_frecuencia_contexto(
    db: &Database,
    espacio_id: &str,
    proyecto_id: &str,
    dia_semana: i64,
    franja_hora: i64,
) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "INSERT INTO contexto_frecuencias (frecuencia_id, dia_semana, franja_hora, espacio_id, proyecto_id, cuenta, updated_at)
         VALUES (?, ?, ?, ?, ?, 1, ?)
         ON CONFLICT(dia_semana, franja_hora, espacio_id, proyecto_id)
         DO UPDATE SET cuenta = cuenta + 1, updated_at = excluded.updated_at",
        params![new_uuid(), dia_semana, franja_hora, espacio_id, proyecto_id, now_iso()],
    )?;
    Ok(())
}

pub fn proponer_contexto_por_hora(
    db: &Database,
    dia_semana: i64,
    franja_hora: i64,
) -> Result<Option<Contexto>> {
    let conn = db.conn()?;
    let ctx = conn
        .query_row(
            "SELECT c.espacio_id, es.nombre, es.color, c.proyecto_id, p.nombre
             FROM contexto_frecuencias c
             JOIN espacios es ON es.espacio_id = c.espacio_id
             JOIN proyectos p ON p.proyecto_id = c.proyecto_id
             WHERE c.dia_semana = ? AND c.franja_hora = ?
             ORDER BY c.cuenta DESC LIMIT 1",
            params![dia_semana, franja_hora],
            |row| {
                let espacio_nombre: String = row.get(1)?;
                let proyecto_nombre: String = row.get(4)?;
                Ok(Contexto {
                    espacio_id: row.get(0)?,
                    espacio_nombre: espacio_nombre.clone(),
                    espacio_color: row.get(2)?,
                    proyecto_id: row.get(3)?,
                    proyecto_nombre: proyecto_nombre.clone(),
                    origen: "aprendido".to_string(),
                    etiqueta: format!("{} / {}", espacio_nombre, proyecto_nombre),
                })
            },
        )
        .optional()?;
    Ok(ctx)
}

// ─────────────────────────── Config ─────────────────────────────
pub fn get_config(db: &Database, clave: &str) -> Result<Option<String>> {
    let conn = db.conn()?;
    let value = conn
        .query_row("SELECT valor FROM config WHERE clave = ?", [clave], |row| {
            row.get::<_, String>(0)
        })
        .optional()?;
    Ok(value)
}

pub fn set_config(db: &Database, clave: &str, valor: &str) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "INSERT INTO config (clave, valor, updated_at) VALUES (?, ?, ?)
         ON CONFLICT(clave) DO UPDATE SET valor = excluded.valor, updated_at = excluded.updated_at",
        params![clave, valor, now_iso()],
    )?;
    Ok(())
}

// ─────────────────────────── Métricas ─────────────────────────────
pub fn registrar_metrica_operacion(
    db: &Database,
    operacion: &str,
    latencia_ms: i64,
    contexto: Option<&str>,
) -> Result<()> {
    let conn = db.conn()?;
    conn.execute(
        "INSERT INTO metricas_operacion (metrica_id, operacion, latencia_ms, contexto, created_at)
         VALUES (?, ?, ?, ?, ?)",
        params![new_uuid(), operacion, latencia_ms, contexto, now_iso()],
    )?;
    Ok(())
}

pub fn calcular_stats(db: &Database) -> Result<Stats> {
    let conn = db.conn()?;
    let hoy = chrono::Utc::now().date_naive().format("%Y-%m-%d").to_string();
    let capturas_hoy: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM entradas WHERE deleted_at IS NULL AND substr(created_at, 1, 10) = ?",
            [&hoy],
            |row| row.get(0),
        )
        .unwrap_or(0);
    let sin_triage: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM entradas WHERE deleted_at IS NULL AND estado IN ('inbox', 'pendiente_resolucion')",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);
    let provisionales: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM entradas WHERE deleted_at IS NULL AND es_provisional = 1",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);
    let latencia_media_ms: i64 = conn
        .query_row(
            "SELECT COALESCE(CAST(AVG(latencia_ms) AS INTEGER), 0) FROM metricas_operacion
             WHERE operacion = 'capturar' AND substr(created_at, 1, 10) = ?",
            [&hoy],
            |row| row.get(0),
        )
        .unwrap_or(0);
    Ok(Stats {
        capturas_hoy,
        sin_triage,
        latencia_media_ms,
        perdidas: 0,
        provisionales,
    })
}
