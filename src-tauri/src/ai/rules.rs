//! Capa 1: reglas deterministas (PRD §6.2, §6.5).
//!
//! Parser de fechas en español y detección de tipos-obvios por verbos/prefijos.
//! Cubre ~90% de expresiones cotidianas; el resto sube a la capa 3.

use chrono::{DateTime, Datelike, Duration, NaiveTime, TimeZone, Timelike, Utc, Weekday};
use chrono_tz::Tz;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct FechaDetectada {
    pub instante: DateTime<Utc>,
    /// Fin exclusivo. Si es None, el caller usa +1 día (all_day) o +1 hora.
    pub fin: Option<DateTime<Utc>>,
    pub all_day: bool,
    pub texto_fuente: String,
    pub confianza: f32,
}

impl FechaDetectada {
    pub fn fin_exclusivo(&self) -> DateTime<Utc> {
        self.fin.unwrap_or_else(|| {
            if self.all_day {
                self.instante + Duration::days(1)
            } else {
                self.instante + Duration::hours(1)
            }
        })
    }

    /// Hora a la que dispara el recordatorio (all-day → 9:30 en la zona del usuario).
    pub fn dispara_at(&self, tz: Tz) -> DateTime<Utc> {
        if self.all_day {
            local_a_utc(
                tz,
                self.instante.year(),
                self.instante.month(),
                self.instante.day(),
                default_time(),
            )
            .unwrap_or(self.instante)
        } else {
            self.instante
        }
    }

    /// Construye una fecha civil (día local + hora opcional) en la zona del usuario.
    pub fn desde_civil(
        tz: Tz,
        año: i32,
        mes: u32,
        dia: u32,
        hora: Option<NaiveTime>,
        all_day: bool,
        fuente: impl Into<String>,
    ) -> Option<Self> {
        let all_day = all_day || hora.is_none();
        let instante = if all_day {
            floating_midnight(año, mes, dia)?
        } else {
            local_a_utc(tz, año, mes, dia, hora.unwrap_or_else(default_time))?
        };
        Some(FechaDetectada {
            instante,
            fin: None,
            all_day,
            texto_fuente: fuente.into(),
            confianza: 0.9,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RulePrediccion {
    pub tipo_sugerido: Option<String>,
    pub fecha: Option<FechaDetectada>,
    pub confianza: f32,
}

// Patrones de tipo por verbo/prefijo. Orden importa: se prueban de arriba a abajo.
static TIPO_PATRONES: &[(&str, &str)] = &[
    (r"(?i)^\s*bug[:\-\s]", "bug"),
    (r"(?i)^\s*script[:\-\s]", "script"),
    (r"(?i)^\s*idea[:\-\s]", "idea"),
    (r"(?i)^\s*todo[:\-\s]", "tarea"),
    (r"(?i)^\s*tarea[:\-\s]", "tarea"),
    (r"(?i)^\s*nota[:\-\s]", "nota"),
    (r"(?i)^\s*recordar[:\-\s]", "recordatorio"),
    (r"(?i)^\s*recordatorio[:\-\s]", "recordatorio"),
    (r"(?i)^\s*recomendaci[oó]n[:\-\s]", "recomendación"),
    (r"(?i)\b(bug|error|crash|falla)\b", "bug"),
    (r"(?i)\bscript\b", "script"),
    (r"(?i)\brecordar\b", "recordatorio"),
    (r"(?i)\breuni[oó]n\b", "recordatorio"),
    (r"(?i)\btraer\b|\benviar\b|\bcomprar\b|\bpagar\b|\brenovar\b", "tarea"),
    (r"(?i)^\s*idea\b", "idea"),
];

static RE_HORA: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(?:a las |sobre las |\blas )(\d{1,2})(?::(\d{2}))?\s*(am|pm|h|hs)?").unwrap()
});

static RE_HORA_RELOJ: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(\d{1,2}):(\d{2})\s*(am|pm)?").unwrap()
});

static RE_RANGO_DIAS: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)(?:del?\s+)?(\d{1,2})(?:\s+de\s+(\w+))?\s+al?\s+(\d{1,2})(?:\s+de\s+(\w+))?",
    )
    .unwrap()
});

static RE_RANGO_SEMANA: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)del?\s+(lunes|martes|miércoles|miercoles|jueves|viernes|sábado|sabado|domingo)\s+al?\s+(lunes|martes|miércoles|miercoles|jueves|viernes|sábado|sabado|domingo)",
    )
    .unwrap()
});

