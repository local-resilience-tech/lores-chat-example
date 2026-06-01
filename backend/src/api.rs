use axum::{extract::State, http::StatusCode, Json};

use crate::AppState;

pub async fn list_regions(State(state): State<AppState>) -> Result<Json<Vec<String>>, StatusCode> {
    let mut panda = state.panda.lock().await;
    let region_ids = panda
        .list_regions()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let hex_ids: Vec<String> = region_ids
        .iter()
        .map(|id| id.iter().map(|b| format!("{b:02x}")).collect())
        .collect();

    Ok(Json(hex_ids))
}
