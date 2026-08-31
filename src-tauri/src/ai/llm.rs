//! Servicio LLM centralizado.
//!
//! Toda petición al modelo (clasificación, embeddings, propuestas de onboarding)
//! pasa por aquí. Un único worker serializa las llamadas (§9 R7) y cada llamada
//! queda registrada en:
//!   1. `tracing` (nivel INFO) para stdout durante desarrollo.
//!   2. Archivo JSONL en `<data_local>/Lunmia Memory/logs/llm.jsonl`.
//!
//! Ambos incluyen: tag, proveedor, modelo, prompt, respuesta, latencia,
//! tokens (si el proveedor los reporta) y error.
//!
//! Proveedores soportados:
//!   - `ollama`     → local, endpoint nativo `/api/generate` (default).
//!   - `dashscope`  → Alibaba, OpenAI-compat `/chat/completions`.
//!
//! Variables de entorno:
//!   LUNMIA_LLM_PROVIDER   ollama | dashscope   (default: ollama)
//!   LUNMIA_LLM_API_KEY    <requerida si dashscope>
//!   LUNMIA_LLM_BASE_URL   sobreescribe la URL base (opcional)
//!   LUNMIA_LLM_MODEL      modelo del clasificador (default por proveedor)
//!   LUNMIA_EMBED_MODEL    modelo de embeddings (default por proveedor)
//!   LUNMIA_OLLAMA_HOST    host de Ollama (default http://127.0.0.1:11434)
//!   LUNMIA_LLM_LOG        1 (default) para escribir a archivo; 0 para desactivar.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use tokio::sync::Mutex;
use tracing::info;

use super::llm_log::{LlmCallLog, LlmLogger};

const DEFAULT_OLLAMA_HOST: &str = "http://127.0.0.1:11434";
const DEFAULT_DASHSCOPE_URL: &str = "https://dashscope-intl.aliyuncs.com/compatible-mode/v1";

#[derive(Debug, Clone)]
pub enum Provider {
    Ollama { host: String },
    Dashscope { base_url: String, api_key: String },
}

impl Provider {
    pub fn name(&self) -> &'static str {
        match self {
            Provider::Ollama { .. } => "ollama",
            Provider::Dashscope { .. } => "dashscope",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenUsage {
    pub prompt: Option<u32>,
    pub completion: Option<u32>,
    pub total: Option<u32>,
}

pub struct LlmClient {
    client: reqwest::Client,
    provider: Provider,
    queue: Arc<Mutex<()>>,
    logger: Arc<LlmLogger>,
    pub embedding_model: String,
    pub classifier_model: String,
}

impl LlmClient {
    pub fn new() -> Self {
        let provider_str = env_llm("LUNMIA_LLM_PROVIDER")
            .unwrap_or_else(|| "ollama".into())
            .to_lowercase();

        let provider = match provider_str.as_str() {
            "dashscope" | "qwen" | "alibaba" => {
                let api_key = env_llm("LUNMIA_LLM_API_KEY").unwrap_or_default();
                let base_url = env_llm("LUNMIA_LLM_BASE_URL")
                    .unwrap_or_else(|| DEFAULT_DASHSCOPE_URL.into());
                Provider::Dashscope { base_url, api_key }
            }
            _ => {
                let host = env_llm("LUNMIA_OLLAMA_HOST")
                    .unwrap_or_else(|| DEFAULT_OLLAMA_HOST.into());
                Provider::Ollama { host }
            }
        };

        let (default_embed, default_llm) = match &provider {
            Provider::Ollama { .. } => ("bge-m3", "qwen2.5:3b"),
            Provider::Dashscope { .. } => ("text-embedding-v3", "qwen-flash"),
        };

        let embedding_model =
            env_llm("LUNMIA_EMBED_MODEL").unwrap_or_else(|| default_embed.into());
        let classifier_model =
            env_llm("LUNMIA_LLM_MODEL").unwrap_or_else(|| default_llm.into());

        let logger = Arc::new(LlmLogger::new(log_path()));
        let key_present = match &provider {
            Provider::Ollama { .. } => true,
            Provider::Dashscope { api_key, .. } => !api_key.is_empty(),
        };

        info!(
            provider = provider.name(),
            embedding_model = %embedding_model,
            classifier_model = %classifier_model,
            key_present,
            log_path = ?logger.path(),
            "LLM configurado"
        );

        let client = Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(90))
                .build()
                .expect("reqwest builder"),
            provider,
            queue: Arc::new(Mutex::new(())),
            logger,
            embedding_model,
            classifier_model,
        };
        client.logger.record(LlmCallLog {
            tag: "boot".into(),
            operation: "config".into(),
            provider: client.provider.name().into(),
            model: client.classifier_model.clone(),
            prompt: String::new(),
            response: format!(
                "provider={} model={} embed={} key_present={}",
                client.provider.name(),
                client.classifier_model,
                client.embedding_model,
                key_present
            ),
            latency_ms: 0,
            tokens: None,
            ok: key_present,
            error: if key_present {
                None
            } else {
                Some("LUNMIA_LLM_API_KEY vacía; el .app no hereda el .env de tauri dev".into())
            },
        });
        client
    }

