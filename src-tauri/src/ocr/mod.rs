//! OCR — envoltorio delgado sobre el trait de plataforma.
//! Toda la lógica FFI vive en platform/macos.rs.
//!
//! Regla dura del PRD (§6.4): la imagen se procesa y descarta inmediatamente.

use std::sync::Arc;

use anyhow::Result;

use crate::platform::{OcrResult, Platform};

pub struct OcrService {
    platform: Arc<Platform>,
}

impl OcrService {
    pub fn new(platform: Arc<Platform>) -> Self {
        Self { platform }
    }

    pub fn capturar_pantalla(&self) -> Result<OcrResult> {
        self.platform.ocr.capture_and_extract()
    }

    pub fn desde_imagen(&self, bytes: &[u8]) -> Result<OcrResult> {
        self.platform.ocr.extract(bytes)
    }

    pub fn disponible(&self) -> bool {
        self.platform.ocr.available()
    }
}
