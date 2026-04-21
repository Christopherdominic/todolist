use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
// use thiserror::Error;

/// API error variants mapped to clear HTTP responses.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("resource not found: {0}")]
    NotFound(String),
    #[error("invalid request: {0}")]
    BadRequest(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

// Generic result alias used by handlers and services.
pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    // Converts domain errors into structured HTTP JSON responses.
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        (status, Json(ErrorBody { error: message })).into_response()
    }
}
