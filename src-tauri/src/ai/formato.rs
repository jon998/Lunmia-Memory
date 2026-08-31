//! Formato local del contenido (sin LLM): vallas de código para SQL/scripts.

pub fn envolver_si_codigo(texto: &str, tipo_nombre: Option<&str>) -> Option<String> {
    let t = texto.trim();
    if t.is_empty() || t.contains("```") {
        return None;
    }
    let lang = detectar_lenguaje(t, tipo_nombre)?;
    Some(format!("```{lang}\n{t}\n```"))
}

fn detectar_lenguaje(texto: &str, tipo_nombre: Option<&str>) -> Option<&'static str> {
    let bajo = texto.to_lowercase();
    if es_sql(&bajo) {
        return Some("sql");
    }
    if bajo.contains("fn ") && (bajo.contains("let ") || bajo.contains("impl ") || bajo.contains("mut ")) {
        return Some("rust");
    }
    if bajo.contains("def ") && (bajo.contains("import ") || bajo.contains("self") || bajo.contains("print(")) {
        return Some("python");
    }
    if bajo.contains("#!/") || bajo.starts_with("curl ") || bajo.contains("chmod ") {
        return Some("bash");
    }
    if (bajo.contains("function ") || bajo.contains("const ") || bajo.contains("=>") || bajo.contains("import "))
        && (bajo.contains('{') || bajo.contains(';'))
    {
        return Some("ts");
    }
    let tipo = tipo_nombre.unwrap_or("");
    if tipo == "script" && parece_codigo(texto) {
        return Some("txt");
    }
    None
}

fn es_sql(bajo: &str) -> bool {
    let hay_select = bajo.contains("select ") && bajo.contains(" from ");
    let hay_ddl = bajo.contains("create table")
        || bajo.contains("alter table")
        || bajo.contains("insert into")
        || (bajo.contains("update ") && bajo.contains(" set "));
    hay_select || hay_ddl
}

fn parece_codigo(texto: &str) -> bool {
    let llaves = texto.chars().filter(|c| *c == '{' || *c == '}' || *c == ';').count();
    llaves >= 3 || texto.lines().count() >= 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envuelve_sql() {
        let s = "SELECT id, nombre FROM usuarios WHERE activo = 1;";
        let out = envolver_si_codigo(s, Some("script")).unwrap();
        assert!(out.starts_with("```sql\n"));
        assert!(out.ends_with("\n```"));
    }

    #[test]
    fn no_toca_si_ya_hay_valla() {
        let s = "```sql\nSELECT 1;\n```";
        assert!(envolver_si_codigo(s, Some("script")).is_none());
    }

    #[test]
    fn nota_corta_se_queda() {
        assert!(envolver_si_codigo("comprar leche mañana", Some("nota")).is_none());
    }
}
