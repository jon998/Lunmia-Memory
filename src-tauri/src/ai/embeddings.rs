//! Capa 2: embeddings y vecino más cercano contra centroides de tipo.

use std::sync::Arc;

use anyhow::Result;

use super::llm::LlmClient;

#[derive(Debug, Clone)]
pub struct NearestType {
    pub tipo_id: String,
    pub tipo_nombre: String,
    pub similitud: f32,
}

pub struct EmbeddingEngine {
    llm: Arc<LlmClient>,
}

impl EmbeddingEngine {
    pub fn new(llm: Arc<LlmClient>) -> Self {
        Self { llm }
    }

    pub async fn embed_text(&self, texto: &str) -> Result<Vec<f32>> {
        self.llm.embed("clasificar_capa2", texto).await
    }
}

pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na.sqrt() * nb.sqrt())
    }
}

pub fn nearest(
    query: &[f32],
    centroides: &[(String, String, Vec<f32>)],
) -> Option<NearestType> {
    let mut best: Option<NearestType> = None;
    for (id, nombre, emb) in centroides {
        let sim = cosine(query, emb);
        if best.as_ref().map(|b| sim > b.similitud).unwrap_or(true) {
            best = Some(NearestType {
                tipo_id: id.clone(),
                tipo_nombre: nombre.clone(),
                similitud: sim,
            });
        }
    }
    best
}
