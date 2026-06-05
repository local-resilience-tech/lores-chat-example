use axum::{extract::State, http::StatusCode, Json};

use crate::AppState;

pub async fn list_regions(State(state): State<AppState>) -> Result<Json<Vec<String>>, StatusCode> {
    println!("Listing regions...");
    let mut panda = state.panda.lock().await;
    let region_ids = panda
        .list_regions()
        .await
        .map_err(|e| {
            println!("Error listing regions: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .into_inner()
        .region_ids;

    let hex_ids: Vec<String> = region_ids
        .iter()
        .map(|id| id.iter().map(|b| format!("{b:02x}")).collect())
        .collect();

    println!("Found regions: {:?}", hex_ids);

    Ok(Json(hex_ids))
}
