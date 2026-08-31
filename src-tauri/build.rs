use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    incrustar_env_llm();
    tauri_build::build();
}

/// Lee `.env` / `.env.local` del repo y los deja en el binario como fallback.
/// En `tauri dev` gana el archivo en disco; en el `.app`/DMG no hay cwd del
/// proyecto, así que sin esto el proveedor cae a Ollama y la IA no corre.
fn incrustar_env_llm() {
    let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".."));
    for name in [".env", ".env.local"] {
        println!("cargo:rerun-if-changed={}", raiz.join(name).display());
    }

    let mut vars = HashMap::new();
    for name in [".env", ".env.local"] {
        let path = raiz.join(name);
        let Ok(iter) = dotenvy::from_path_iter(&path) else {
            continue;
        };
        for item in iter.flatten() {
            if item.0.starts_with("LUNMIA_") && !item.1.trim().is_empty() {
                vars.insert(item.0, item.1);
            }
        }
    }
    for (k, v) in vars {
        // Una sola línea: cargo rustc-env no admite saltos.
        let v = v.replace(['\n', '\r'], "");
        println!("cargo:rustc-env={k}={v}");
    }
}