pub fn analizar(texto: &str) -> RulePrediccion {
    analizar_en(texto, chrono_tz::UTC)
}

pub fn analizar_en(texto: &str, tz: Tz) -> RulePrediccion {
    let tipo_sugerido = detectar_tipo(texto);
    let fecha = detectar_fecha_en(texto, tz);
    let confianza = match (tipo_sugerido.is_some(), fecha.is_some()) {
        (true, true) => 0.92,
        (true, false) => 0.75,
        (false, true) => 0.72,
        (false, false) => 0.0,
    };
    RulePrediccion {
        tipo_sugerido,
        fecha,
        confianza,
    }
}

fn detectar_tipo(texto: &str) -> Option<String> {
    for (patron, tipo) in TIPO_PATRONES {
        let re = Regex::new(patron).unwrap();
        if re.is_match(texto) {
            return Some((*tipo).to_string());
        }
    }
    None
}

/// Detecta ~30 formas de fecha en español, incluyendo rangos (del 15 al 20).
pub fn detectar_fecha(texto: &str) -> Option<FechaDetectada> {
    detectar_fecha_en(texto, chrono_tz::UTC)
}

pub fn detectar_fecha_en(texto: &str, tz: Tz) -> Option<FechaDetectada> {
    let bajo = texto.to_lowercase();
    let ahora = Utc::now().with_timezone(&tz);
    let hora_opt = extraer_hora(&bajo);
    let all_day = hora_opt.is_none();
    let hora = hora_opt.unwrap_or_else(default_time);

    if let Some(rango) = detectar_rango_dias(&bajo, ahora, tz, all_day, hora) {
        return Some(rango);
    }
    if let Some(rango) = detectar_rango_semana(&bajo, ahora, tz, all_day, hora) {
        return Some(rango);
    }
    if let Some(rango) = detectar_esta_o_proxima_semana(&bajo, ahora, tz, all_day, hora) {
        return Some(rango);
    }

    macro_rules! relativo {
        ($frag:expr, $days:expr) => {
            if bajo.contains($frag) {
                let dia = mas_dias(ahora, $days);
                return Some(puntual(tz, dia, hora, all_day, $frag, 0.9));
            }
        };
    }

    relativo!("pasado mañana", 2);
    relativo!("mañana", 1);
    relativo!("hoy", 0);
    relativo!("esta noche", 0);
    relativo!("esta tarde", 0);
    relativo!("esta mañana", 0);

    for (frag, dia_semana) in [
        ("lunes", Weekday::Mon),
        ("martes", Weekday::Tue),
        ("miercoles", Weekday::Wed),
        ("miércoles", Weekday::Wed),
        ("jueves", Weekday::Thu),
        ("viernes", Weekday::Fri),
        ("sabado", Weekday::Sat),
        ("sábado", Weekday::Sat),
        ("domingo", Weekday::Sun),
    ] {
        if bajo.contains(frag) {
            let obj = proximo_dia_semana(ahora, dia_semana);
            return Some(puntual(tz, obj, hora, all_day, frag, 0.87));
        }
    }

    if let Some(m) = Regex::new(r"(?i)en\s+(\d+)\s+d[ií]a").unwrap().captures(&bajo) {
        if let Some(n) = m.get(1).and_then(|s| s.as_str().parse::<i64>().ok()) {
            let obj = mas_dias(ahora, n);
            return Some(puntual(tz, obj, hora, all_day, format!("en {n} días"), 0.9));
        }
    }

    if let Some(m) = Regex::new(r"(?i)el\s+(\d{1,2})(?:\s+de\s+(\w+))?").unwrap().captures(&bajo) {
        let dia = m.get(1).and_then(|s| s.as_str().parse::<u32>().ok());
        let mes = m.get(2).map(|s| mes_a_numero(s.as_str())).unwrap_or_else(|| ahora.month());
        if let Some(d) = dia {
            let año = if mes < ahora.month() || (mes == ahora.month() && d < ahora.day()) {
                ahora.year() + 1
            } else {
                ahora.year()
            };
            if let Some(base) = tz.with_ymd_and_hms(año, mes, d, 12, 0, 0).single() {
                return Some(puntual(tz, base, hora, all_day, format!("día {d}"), 0.85));
            }
        }
    }

    if bajo.contains("fin de mes") || bajo.contains("a fin de mes") {
        let año = ahora.year();
        let mes = ahora.month();
        let ultimo = ultimo_dia_del_mes(año, mes);
        if let Some(base) = tz.with_ymd_and_hms(año, mes, ultimo, 12, 0, 0).single() {
            return Some(puntual(tz, base, hora, all_day, "fin de mes", 0.78));
        }
    }

    None
}

