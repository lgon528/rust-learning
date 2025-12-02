use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Password hashing error: {0}")]
    PasswordHashing(#[from] bcrypt::BcryptError),

    #[error("UUID parsing error: {0}")]
    UuidParsing(#[from] uuid::Error),

    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Authentication(ref message) => {
                (StatusCode::UNAUTHORIZED, message.clone())
            }
            AppError::Authorization(ref message) => {
                (StatusCode::FORBIDDEN, message.clone())
            }
            AppError::Validation(ref message) => {
                (StatusCode::BAD_REQUEST, message.clone())
            }
            AppError::NotFound(ref message) => {
                (StatusCode::NOT_FOUND, message.clone())
            }
            AppError::Conflict(ref message) => {
                (StatusCode::CONFLICT, message.clone())
            }
            AppError::Internal(ref message) => {
                tracing::error!("Internal error: {}", message);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Jwt(ref err) => {
                tracing::warn!("JWT error: {:?}", err);
                (StatusCode::UNAUTHORIZED, "Invalid token".to_string())
            }
            AppError::PasswordHashing(ref err) => {
                tracing::error!("Password hashing error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::UuidParsing(_) => {
                (StatusCode::BAD_REQUEST, "Invalid ID format".to_string())
            }
            AppError::SerdeJson(ref err) => {
                tracing::error!("JSON serialization error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
