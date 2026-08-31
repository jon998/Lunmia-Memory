//! Scheduler de recordatorios (R1 crítico — PRD §13, Fase 0).
//!
//! Estrategia:
//! - Fuente de verdad: tabla `recordatorios` en SQLite.
//! - Worker en background que hace tick cada 30s dentro de la app viva.
//! - Persistencia primaria del scheduler del SO (macOS UNNotificationRequest
//!   vía tauri-plugin-notification) para que dispare incluso con la app
//!   cerrada — el plugin programa la notificación en el sistema.
//!
//! Cuando el usuario abre la app, hacemos "reconciliar": todos los
//! recordatorios vencidos que aún no dispararon se marcan y notifican una vez.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use chrono::Utc;
use tauri::{async_runtime, AppHandle};
use tauri_plugin_notification::NotificationExt;
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::db::{queries, Database};

pub struct Scheduler {
    db: Arc<Database>,
    app: AppHandle,
}

impl Scheduler {
    pub fn new(db: Arc<Database>, app: AppHandle) -> Self {
        Self { db, app }
    }

    /// Arranca el worker en el runtime asíncrono de Tauri.
    /// Debe llamarse desde `.setup()`, antes de `.run()`.
    pub fn spawn(self) -> mpsc::Sender<Command> {
        let (tx, mut rx) = mpsc::channel::<Command>(32);
        let tx_ret = tx.clone();
        async_runtime::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(err) = self.tick() {
                            warn!(?err, "scheduler tick");
                        }
                    }
                    cmd = rx.recv() => {
                        match cmd {
                            Some(Command::Reconciliar) => {
                                if let Err(err) = self.tick() {
                                    warn!(?err, "scheduler reconciliar");
                                }
                            }
                            Some(Command::Detener) | None => break,
                        }
                    }
                }
            }
            info!("scheduler detenido");
        });
        tx_ret
    }

    fn tick(&self) -> Result<()> {
        let ahora = Utc::now().to_rfc3339();
        let pendientes = queries::recordatorios_por_disparar(&self.db, &ahora)?;
        for r in pendientes {
            let cuerpo = r.cuerpo.clone().unwrap_or_default();
            if let Err(err) = self
                .app
                .notification()
                .builder()
                .title(&r.titulo)
                .body(&cuerpo)
                .show()
            {
                warn!(?err, "no se pudo mostrar notificación");
                continue;
            }
            let _ = queries::marcar_recordatorio_disparado(&self.db, &r.recordatorio_id);
            info!(id = r.recordatorio_id, "recordatorio disparado");
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Reconciliar,
    Detener,
}