fn puntual(
    tz: Tz,
    dia: DateTime<Tz>,
    hora: NaiveTime,
    all_day: bool,
    fuente: impl Into<String>,
    confianza: f32,
) -> FechaDetectada {
    let instante = if all_day {
        floating_midnight(dia.year(), dia.month(), dia.day())
            .unwrap_or_else(|| dia.with_timezone(&Utc))
    } else {
        local_a_utc(tz, dia.year(), dia.month(), dia.day(), hora)
            .unwrap_or_else(|| dia.with_timezone(&Utc))
    };
    FechaDetectada {
        instante,
        fin: None,
        all_day,
        texto_fuente: fuente.into(),
        confianza,
    }
}

fn detectar_rango_dias(
    bajo: &str,
    ahora: DateTime<Tz>,
    tz: Tz,
    all_day: bool,
    hora: NaiveTime,
) -> Option<FechaDetectada> {
    let cap = RE_RANGO_DIAS.captures(bajo)?;
    let dia_ini: u32 = cap.get(1)?.as_str().parse().ok()?;
    let dia_fin: u32 = cap.get(3)?.as_str().parse().ok()?;
    if !(1..=31).contains(&dia_ini) || !(1..=31).contains(&dia_fin) {
        return None;
    }
    let mes_ini = cap
        .get(2)
        .map(|s| mes_a_numero(s.as_str()))
        .unwrap_or_else(|| ahora.month());
    let mes_fin = cap.get(4).map(|s| mes_a_numero(s.as_str())).unwrap_or_else(|| {
        if dia_fin < dia_ini && cap.get(2).is_none() {
            if mes_ini == 12 { 1 } else { mes_ini + 1 }
        } else {
            mes_ini
        }
    });

    let mut año_ini = ahora.year();
    let mut año_fin = if mes_fin < mes_ini { año_ini + 1 } else { año_ini };
    let mut inicio = tz.with_ymd_and_hms(año_ini, mes_ini, dia_ini, 12, 0, 0).single()?;
    let mut fin_dia = tz.with_ymd_and_hms(año_fin, mes_fin, dia_fin, 12, 0, 0).single()?;
    if fin_dia < ahora {
        año_ini += 1;
        año_fin += 1;
        inicio = tz.with_ymd_and_hms(año_ini, mes_ini, dia_ini, 12, 0, 0).single()?;
        fin_dia = tz.with_ymd_and_hms(año_fin, mes_fin, dia_fin, 12, 0, 0).single()?;
    }
    let instante = if all_day {
        floating_midnight(inicio.year(), inicio.month(), inicio.day())?
    } else {
        local_a_utc(tz, inicio.year(), inicio.month(), inicio.day(), hora)?
    };
    let fin_cal = fin_dia.date_naive() + Duration::days(1);
    let fin = Some(floating_midnight(fin_cal.year(), fin_cal.month(), fin_cal.day())?);
    Some(FechaDetectada {
        instante,
        fin,
        all_day,
        texto_fuente: format!("del {dia_ini} al {dia_fin}"),
        confianza: 0.9,
    })
}

fn detectar_rango_semana(
    bajo: &str,
    ahora: DateTime<Tz>,
    tz: Tz,
    all_day: bool,
    hora: NaiveTime,
) -> Option<FechaDetectada> {
    let cap = RE_RANGO_SEMANA.captures(bajo)?;
    let wd_ini = parse_weekday(cap.get(1)?.as_str())?;
    let wd_fin = parse_weekday(cap.get(2)?.as_str())?;
    let inicio_dia = este_o_proximo(ahora, wd_ini);
    let span = {
        let a = wd_ini.num_days_from_monday() as i64;
        let b = wd_fin.num_days_from_monday() as i64;
        if a == b { 1 } else { (b - a).rem_euclid(7) + 1 }
    };
    let instante = if all_day {
        floating_midnight(inicio_dia.year(), inicio_dia.month(), inicio_dia.day())?
    } else {
        local_a_utc(tz, inicio_dia.year(), inicio_dia.month(), inicio_dia.day(), hora)?
    };
    let fin_cal = inicio_dia.date_naive() + Duration::days(span);
    let fin = Some(floating_midnight(fin_cal.year(), fin_cal.month(), fin_cal.day())?);
    Some(FechaDetectada {
        instante,
        fin,
        all_day,
        texto_fuente: format!("del {} al {}", cap.get(1)?.as_str(), cap.get(2)?.as_str()),
        confianza: 0.88,
    })
}

