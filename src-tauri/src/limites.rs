//! Topes de texto. Deben coincidir con `src/lib/limites.ts`.

pub const MAX_CAPTURA: usize = 8_000;
pub const MAX_ENTRADA: usize = 12_000;
pub const MAX_OFICIO: usize = 1_500;
pub const MAX_NOMBRE: usize = 40;
pub const MAX_PROMPT: usize = 500;

pub fn asegurar_max(texto: &str, max: usize, campo: &str) -> Result<(), String> {
    let n = texto.chars().count();
    if n > max {
        Err(format!("{campo} supera el máximo de {max} caracteres."))
    } else {
        Ok(())
    }
}

pub fn truncar(texto: &str, max: usize) -> String {
    if texto.chars().count() <= max {
        texto.to_string()
    } else {
        texto.chars().take(max).collect()
    }
}
