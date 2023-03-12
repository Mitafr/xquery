use axum::{extract::State, Json};
use chrono::NaiveDate;

use crate::{db::get_last_issued_date, AppState};

pub async fn issued_date_get(State(state): State<AppState>) -> Json<NaiveDate> {
    Json(get_last_issued_date(&state.db).await)
}