fn detectar_esta_o_proxima_semana(
    bajo: &str,
    ahora: DateTime<Tz>,
    tz: Tz,
    all_day: bool,
    hora: NaiveTime,
) -> Option<FechaDetectada> {
    let proxima = bajo.contains("próxima semana")
        || bajo.contains("proxima semana")
        || bajo.contains("semana que viene");
    let esta = bajo.contains("esta semana") || bajo.contains("toda la semana");
    if !proxima && !esta {
        return None;
    }
    let offset = ahora.weekday().num_days_from_monday() as i64;
    let lunes_esta = mas_dias(ahora, -offset);
    let lunes = if proxima {
        mas_dias(lunes_esta, 7)
    } else {
        lunes_esta
    };
    let instante = if all_day {
        floating_midnight(lunes.year(), lunes.month(), lunes.day())?
    } else {
        local_a_utc(tz, lunes.year(), lunes.month(), lunes.day(), hora)?
    };
    let fin_cal = lunes.date_naive() + Duration::days(7);
    let fin = Some(floating_midnight(fin_cal.year(), fin_cal.month(), fin_cal.day())?);
    Some(FechaDetectada {
        instante,
        fin,
        all_day,
        texto_fuente: if proxima {
            "próxima semana".into()
        } else {
            "esta semana".into()
        },
        confianza: 0.86,
    })
}

fn floating_midnight(año: i32, mes: u32, dia: u32) -> Option<DateTime<Utc>> {
    Utc.with_ymd_and_hms(año, mes, dia, 0, 0, 0).single()
}

fn local_a_utc(tz: Tz, año: i32, mes: u32, dia: u32, hora: NaiveTime) -> Option<DateTime<Utc>> {
    tz.with_ymd_and_hms(año, mes, dia, hora.hour(), hora.minute(), 0)
        .single()
        .map(|dt| dt.with_timezone(&Utc))
}

fn mas_dias(desde: DateTime<Tz>, n: i64) -> DateTime<Tz> {
    let tz = desde.timezone();
    let naive = desde.date_naive() + Duration::days(n);
    tz.with_ymd_and_hms(naive.year(), naive.month(), naive.day(), 12, 0, 0)
        .single()
        .unwrap_or(desde)
}

fn parse_weekday(nombre: &str) -> Option<Weekday> {
    match nombre.to_lowercase().as_str() {
        "lunes" => Some(Weekday::Mon),
        "martes" => Some(Weekday::Tue),
        "miercoles" | "miércoles" => Some(Weekday::Wed),
        "jueves" => Some(Weekday::Thu),
        "viernes" => Some(Weekday::Fri),
        "sabado" | "sábado" => Some(Weekday::Sat),
        "domingo" => Some(Weekday::Sun),
        _ => None,
    }
}

fn este_o_proximo(desde: DateTime<Tz>, objetivo: Weekday) -> DateTime<Tz> {
    let hoy_num = desde.weekday().num_days_from_monday() as i64;
    let obj_num = objetivo.num_days_from_monday() as i64;
    let diff = (obj_num - hoy_num).rem_euclid(7);
    mas_dias(desde, diff)
}

fn default_time() -> NaiveTime {
    NaiveTime::from_hms_opt(9, 30, 0).unwrap()
}

fn proximo_dia_semana(desde: DateTime<Tz>, objetivo: Weekday) -> DateTime<Tz> {
    let hoy_num = desde.weekday().num_days_from_monday() as i64;
    let obj_num = objetivo.num_days_from_monday() as i64;
    let diff = (obj_num - hoy_num).rem_euclid(7);
    let diff = if diff == 0 { 7 } else { diff };
    mas_dias(desde, diff)
}

fn ultimo_dia_del_mes(año: i32, mes: u32) -> u32 {
    let siguiente = if mes == 12 {
        Utc.with_ymd_and_hms(año + 1, 1, 1, 0, 0, 0).unwrap()
    } else {
        Utc.with_ymd_and_hms(año, mes + 1, 1, 0, 0, 0).unwrap()
    };
    (siguiente - Duration::days(1)).day()
}

