use anyhow::Result;
use parking_lot::Mutex;

use super::{AutoStart, Notifier, OcrEngine, OcrResult, SpeechRecognizer, SpeechResult};

#[derive(Default)]
pub struct StubOcr;

impl OcrEngine for StubOcr {
    fn extract(&self, _image_bytes: &[u8]) -> Result<OcrResult> {
        Ok(OcrResult {
            text: "[OCR no disponible fuera de macOS]".to_string(),
            confidence: 0.0,
            source_app: None,
            window_title: None,
        })
    }
    fn capture_and_extract(&self) -> Result<OcrResult> {
        Ok(OcrResult {
            text: "[Captura de pantalla no disponible fuera de macOS]".to_string(),
            confidence: 0.0,
            source_app: None,
            window_title: None,
        })
    }
    fn available(&self) -> bool {
        false
    }
}

#[derive(Default)]
pub struct StubSpeech;

impl SpeechRecognizer for StubSpeech {
    fn recognize(&self, _audio_bytes: &[u8]) -> Result<SpeechResult> {
        Ok(SpeechResult {
            transcript: String::new(),
            duration_ms: 0,
            confidence: 0.0,
        })
    }
    fn available(&self) -> bool {
        false
    }
}

#[derive(Default)]
pub struct StubNotifier {
    granted: Mutex<bool>,
}

impl Notifier for StubNotifier {
    fn notify(&self, title: &str, body: &str) -> Result<()> {
        tracing::info!(title, body, "notify (stub)");
        Ok(())
    }
    fn request_permission(&self) -> Result<bool> {
        *self.granted.lock() = true;
        Ok(true)
    }
    fn permission_granted(&self) -> bool {
        *self.granted.lock()
    }
}

#[derive(Default)]
pub struct StubAutoStart {
    on: Mutex<bool>,
}

impl AutoStart for StubAutoStart {
    fn enable(&self) -> Result<()> {
        *self.on.lock() = true;
        Ok(())
    }
    fn disable(&self) -> Result<()> {
        *self.on.lock() = false;
        Ok(())
    }
    fn enabled(&self) -> bool {
        *self.on.lock()
    }
}
