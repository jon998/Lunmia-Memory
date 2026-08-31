pub mod models;
pub mod queries;

use std::path::PathBuf;

use anyhow::{Context, Result};
use parking_lot::Mutex;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub type SqlitePool = Pool<SqliteConnectionManager>;

const SCHEMA_SQL: &str = include_str!("schema.sql");

pub struct Database {
    pool: SqlitePool,
    _lock: Mutex<()>,
}

impl Database {
    pub fn open(path: PathBuf) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context("crear carpeta de datos")?;
        }
        let manager = SqliteConnectionManager::file(&path).with_init(|conn| {
            conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")
        });
        let pool = Pool::builder().max_size(4).build(manager)?;
        let db = Self {
            pool,
            _lock: Mutex::new(()),
        };
        db.migrate()?;
        queries::asegurar_tipos_por_defecto(&db)
            .context("sembrar tipos por defecto")?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        let conn = self.pool.get()?;
        conn.execute_batch(SCHEMA_SQL)
            .context("aplicar esquema base")?;
        Ok(())
    }

    pub fn conn(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        Ok(self.pool.get()?)
    }
}

pub fn default_path() -> PathBuf {
    let carpeta = if cfg!(debug_assertions) {
        "Lunmia Memory (dev)"
    } else {
        "Lunmia Memory"
    };
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(carpeta);
    base.join("lunmia.db")
}
