use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

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