    pub fn provider_name(&self) -> &'static str {
        self.provider.name()
    }

    pub fn log_path(&self) -> PathBuf {
        self.logger.path().to_path_buf()
    }

    /// Petición de embedding. `tag` identifica el call-site en los logs
    /// (ej. "clasificar_capa2", "onboarding_similitud").
    pub async fn embed(&self, tag: &str, prompt: &str) -> Result<Vec<f32>> {
        let _guard = self.queue.lock().await;
        let started = Instant::now();
        let model = self.embedding_model.clone();
        let provider = self.provider.name();
        let (result, response_summary) = match &self.provider {
            Provider::Ollama { host } => {
                let url = format!("{}/api/embeddings", host);
                let body = serde_json::json!({
                    "model": model,
                    "prompt": prompt,
                    "keep_alive": "-1",
                });
                match post_json(&self.client, &url, None, &body).await {
                    Ok(value) => {
                        let sum = format!(
                            "dims={}",
                            value["embedding"].as_array().map(|a| a.len()).unwrap_or(0)
                        );
                        (extract_ollama_embedding(&value), sum)
                    }
                    Err(err) => (Err(err), String::new()),
                }
            }
            Provider::Dashscope { base_url, api_key } => {
                let url = format!("{}/embeddings", base_url);
                let body = serde_json::json!({
                    "model": model,
                    "input": prompt,
                });
                match post_json(&self.client, &url, Some(api_key), &body).await {
                    Ok(value) => {
                        let dims = value["data"][0]["embedding"]
                            .as_array()
                            .map(|a| a.len())
                            .unwrap_or(0);
                        let sum = format!("dims={dims}");
                        (extract_openai_embedding(&value), sum)
                    }
                    Err(err) => (Err(err), String::new()),
                }
            }
        };
        let latency_ms = started.elapsed().as_millis() as u64;
        let (ok, err) = match &result {
            Ok(_) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };
        self.logger.record(LlmCallLog {
            tag: tag.to_string(),
            operation: "embed".into(),
            provider: provider.into(),
            model,
            prompt: prompt.to_string(),
            response: response_summary.clone(),
            latency_ms,
            tokens: None,
            ok,
            error: err.clone(),
        });
        info!(
            tag,
            operation = "embed",
            provider,
            model = %self.embedding_model,
            latency_ms,
            ok,
            summary = %response_summary,
            "llm.embed"
        );
        result
    }

    /// Petición de generación con salida JSON (usada por el clasificador y
    /// otras propuestas estructuradas).
    pub async fn generate_json(&self, tag: &str, prompt: &str) -> Result<String> {
        let _guard = self.queue.lock().await;
        let started = Instant::now();
        let model = self.classifier_model.clone();
        let provider = self.provider.name();
        let (result, tokens) = match &self.provider {
            Provider::Ollama { host } => {
                let url = format!("{}/api/generate", host);
                let body = serde_json::json!({
                    "model": model,
                    "prompt": prompt,
                    "stream": false,
                    "keep_alive": "5m",
                    "format": "json",
                });
                match post_json(&self.client, &url, None, &body).await {
                    Ok(value) => {
                        let response = value["response"].as_str().unwrap_or_default().to_string();
                        let tokens = TokenUsage {
                            prompt: value["prompt_eval_count"].as_u64().map(|x| x as u32),
                            completion: value["eval_count"].as_u64().map(|x| x as u32),
                            total: None,
                        };
                        (Ok(response), Some(tokens))
                    }
                    Err(err) => (Err(err), None),
                }
            }
            Provider::Dashscope { base_url, api_key } => {
                let url = format!("{}/chat/completions", base_url);
                let body = serde_json::json!({
                    "model": model,
                    "messages": [{ "role": "user", "content": prompt }],
                    "response_format": { "type": "json_object" },
                    "stream": false,
                });
                match post_json(&self.client, &url, Some(api_key), &body).await {
                    Ok(value) => {
                        let response = value["choices"][0]["message"]["content"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        let tokens = TokenUsage {
                            prompt: value["usage"]["prompt_tokens"].as_u64().map(|x| x as u32),
                            completion: value["usage"]["completion_tokens"]
                                .as_u64()
                                .map(|x| x as u32),
                            total: value["usage"]["total_tokens"].as_u64().map(|x| x as u32),
                        };
                        (Ok(response), Some(tokens))
                    }
                    Err(err) => (Err(err), None),
                }
            }
        };
        let latency_ms = started.elapsed().as_millis() as u64;
        let (ok, err, response_text) = match &result {
            Ok(text) => (true, None, text.clone()),
            Err(e) => (false, Some(e.to_string()), String::new()),
        };
        self.logger.record(LlmCallLog {
            tag: tag.to_string(),
            operation: "chat_json".into(),
            provider: provider.into(),
            model: model.clone(),
            prompt: prompt.to_string(),
            response: response_text.clone(),
            latency_ms,
            tokens: tokens.clone(),
            ok,
            error: err.clone(),
        });
        info!(
            tag,
            operation = "chat_json",
            provider,
            model = %model,
            latency_ms,
            ok,
            prompt_tokens = tokens.as_ref().and_then(|t| t.prompt).unwrap_or(0),
            completion_tokens = tokens.as_ref().and_then(|t| t.completion).unwrap_or(0),
            "llm.chat_json\n----- prompt -----\n{prompt}\n----- respuesta -----\n{response_text}\n-----"
        );
        result
    }

    pub async fn ping(&self) -> bool {
        self.diagnostico().await.0
    }

    /// ¿El proveedor responde? `aviso` explica al usuario por qué no, si falla.
    pub async fn diagnostico(&self) -> (bool, Option<String>) {
        if let Provider::Dashscope { api_key, .. } = &self.provider {
            if api_key.trim().is_empty() {
                return (
                    false,
                    Some("No hay API key de la IA. La nota se guardará tal cual.".into()),
                );
            }
        }
        let ok = match &self.provider {
            Provider::Ollama { host } => {
                let url = format!("{}/api/tags", host);
                matches!(self.client.get(&url).send().await, Ok(r) if r.status().is_success())
            }
            Provider::Dashscope { base_url, api_key } => {
                let url = format!("{}/models", base_url);
                matches!(
                    self.client.get(&url).bearer_auth(api_key).send().await,
                    Ok(r) if r.status().is_success()
                )
            }
        };
        if ok {
            return (true, None);
        }
        let aviso = match &self.provider {
            Provider::Ollama { host } => format!(
                "Ollama no responde ({host}). La nota se guardará tal cual."
            ),
            Provider::Dashscope { .. } => {
                "Sin conexión con la IA. La nota se guardará tal cual.".into()
            }
        };
        (false, Some(aviso))
    }

    /// Deja rastro en llm.jsonl cuando se salta el modelo (ping, key, etc.).
    pub fn log_skip(&self, tag: &str, reason: &str) {
        self.logger.record(LlmCallLog {
            tag: tag.to_string(),
            operation: "skip".into(),
            provider: self.provider.name().into(),
            model: self.classifier_model.clone(),
            prompt: String::new(),
            response: String::new(),
            latency_ms: 0,
            tokens: None,
            ok: false,
            error: Some(reason.to_string()),
        });
    }
}

