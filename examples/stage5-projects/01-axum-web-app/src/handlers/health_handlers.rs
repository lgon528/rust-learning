use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde_json::json;

use crate::error::Result;
use crate::state::AppState;

pub async fn health_check(State(state): State<AppState>) -> Result<Json<serde_json::Value>> {
    // Test database connection
    let db = state.user_service.get_db();
    sqlx::query("SELECT 1")
        .fetch_one(db)
        .await?;

    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "axum-web-app",
        "version": "0.1.0"
    })))
}

pub async fn readiness_check(State(state): State<AppState>) -> Result<Json<serde_json::Value>> {
    // Check if database is ready
    let db = state.user_service.get_db();
    let _ = sqlx::query("SELECT 1")
        .fetch_one(db)
        .await?;

    Ok(Json(json!({
        "status": "ready",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn liveness_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "alive",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
