//! Logger de llamadas al LLM.
//!
//! Escribe una línea JSON por llamada en `logs/llm.jsonl`. Cada línea contiene
//! prompt completo, respuesta, latencia y — cuando el proveedor los reporta —
//! los tokens. Rota el archivo cuando pasa 5 MB (renombra a `llm.jsonl.old`).
//!
//! El objetivo es que cualquier persona pueda `tail -f` el archivo mientras usa
//! la app y ver exactamente qué se le envía al modelo y qué responde, para ir
//! afinando prompts sin adivinar.

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use chrono::Utc;
use serde::Serialize;
use tracing::warn;

use super::llm::TokenUsage;

const MAX_BYTES: u64 = 5 * 1024 * 1024;

#[derive(Debug, Clone, Serialize)]
pub struct LlmCallLog {
    pub tag: String,
    pub operation: String,
    pub provider: String,
    pub model: String,
    pub prompt: String,
    pub response: String,
    pub latency_ms: u64,
    pub tokens: Option<TokenUsage>,
    pub ok: bool,
    pub error: Option<String>,
}

pub struct LlmLogger {
    path: PathBuf,
    enabled: bool,
    lock: Mutex<()>,
}

impl LlmLogger {
    pub fn new(path: PathBuf) -> Self {
        let enabled = std::env::var("LUNMIA_LLM_LOG")
            .map(|v| !matches!(v.as_str(), "0" | "false" | "off"))
            .unwrap_or(true);
        if enabled {
            if let Some(parent) = path.parent() {
                if let Err(err) = fs::create_dir_all(parent) {
                    warn!(?err, ?parent, "no se pudo crear carpeta de logs LLM");
                }
            }
        }
        Self {
            path,
            enabled,
            lock: Mutex::new(()),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn record(&self, entry: LlmCallLog) {
        if !self.enabled {
            return;
        }
        let Ok(mut line) = serde_json::to_string(&LlmLogRecord {
            timestamp: Utc::now().to_rfc3339(),
            entry,
        }) else {
            return;
        };
        line.push('\n');

        let _guard = self.lock.lock().unwrap_or_else(|e| e.into_inner());
        if let Err(err) = self.rotate_if_needed() {
            warn!(?err, "rotación de log LLM falló");
        }
        match OpenOptions::new().create(true).append(true).open(&self.path) {
            Ok(mut file) => {
                if let Err(err) = file.write_all(line.as_bytes()) {
                    warn!(?err, "no se pudo escribir log LLM");
                }
            }
            Err(err) => warn!(?err, path = ?self.path, "no se pudo abrir log LLM"),
        }
    }

    fn rotate_if_needed(&self) -> std::io::Result<()> {
        let Ok(meta) = fs::metadata(&self.path) else {
            return Ok(());
        };
        if meta.len() < MAX_BYTES {
            return Ok(());
        }
        let rotated = self.path.with_extension("jsonl.old");
        let _ = fs::remove_file(&rotated);
        fs::rename(&self.path, &rotated)?;
        File::create(&self.path)?;
        Ok(())
    }
}

#[derive(Serialize)]
struct LlmLogRecord {
    timestamp: String,
    #[serde(flatten)]
    entry: LlmCallLog,
}
