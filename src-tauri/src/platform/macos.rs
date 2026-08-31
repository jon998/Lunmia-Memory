//! Implementación macOS de los traits de plataforma.
//!
//! El OCR real usa el framework Vision (VNRecognizeTextRequest). En esta
//! primera versión del MVP1 llamamos al binario `screencapture -i` de macOS
//! para tomar la selección y una llamada FFI mínima hará el reconocimiento.
//! La imagen se descarta inmediatamente (§6.4 del PRD).

use std::process::Command;

use anyhow::{anyhow, Result};

use super::{OcrEngine, OcrResult, SpeechRecognizer, SpeechResult};

/// Comprueba si la app tiene permiso de Accesibilidad concedido.
/// Necesario para atajos globales tipo ⌘⇧Space encima de cualquier app.
pub fn accesibilidad_concedida() -> bool {
    unsafe { AXIsProcessTrusted() }
}

/// Abre System Settings → Privacy & Security → Accessibility.
pub fn abrir_ajustes_accesibilidad() {
    let _ = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .status();
}

/// Abre System Settings → Privacy & Security → Microphone.
pub fn abrir_ajustes_microfono() {
    let _ = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Microphone")
        .status();
}

/// Abre System Settings → Notifications, filtrado a nuestra app cuando aplica.
pub fn abrir_ajustes_notificaciones() {
    let _ = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.notifications")
        .status();
}

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
}

pub struct MacOcr;

impl MacOcr {
    pub fn new() -> Self {
        Self
    }
}

impl OcrEngine for MacOcr {
    fn extract(&self, _image_bytes: &[u8]) -> Result<OcrResult> {
        // TODO Fase 1: FFI a Vision. Por ahora placeholder honesto.
        Ok(OcrResult {
            text: String::new(),
            confidence: 0.0,
            source_app: current_frontmost_app(),
            window_title: None,
        })
    }

    fn capture_and_extract(&self) -> Result<OcrResult> {
        let tmp = std::env::temp_dir().join(format!("lunmia-ocr-{}.png", uuid_ish()));
        let status = Command::new("screencapture")
            .args(["-i", "-x", tmp.to_string_lossy().as_ref()])
            .status()
            .map_err(|e| anyhow!("no se pudo ejecutar screencapture: {e}"))?;
        if !status.success() {
            return Err(anyhow!("captura cancelada"));
        }
        let bytes = std::fs::read(&tmp).ok().unwrap_or_default();
        let _ = std::fs::remove_file(&tmp);
        let result = self.extract(&bytes)?;
        Ok(result)
    }

    fn available(&self) -> bool {
        true
    }
}

pub struct MacSpeech;

impl MacSpeech {
    pub fn new() -> Self {
        Self
    }
}

impl SpeechRecognizer for MacSpeech {
    fn recognize(&self, _audio_bytes: &[u8]) -> Result<SpeechResult> {
        // TODO D1: FFI a SFSpeechRecognizer del framework Speech.
        Ok(SpeechResult {
            transcript: String::new(),
            duration_ms: 0,
            confidence: 0.0,
        })
    }
    fn available(&self) -> bool {
        true
    }
}

fn current_frontmost_app() -> Option<String> {
    let out = Command::new("osascript")
        .args([
            "-e",
            "tell application \"System Events\" to get name of first process whose frontmost is true",
        ])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn uuid_ish() -> String {
    uuid::Uuid::now_v7().to_string()
}
