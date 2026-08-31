use std::sync::Arc;

pub mod stub;

#[cfg(target_os = "macos")]
pub mod macos;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    pub text: String,
    pub confidence: f32,
    pub source_app: Option<String>,
    pub window_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechResult {
    pub transcript: String,
    pub duration_ms: u64,
    pub confidence: f32,
}

pub trait OcrEngine: Send + Sync {
    fn extract(&self, image_bytes: &[u8]) -> Result<OcrResult>;
    fn capture_and_extract(&self) -> Result<OcrResult>;
    fn available(&self) -> bool;
}

pub trait SpeechRecognizer: Send + Sync {
    fn recognize(&self, audio_bytes: &[u8]) -> Result<SpeechResult>;
    fn available(&self) -> bool;
}

pub trait Notifier: Send + Sync {
    fn notify(&self, title: &str, body: &str) -> Result<()>;
    fn request_permission(&self) -> Result<bool>;
    fn permission_granted(&self) -> bool;
}

pub trait AutoStart: Send + Sync {
    fn enable(&self) -> Result<()>;
    fn disable(&self) -> Result<()>;
    fn enabled(&self) -> bool;
}

pub struct Platform {
    pub ocr: Arc<dyn OcrEngine>,
    pub speech: Arc<dyn SpeechRecognizer>,
    pub notifier: Arc<dyn Notifier>,
    pub autostart: Arc<dyn AutoStart>,
}

impl Platform {
    pub fn new() -> Self {
        #[cfg(target_os = "macos")]
        {
            Self {
                ocr: Arc::new(macos::MacOcr::new()),
                speech: Arc::new(macos::MacSpeech::new()),
                notifier: Arc::new(stub::StubNotifier::default()),
                autostart: Arc::new(stub::StubAutoStart::default()),
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            Self {
                ocr: Arc::new(stub::StubOcr::default()),
                speech: Arc::new(stub::StubSpeech::default()),
                notifier: Arc::new(stub::StubNotifier::default()),
                autostart: Arc::new(stub::StubAutoStart::default()),
            }
        }
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self::new()
    }
}
