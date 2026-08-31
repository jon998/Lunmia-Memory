//! Pipeline de clasificación en cascada (PRD §6.2).
//!
//! Regla dorada (§6.3): la clasificación NUNCA está en el camino crítico.
//! Cuando esta función se llama, la Entrada ya está persistida y confirmada
//! al usuario. Sólo actualiza la Entrada en background.

use std::sync::Arc;

use anyhow::Result;
use serde::Serialize;
use tracing::{info, warn};

use crate::db::{models::Contexto, queries, Database};
use crate::platform::Platform;

use super::classifier::{agenda_a_fecha, LlmClassifier};
use super::embeddings::EmbeddingEngine;
use super::formato;
use super::llm::LlmClient;
use super::rules;

const UMBRAL_ALTA_CONFIANZA: f32 = 0.72;
const UMBRAL_PREGUNTA: f32 = 0.55;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationResult {
    pub tipo_id: Option<String>,
    pub tipo_nombre: Option<String>,
    pub espacio_id: String,
    pub proyecto_id: String,
    pub etiquetas: Vec<String>,
    pub fecha_detectada: Option<String>,
    pub confianza: f32,
    pub capa: i64,
    pub es_provisional: bool,
    pub preguntas: Vec<PropuestaPregunta>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropuestaPregunta {
    pub texto: String,
    pub opciones: Vec<String>,
}

pub struct Pipeline {
    db: Arc<Database>,
    llm: Arc<LlmClient>,
    embeddings: EmbeddingEngine,
    classifier: LlmClassifier,
    _platform: Arc<Platform>,
}

impl Pipeline {
    pub fn new(db: Arc<Database>, llm: Arc<LlmClient>, platform: Arc<Platform>) -> Self {
        let embeddings = EmbeddingEngine::new(llm.clone());
        let classifier = LlmClassifier::new(llm.clone());
        Self {
            db,
            llm,
            embeddings,
            classifier,
            _platform: platform,
        }
    }

    /// Clasifica una entrada ya persistida y actualiza la BD.
    pub async fn clasificar(&self, entry_id: &str) -> Result<ClassificationResult> {
        let entrada = queries::get_entrada(&self.db, entry_id)?
            .ok_or_else(|| anyhow::anyhow!("entrada no encontrada"))?;

        // Contexto por defecto: el espacio/proyecto ya asignado en la captura.
        let ctx = Contexto {
            espacio_id: entrada.espacio_id.clone(),
            espacio_nombre: entrada.espacio_nombre.clone(),
            espacio_color: entrada.espacio_color.clone(),
            proyecto_id: entrada.proyecto_id.clone(),
            proyecto_nombre: entrada.proyecto_nombre.clone(),
            origen: "sesion".into(),
            etiqueta: format!("{} / {}", entrada.espacio_nombre, entrada.proyecto_nombre),
        };

        let tz = crate::zona::de_db(&self.db);
        let tipo_usuario = entrada.tipo_id.is_some();

        // Agenda: solo el LLM. Si no hay modelo o no trae fechas, el usuario las elige.
        if self.llm.ping().await {
            let tipos_conocidos = queries::list_tipos(&self.db)
                .unwrap_or_default()
                .into_iter()
                .map(|t| t.nombre)
                .collect::<Vec<_>>();
            match self
                .classifier
                .clasificar(&entrada.contenido, &tipos_conocidos, tz)
                .await
            {
                Ok(pred) => {
                    let (tipo_id, tipo_nombre) = if tipo_usuario {
                        (
                            entrada.tipo_id.clone().unwrap(),
                            entrada.tipo_nombre.clone().unwrap_or_else(|| pred.tipo.clone()),
                        )
                    } else {
                        match resolver_tipo_existente(&self.db, &pred.tipo) {
                            Some(par) => par,
                            None => {
                                warn!(tipo = %pred.tipo, "LLM propuso un tipo desconocido; se ignora");
                                return self
                                    .caida_a_provisional(&entrada.entry_id, &ctx, None)
                                    .await;
                            }
                        }
                    };
                    aplicar_contenido_llm(
                        &self.db,
                        &entrada.entry_id,
                        &entrada.contenido,
                        pred.contenido.as_deref(),
                    );
                    let tags: Vec<String> = pred
                        .etiquetas
                        .iter()
                        .map(|e| e.trim().to_lowercase())
                        .filter(|s| !s.is_empty())
                        .take(3)
                        .collect();
                    if !tags.is_empty() {
                        let _ = queries::set_etiquetas(&self.db, &entrada.entry_id, &tags);
                    }
                    let fecha = pred.agenda.as_ref().and_then(|a| agenda_a_fecha(a, tz));
                    let preguntas =
                        self.generar_preguntas_si_ambiguo(pred.confianza, fecha.is_some());
                    let baja = pred.confianza < UMBRAL_PREGUNTA;
                    return self
                        .persistir(
                            &entrada.entry_id,
                            Some(&tipo_id),
                            Some(&tipo_nombre),
                            &ctx,
                            if tipo_usuario { 1.0 } else { pred.confianza },
                            3,
                            baja && !tipo_usuario,
                            &preguntas,
                            fecha.as_ref(),
                        )
                        .await;
                }
                Err(err) => {
                    warn!(?err, "capa 3 falló; se usa respaldo por reglas");
                    self.llm.log_skip(
                        "clasificar_capa3",
                        &format!("capa 3 falló; se guarda con reglas: {err}"),
                    );
                }
            }
        } else {
            warn!("LLM no disponible, capa 3 saltada");
            self.llm.log_skip(
                "clasificar_capa3",
                "ping falló; la nota se guarda tal cual (sin reescritura ni agenda del modelo)",
            );
        }

        if tipo_usuario {
            return self
                .persistir(
                    &entrada.entry_id,
                    entrada.tipo_id.as_deref(),
                    entrada.tipo_nombre.as_deref(),
                    &ctx,
                    1.0,
                    0,
                    false,
                    &[],
                    None,
                )
                .await;
        }

        let capa1 = rules::analizar_en(&entrada.contenido, tz);
        if capa1.confianza >= UMBRAL_ALTA_CONFIANZA {
            if let Some(tipo) = capa1.tipo_sugerido.as_ref() {
                if let Some((tipo_id, tipo_nombre)) = resolver_tipo_existente(&self.db, tipo) {
                    return self
                        .persistir(
                            &entrada.entry_id,
                            Some(&tipo_id),
                            Some(&tipo_nombre),
                            &ctx,
                            capa1.confianza,
                            1,
                            false,
                            &[],
                            None,
                        )
                        .await;
                }
            }
        }

        let capa2 = self.clasificar_por_embeddings(&entrada.contenido).await;
        if let Ok(Some(pred)) = capa2 {
            if pred.similitud >= UMBRAL_ALTA_CONFIANZA {
                return self
                    .persistir(
                        &entrada.entry_id,
                        Some(&pred.tipo_id),
                        Some(&pred.tipo_nombre),
                        &ctx,
                        pred.similitud,
                        2,
                        false,
                        &[],
                        None,
                    )
                    .await;
            }
        }

        self.caida_a_provisional(&entrada.entry_id, &ctx, None)
            .await
    }

    async fn clasificar_por_embeddings(
        &self,
        texto: &str,
    ) -> Result<Option<super::embeddings::NearestType>> {
        if !self.llm.ping().await {
            return Ok(None);
        }
        let query = self.embeddings.embed_text(texto).await?;
        let conn = self.db.conn()?;
        let mut stmt = conn.prepare(
            "SELECT tipo_id, nombre, centroide_embedding FROM tipos
             WHERE deleted_at IS NULL AND archivado = 0 AND centroide_embedding IS NOT NULL",
        )?;
        let rows: Vec<(String, String, Vec<f32>)> = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let nombre: String = row.get(1)?;
                let blob: Vec<u8> = row.get(2)?;
                let emb = bytes_to_f32(&blob);
                Ok((id, nombre, emb))
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(super::embeddings::nearest(&query, &rows))
    }

    async fn caida_a_provisional(
        &self,
        entry_id: &str,
        ctx: &Contexto,
        fecha: Option<&rules::FechaDetectada>,
    ) -> Result<ClassificationResult> {
        info!(entry_id, "clasificación cae a provisional");
        self.persistir(entry_id, None, None, ctx, 0.0, 0, true, &[], fecha)
            .await
    }

    async fn persistir(
        &self,
        entry_id: &str,
        tipo_id: Option<&str>,
        tipo_nombre: Option<&str>,
        ctx: &Contexto,
        confianza: f32,
        capa: i64,
        provisional: bool,
        preguntas: &[PropuestaPregunta],
        fecha: Option<&rules::FechaDetectada>,
    ) -> Result<ClassificationResult> {
        let estado = if provisional { "pendiente_resolucion" } else { "activo" };
        queries::update_entrada_clasificacion(
            &self.db,
            entry_id,
            tipo_id,
            &ctx.espacio_id,
            &ctx.proyecto_id,
            estado,
            provisional,
            Some(confianza as f64),
            Some(capa),
        )?;
        asegurar_valla_codigo(&self.db, entry_id, tipo_nombre);
        for (i, p) in preguntas.iter().enumerate() {
            let _ = queries::create_pregunta(
                &self.db,
                entry_id,
                i as i64,
                &p.texto,
                &p.opciones,
            );
        }
        let fecha_iso = fecha.map(|f| f.instante.to_rfc3339());
        if let Some(f) = fecha {
            let ya_hay_evento = queries::get_evento_por_entrada(&self.db, entry_id)
                .ok()
                .flatten()
                .is_some();
            if !ya_hay_evento {
                let dispara = f.dispara_at(crate::zona::de_db(&self.db)).to_rfc3339();
                let _ = queries::create_recordatorio(
                    &self.db,
                    Some(entry_id),
                    "Recordatorio de Lunmia",
                    None,
                    &dispara,
                );
                if let Ok(Some(entrada)) = queries::get_entrada(&self.db, entry_id) {
                    let titulo = titulo_evento(&entrada.contenido);
                    let inicio = f.instante.to_rfc3339();
                    let fin = f.fin_exclusivo().to_rfc3339();
                    let _ = queries::create_evento(
                        &self.db,
                        queries::NewEvento {
                            entry_id: Some(entry_id),
                            espacio_id: &ctx.espacio_id,
                            proyecto_id: Some(&ctx.proyecto_id),
                            titulo: &titulo,
                            descripcion: Some(&entrada.contenido),
                            inicio_at: &inicio,
                            fin_at: &fin,
                            all_day: f.all_day,
                            ubicacion: None,
                            rrule: None,
                            color: None,
                        },
                    );
                }
            }
        }
        Ok(ClassificationResult {
            tipo_id: tipo_id.map(|s| s.to_string()),
            tipo_nombre: tipo_nombre.map(|s| s.to_string()),
            espacio_id: ctx.espacio_id.clone(),
            proyecto_id: ctx.proyecto_id.clone(),
            etiquetas: vec![],
            fecha_detectada: fecha_iso,
            confianza,
            capa,
            es_provisional: provisional,
            preguntas: preguntas.to_vec(),
        })
    }

    fn generar_preguntas_si_ambiguo(&self, confianza: f32, hay_fecha: bool) -> Vec<PropuestaPregunta> {
        let mut out = vec![];
        if confianza < UMBRAL_PREGUNTA {
            out.push(PropuestaPregunta {
                texto: "Dos tipos empatados — ¿cuál es?".into(),
                opciones: vec!["idea".into(), "tarea".into()],
            });
        }
        if hay_fecha && confianza < 0.85 {
            out.push(PropuestaPregunta {
                texto: "¿Qué día exactamente?".into(),
                opciones: vec!["Este".into(), "El próximo".into()],
            });
        }
        out.truncate(2);
        out
    }
}

/// Solo tipos que ya existen. Nunca crea uno nuevo.
fn resolver_tipo_existente(db: &Database, propuesto: &str) -> Option<(String, String)> {
    let p = propuesto.trim().to_lowercase();
    if p.is_empty() {
        return None;
    }
    let conocidos: Vec<String> = queries::list_tipos(db)
        .ok()?
        .into_iter()
        .map(|t| t.nombre)
        .collect();
    if conocidos.is_empty() {
        return None;
    }
    let nombre = if conocidos.iter().any(|c| c == &p) {
        p
    } else if conocidos.iter().any(|c| c == "nota") {
        "nota".into()
    } else {
        conocidos[0].clone()
    };
    let t = queries::get_tipo_por_nombre(db, &nombre).ok().flatten()?;
    Some((t.tipo_id, t.nombre))
}

fn aplicar_contenido_llm(db: &Database, entry_id: &str, original: &str, propuesto: Option<&str>) {
    let Some(nuevo) = propuesto.map(str::trim).filter(|s| !s.is_empty()) else {
        return;
    };
    if nuevo == original.trim() {
        return;
    }
    let recorte = crate::limites::truncar(nuevo, crate::limites::MAX_ENTRADA);
    let _ = queries::update_entrada_contenido(db, entry_id, &recorte);
}

fn asegurar_valla_codigo(db: &Database, entry_id: &str, tipo_nombre: Option<&str>) {
    let Ok(Some(entrada)) = queries::get_entrada(db, entry_id) else {
        return;
    };
    let Some(fmt) = formato::envolver_si_codigo(&entrada.contenido, tipo_nombre) else {
        return;
    };
    let recorte = crate::limites::truncar(&fmt, crate::limites::MAX_ENTRADA);
    let _ = queries::update_entrada_contenido(db, entry_id, &recorte);
}

fn bytes_to_f32(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

fn titulo_evento(contenido: &str) -> String {
    let linea = contenido
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with("```"))
        .unwrap_or("Evento");
    if linea.chars().count() > 72 {
        format!("{}…", linea.chars().take(72).collect::<String>())
    } else {
        linea.to_string()
    }
}