/// Runtime (dotenv / entorno) gana; si el `.app` no tiene .env, usa lo incrustado al compilar.
fn env_llm(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| compiled_llm(key).map(|s| s.to_string()))
}

fn compiled_llm(key: &str) -> Option<&'static str> {
    match key {
        "LUNMIA_LLM_PROVIDER" => option_env!("LUNMIA_LLM_PROVIDER"),
        "LUNMIA_LLM_API_KEY" => option_env!("LUNMIA_LLM_API_KEY"),
        "LUNMIA_LLM_BASE_URL" => option_env!("LUNMIA_LLM_BASE_URL"),
        "LUNMIA_LLM_MODEL" => option_env!("LUNMIA_LLM_MODEL"),
        "LUNMIA_EMBED_MODEL" => option_env!("LUNMIA_EMBED_MODEL"),
        "LUNMIA_OLLAMA_HOST" => option_env!("LUNMIA_OLLAMA_HOST"),
        _ => None,
    }
}

impl Default for LlmClient {
    fn default() -> Self {
        Self::new()
    }
}

fn log_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Lunmia Memory")
        .join("logs")
        .join("llm.jsonl")
}

async fn post_json(
    client: &reqwest::Client,
    url: &str,
    bearer: Option<&str>,
    body: &serde_json::Value,
) -> Result<serde_json::Value> {
    let mut req = client.post(url).json(body);
    if let Some(token) = bearer {
        req = req.bearer_auth(token);
    }
    let res = req.send().await.with_context(|| format!("POST {url}"))?;
    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(anyhow!("HTTP {} desde {}: {}", status, url, body));
    }
    res.json::<serde_json::Value>()
        .await
        .with_context(|| format!("decodificando respuesta de {url}"))
}

fn extract_ollama_embedding(v: &serde_json::Value) -> Result<Vec<f32>> {
    let arr = v["embedding"]
        .as_array()
        .ok_or_else(|| anyhow!("respuesta sin 'embedding'"))?;
    Ok(arr.iter().filter_map(|x| x.as_f64().map(|n| n as f32)).collect())
}

fn extract_openai_embedding(v: &serde_json::Value) -> Result<Vec<f32>> {
    let arr = v["data"][0]["embedding"]
        .as_array()
        .ok_or_else(|| anyhow!("respuesta sin data[0].embedding"))?;
    Ok(arr.iter().filter_map(|x| x.as_f64().map(|n| n as f32)).collect())
}
