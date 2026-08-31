use chrono_tz::Tz;

use crate::db::{queries, Database};

pub const CLAVE: &str = "zona_horaria";

pub fn parsear(s: &str) -> Tz {
    s.parse().unwrap_or(chrono_tz::UTC)
}

pub fn de_db(db: &Database) -> Tz {
    if let Ok(Some(s)) = queries::get_config(db, CLAVE) {
        if !s.is_empty() {
            return parsear(&s);
        }
    }
    std::env::var("TZ")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(chrono_tz::UTC)
}
