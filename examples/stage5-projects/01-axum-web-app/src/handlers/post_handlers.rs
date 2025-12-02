use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::post::{
    CreatePostRequest, UpdatePostRequest, PostResponse, PostWithAuthor
};
use crate::error::{AppError, Result};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default)]
    pub offset: i64,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,

    #[serde(default = "default_limit")]
    pub limit: i64,

    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    20
}

pub async fn create_post(
    State(state): State<AppState>,
    claims: axum::extract::Extension<crate::models::user::Claims>,
    Json(request): Json<CreatePostRequest>,
    ) -> Result<(StatusCode, Json<PostResponse>)> {
    let author_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    let post = state.post_service.create_post(&author_id, request).await?;
    Ok((StatusCode::CREATED, Json(post)))
}

pub async fn get_post(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<PostWithAuthor>> {
    let post = state.post_service.get_post(&post_id).await?;
    Ok(Json(post))
}

pub async fn get_posts(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<PostWithAuthor>>> {
    let posts = state.post_service.get_posts(pagination.limit, pagination.offset).await?;
    Ok(Json(posts))
}

pub async fn get_user_posts(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<Vec<PostWithAuthor>>> {
    let posts = state.post_service.get_user_posts(&user_id, pagination.limit, pagination.offset).await?;
    Ok(Json(posts))
}

pub async fn update_post(
    State(state): State<AppState>,
    claims: axum::extract::Extension<crate::models::user::Claims>,
    Path(post_id): Path<Uuid>,
    Json(request): Json<UpdatePostRequest>,
) -> Result<Json<PostResponse>> {
    let author_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

    let post = state.post_service.update_post(&post_id, &author_id, request).await?;
    Ok(Json(post))
}

pub async fn delete_post(
    State(state): State<AppState>,
    claims: axum::extract::Extension<crate::models::user::Claims>,
    Path(post_id): Path<Uuid>,
    ) -> Result<StatusCode> {
        let author_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

        state.post_service.delete_post(&post_id, &author_id).await?;
        Ok(StatusCode::NO_CONTENT)
    }

pub async fn search_posts(
    State(state): State<AppState>,
    Query(search_params): Query<SearchQuery>,
    ) -> Result<Json<Vec<PostWithAuthor>>> {
        let posts = state.post_service.search_posts(&search_params.q, search_params.limit, search_params.offset).await?;
        Ok(Json(posts))
    }

pub async fn get_current_user_posts(
    State(state): State<AppState>,
    claims: axum::extract::Extension<crate::models::user::Claims>,
    Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<Vec<PostWithAuthor>>> {
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?;

        let posts = state.post_service.get_user_posts(&user_id, pagination.limit, pagination.offset).await?;
        Ok(Json(posts))
    }
