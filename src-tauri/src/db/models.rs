use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Espacio {
    pub espacio_id: String,
    pub nombre: String,
    pub tipo: String,
    pub color: String,
    pub fijado: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proyecto {
    pub proyecto_id: String,
    pub espacio_id: String,
    pub nombre: String,
    pub es_por_defecto: bool,
    pub fijado: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tipo {
    pub tipo_id: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub usos_total: i64,
    pub archivado: bool,
    pub ultima_uso_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entrada {
    pub entry_id: String,
    pub contenido_original: String,
    pub contenido: String,
    pub tipo_id: Option<String>,
    pub tipo_nombre: Option<String>,
    pub espacio_id: String,
    pub espacio_nombre: String,
    pub espacio_color: String,
    pub proyecto_id: String,
    pub proyecto_nombre: String,
    pub es_provisional: bool,
    pub estado: String,
    pub origen: String,
    pub metadata_captura: Option<String>,
    pub confianza: Option<f64>,
    pub confianza_capa: Option<i64>,
    pub etiquetas: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreguntaPendiente {
    pub pregunta_id: String,
    pub entry_id: String,
    pub orden: i64,
    pub texto: String,
    pub opciones: Vec<String>,
    pub respuesta_opcion: Option<i64>,
    pub respuesta_texto: Option<String>,
    pub resuelta: bool,
    pub descartada: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recordatorio {
    pub recordatorio_id: String,
    pub entry_id: Option<String>,
    pub titulo: String,
    pub cuerpo: Option<String>,
    pub dispara_at: String,
    pub disparado: bool,
    pub cancelado: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evento {
    pub evento_id: String,
    pub entry_id: Option<String>,
    pub espacio_id: String,
    pub espacio_nombre: String,
    pub espacio_color: String,
    pub proyecto_id: Option<String>,
    pub proyecto_nombre: Option<String>,
    pub titulo: String,
    pub descripcion: Option<String>,
    pub inicio_at: String,
    pub fin_at: String,
    pub all_day: bool,
    pub ubicacion: Option<String>,
    pub rrule: Option<String>,
    pub color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contexto {
    pub espacio_id: String,
    pub espacio_nombre: String,
    pub espacio_color: String,
    pub proyecto_id: String,
    pub proyecto_nombre: String,
    pub origen: String,
    pub etiqueta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub capturas_hoy: i64,
    pub sin_triage: i64,
    pub latencia_media_ms: i64,
    pub perdidas: i64,
    pub provisionales: i64,
}
