use std::fmt;

use async_session::log::RecordBuilder;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info};
use tracing_log::log::Record;

macro_rules! log {
    ($state:expr, $lvl:expr, $arg:expr, $user:expr) => {
        $state.logger.log(
            &AppRecordBuilder::new()
                .level($lvl)
                .args($arg)
                .user($user)
                .build(),
        );
    };
}
pub(crate) use log;
use uuid::Uuid;

use crate::session::{UserId, UserIdFromSession};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
enum ActionCommand {
    #[default]
    Record,
    Flush,
    Exit,
}

#[derive(Debug, Clone)]
pub struct ActionLogger {
    command: ActionCommand,
    entity: Option<entities::log::ActiveModel>,
}

#[derive(Clone)]
pub struct AppLogger {
    sender: Sender<ActionLogger>,
}

#[derive(Clone, Debug)]
pub struct AppRecord<'a> {
    record: Option<Record<'a>>,
    user: UserId,
}

#[derive(Debug)]
pub struct AppRecordBuilder<'a> {
    record: AppRecord<'a>,
    inner_builder: RecordBuilder<'a>,
}

impl<'a> AppRecordBuilder<'a> {
    pub fn new() -> AppRecordBuilder<'a> {
        AppRecordBuilder {
            record: AppRecord {
                record: None,
                user: UserId {
                    user_id: Uuid::nil(),
                    username: "SYSTEM".to_owned(),
                },
            },
            inner_builder: RecordBuilder::new(),
        }
    }

    pub fn user(&mut self, user: UserIdFromSession) -> &mut AppRecordBuilder<'a> {
        match user {
            UserIdFromSession::FoundUserId(user) => self.record.user = user,
            UserIdFromSession::NotFound() => {}
        };
        self
    }

    pub fn level(&mut self, level: tracing_log::log::Level) -> &mut AppRecordBuilder<'a> {
        self.inner_builder.level(level);
        self
    }

    pub fn args(&mut self, args: fmt::Arguments<'a>) -> &mut AppRecordBuilder<'a> {
        self.inner_builder.args(args);
        self
    }

    pub fn build(&mut self) -> AppRecord<'a> {
        self.record.record = Some(self.inner_builder.build());
        self.record.clone()
    }
}

async fn handle_logs(mut receiver: Receiver<ActionLogger>, db: DatabaseConnection) {
    let mut messages = Vec::new();
    'main: loop {
        let mut call_flush = false;
        let mut exit = false;
        'rx: loop {
            match receiver.recv().await {
                Some(a) => match a.command {
                    ActionCommand::Record => {
                        messages.push(a);
                        if messages.len() > 10 {
                            messages.push(ActionLogger {
                                command: ActionCommand::Flush,
                                entity: None,
                            });
                            break 'rx;
                        }
                    }
                    ActionCommand::Flush | ActionCommand::Exit => {
                        call_flush = true;
                        break 'rx;
                    }
                },
                None => {
                    call_flush = true;
                    break 'rx;
                }
            }
        }
        if call_flush {
            exit = flush(&mut messages, &db).await;
        }
        if exit {
            info!("Exiting {}", messages.len());
            break 'main;
        }
    }
}

impl AppLogger {
    pub fn new(db: DatabaseConnection) -> AppLogger {
        let (sender, receiver) = tokio::sync::mpsc::channel::<ActionLogger>(256);
        tokio::spawn(async move { handle_logs(receiver, db).await });
        AppLogger { sender }
    }

    pub fn terminate(&mut self) {
        self.sender
            .try_send(ActionLogger {
                command: ActionCommand::Exit,
                entity: None,
            })
            .unwrap()
    }

    pub fn log(&self, record: &AppRecord) {
        let inner_record = record.record.as_ref().unwrap();
        let log = {
            let line = match inner_record.line() {
                Some(l) => l.to_string(),
                None => String::new(),
            };
            let description = format!("{line} {}", inner_record.args());
            tracing_log::log::log!(inner_record.level(), "{line} : {}", inner_record.args());
            ActionLogger {
                command: ActionCommand::default(),
                entity: Some(entities::log::ActiveModel {
                    user: Set(record.user.username.to_owned()),
                    description: Set(description),
                    created_at: Set(chrono::Utc::now().naive_utc()),
                    level: Set(inner_record.level().to_string()),
                    ..Default::default()
                }),
            }
        };
        match self.sender.try_send(log) {
            Ok(_) => {}
            Err(e) => {
                error!("{}", e);
            }
        }
    }
}

impl Drop for AppLogger {
    fn drop(&mut self) {
        self.terminate();
    }
}

async fn flush(messages: &mut Vec<ActionLogger>, db: &DatabaseConnection) -> bool {
    let mut exit = false;
    for message in &mut *messages {
        exit = exit && message.command == ActionCommand::Exit;
        let entity = match message.entity.as_ref() {
            Some(e) => e,
            None => continue,
        };
        entity.clone().insert(db).await.unwrap();
    }
    messages.clear();
    exit
}
