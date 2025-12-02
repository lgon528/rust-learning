use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;

use crate::services::UserService;
use crate::models::user::{
    CreateUserRequest, UpdateUserRequest, LoginRequest, UserResponse, LoginResponse
};
use crate::error::{AppError, Result};
use crate::state::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>)> {
    let user = state.user_service.create_user(request).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let login_response = state.user_service.login(request).await?;
    Ok(Json(login_response))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserResponse>> {
    let user = state.user_service.get_user(&user_id).await?;
    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
    ) -> Result<Json<UserResponse>> {
    let user = state.user_service.update_user(&user_id, request).await?;
    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    ) -> Result<StatusCode> {
    state.user_service.delete_user(&user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_current_user(
    State(state): State<AppState>,
    // This will be populated by the auth middleware
    claims: axum::extract::Extension<crate::models::user::Claims>,
    ) -> Result<Json<UserResponse>> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    let user = state.user_service.get_user(&user_id).await?;
    Ok(Json(user))
}

pub async fn update_current_user(
    State(state): State<AppState>,
    claims: axum::extract::Extension<crate::models::user::Claims>,
    Json(request): Json<UpdateUserRequest>,
    ) -> Result<Json<UserResponse>> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    let user = state.user_service.update_user(&user_id, request).await?;
    Ok(Json(user))
}

pub async fn delete_current_user(
    State(state): State<AppState>,
    claims: axum::extract::Extension<crate::models::user::Claims>,
    ) -> Result<StatusCode> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    state.user_service.delete_user(&user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
