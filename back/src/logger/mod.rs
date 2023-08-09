use std::{
    sync::{mpsc, Arc},
    time::Duration,
};

use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use tracing_log::log::{Metadata, Record};

macro_rules! log {
    ($state:expr, $lvl:expr, $arg:expr) => {
        $state
            .logger
            .log(&RecordBuilder::new().level($lvl).args($arg).build());
    };
}
pub(crate) use log;

pub struct ActionLogger {
    entity: entities::log::ActiveModel,
}

async fn logger_recorder(log_rx: mpsc::Receiver<ActionLogger>, db: Arc<DatabaseConnection>) {
    loop {
        match log_rx.recv_timeout(Duration::from_secs(60)) {
            Ok(a) => {
                let db_clone = Arc::clone(&db);
                tokio::spawn(async move {
                    a.entity.insert(&*db_clone).await.unwrap();
                });
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                eprintln!("AppLogger disconnected; Logging has terminated");
                break;
            }
        };
    }
}

#[derive(Clone)]
pub struct AppLogger {
    log_tx: mpsc::SyncSender<ActionLogger>,
}

impl AppLogger {
    pub fn new(db: DatabaseConnection) -> AppLogger {
        let (log_tx, log_rx) = mpsc::sync_channel(0);
        tokio::spawn(async move { logger_recorder(log_rx, Arc::new(db)).await });
        AppLogger { log_tx }
    }
}

impl tracing_log::log::Log for AppLogger {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let log = {
            let line = match record.line() {
                Some(l) => l.to_string(),
                None => String::new(),
            };
            let description = format!("{line} {}", record.args());
            tracing_log::log::log!(record.level(), "{line} : {}", record.args());
            ActionLogger {
                entity: entities::log::ActiveModel {
                    description: Set(description),
                    created_at: Set(chrono::Utc::now().naive_utc()),
                    level: Set(record.level().to_string()),
                    ..Default::default()
                },
            }
        };
        self.log_tx.send(log).unwrap();
    }

    fn flush(&self) {}
}
