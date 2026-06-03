use crate::error::ApiError;
use crate::models::HealthResponse;
use crate::state::AppState;
use axum::{extract::State, Json};

pub async fn health_handler(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, ApiError> {
    // Check storage backend health
    let redis_status = match state.store.health_check().await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    // Get last update date
    let last_update = state.store.get_last_update_date().await.ok().flatten();

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        redis: redis_status.to_string(),
        last_update,
    }))
}