fn mes_a_numero(nombre: &str) -> u32 {
    let n = nombre.to_lowercase();
    match n.as_str() {
        "enero" => 1,
        "febrero" => 2,
        "marzo" => 3,
        "abril" => 4,
        "mayo" => 5,
        "junio" => 6,
        "julio" => 7,
        "agosto" => 8,
        "septiembre" | "setiembre" => 9,
        "octubre" => 10,
        "noviembre" => 11,
        "diciembre" => 12,
        _ => Utc::now().month(),
    }
}

fn extraer_hora(bajo: &str) -> Option<NaiveTime> {
    let (horas_s, mins_s, sufijo_s) = if let Some(cap) = RE_HORA.captures(bajo) {
        (
            cap.get(1)?.as_str(),
            cap.get(2).map(|s| s.as_str()),
            cap.get(3).map(|s| s.as_str()),
        )
    } else if let Some(cap) = RE_HORA_RELOJ.captures(bajo) {
        (
            cap.get(1)?.as_str(),
            cap.get(2).map(|s| s.as_str()),
            cap.get(3).map(|s| s.as_str()),
        )
    } else {
        return None;
    };
    let mut horas: u32 = horas_s.parse().ok()?;
    if !(0..=23).contains(&horas) {
        return None;
    }
    let mins: u32 = mins_s.and_then(|s| s.parse().ok()).unwrap_or(0);
    if mins > 59 {
        return None;
    }
    let sufijo = sufijo_s.map(|s| s.to_ascii_lowercase());
    if let Some(s) = sufijo {
        if s == "pm" && horas < 12 {
            horas += 12;
        }
        if s == "am" && horas == 12 {
            horas = 0;
        }
    } else if horas <= 6 {
        // "las 3" a solas suele ser 15:00 en conversación.
        horas += 12;
    }
    NaiveTime::from_hms_opt(horas, mins, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn detecta_manana() {
        let f = detectar_fecha("recordar llamar mañana a las 10").unwrap();
        assert!(f.confianza > 0.8);
    }

    #[test]
    fn detecta_lunes() {
        let f = detectar_fecha("enviar propuesta el lunes").unwrap();
        assert!(f.confianza > 0.8);
    }

    #[test]
    fn detecta_tipo_idea() {
        let p = analizar("Idea: modo oscuro automático");
        assert_eq!(p.tipo_sugerido.as_deref(), Some("idea"));
    }

    #[test]
    fn detecta_tipo_bug() {
        let p = analizar("Bug: el flotante no recupera el foco");
        assert_eq!(p.tipo_sugerido.as_deref(), Some("bug"));
    }

    #[test]
    fn detecta_tipo_script() {
        let p = analizar("Script: exportar entradas a markdown");
        assert_eq!(p.tipo_sugerido.as_deref(), Some("script"));
    }

    #[test]
    fn detecta_rango_dias() {
        let f = detectar_fecha("vacaciones del 18 al 22 de agosto").unwrap();
        assert!(f.all_day);
        assert!(f.fin.is_some());
        let span = (f.fin_exclusivo() - f.instante).num_days();
        assert_eq!(span, 5, "del 18 al 22 inclusive son 5 días, fin exclusivo al 23");
    }

    #[test]
    fn detecta_rango_semana() {
        let f = detectar_fecha("sprint del lunes al viernes").unwrap();
        assert!(f.all_day);
        let span = (f.fin_exclusivo() - f.instante).num_days();
        assert_eq!(span, 5);
    }

    #[test]
    fn detecta_esta_semana() {
        let f = detectar_fecha("reunión esta semana").unwrap();
        assert!(f.all_day);
        assert!(f.fin.is_some());
        assert_eq!((f.fin_exclusivo() - f.instante).num_days(), 7);
    }

    #[test]
    fn con_hora_no_es_all_day() {
        let f = detectar_fecha("recordar llamar mañana a las 10").unwrap();
        assert!(!f.all_day);
        let mx = chrono_tz::America::Mexico_City;
        let f2 = detectar_fecha_en("reunión mañana a las 16:00", mx).unwrap();
        assert!(!f2.all_day);
        assert_eq!(f2.instante.with_timezone(&mx).hour(), 16);
    }
}
