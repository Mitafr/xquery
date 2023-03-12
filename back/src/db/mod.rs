use async_trait::async_trait;
use axum::extract::FromRef;
use chrono::NaiveDate;
use sea_orm::sea_query::Iden;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DatabaseTransaction, EntityTrait, IntoActiveModel,
    Iterable, Value,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::fmt::Write;
use tokio::sync::MutexGuard;

use crate::AppState;

pub mod entity;

/// Initialize a Postgres DB Pool
pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    let options = ConnectOptions::new(dotenvy::var("DB").unwrap());
    tracing::info!("Trying to connect to db...");
    match Database::connect(options).await {
        Ok(db) => {
            tracing::info!(
                "Successfully connected to database {}",
                dotenvy::var("DB").unwrap()
            );
            Ok(db)
        }
        Err(e) => Err(e),
    }
}

/// `UPPER(*)` SQL operator
struct SqlUpper;

impl Iden for SqlUpper {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "UPPER").unwrap();
    }
}

/// `GROUP_CONCAT()` SQL operator
struct GroupConcat;

impl Iden for GroupConcat {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "GROUP_CONCAT").unwrap();
    }
}

/// Represents an Ofac Ref (i.e. referential) entity that can be loaded from xml document
#[async_trait]
pub trait OfacRefEntity<T: std::marker::Sync, R, M> {
    async fn from_ofac_document(
        entity: &T,
        in_db: &[M],
        tx: &MutexGuard<DatabaseTransaction>,
    ) -> Result<Option<R>, DbErr>;
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

/// Represents an Ofac Relation entity between Many to Many relation
pub trait OfacRelEntity {
    fn generate(lhs: i32, rhs: i32) -> Self
    where
        Self: ActiveModelBehavior + IntoActiveModel<Self>,
    {
        let mut columns =
            <<Self as sea_orm::ActiveModelTrait>::Entity as sea_orm::EntityTrait>::Column::iter();
        let mut model = <Self as ActiveModelTrait>::default();
        model.set(columns.next().unwrap(), Value::Int(Some(lhs)));
        model.set(columns.next().unwrap(), Value::Int(Some(rhs)));
        model
    }
}

pub async fn get_last_issued_date(db: &DatabaseConnection) -> NaiveDate {
    match entity::dateofissue::Entity::find_by_id(0)
        .one(db)
        .await
        .unwrap()
    {
        Some(date_of_issue) => date_of_issue.last_document,
        None => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
    }
}
