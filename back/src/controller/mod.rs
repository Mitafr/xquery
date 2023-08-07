use async_session::log::RecordBuilder;
use axum::{
    extract::{Query, State},
    Json,
};
use entities::log::Model as LogModel;
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};
use serde::{Deserialize, Serialize};
use tracing_log::log::Log;

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct LogResult {
    results: Vec<LogModel>,
    total: usize,
}

#[derive(Deserialize)]
pub struct PaginatorParams {
    pub page: u64,
}

#[axum::debug_handler]
pub async fn get_logs(
    Query(params): Query<PaginatorParams>,
    State(state): State<AppState>,
) -> Json<LogResult> {
    let page = params.page;
    state.logger.log(
        &RecordBuilder::new()
            .level(tracing_log::log::Level::Debug)
            .args(format_args!("get_log {}", page))
            .build(),
    );
    let count = entities::log::Entity::find()
        .count(&state.db)
        .await
        .unwrap() as usize;
    tracing::debug!("{}", count);
    Json(LogResult {
        results: entities::log::Entity::find()
            .order_by_desc(entities::log::Column::CreatedAt)
            .paginate(&state.db, 20)
            .fetch_page(page)
            .await
            .unwrap(),
        total: count,
    })
}
