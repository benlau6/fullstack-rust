use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommonError {
    // ref: zero2prod 8.4.2 anyhow Or thiserror?
    #[error("Resource not found")]
    NotFound,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for CommonError {
    fn into_response(self) -> Response {
        let status = match self {
            CommonError::NotFound => StatusCode::NOT_FOUND,
            CommonError::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(json!({
            "message": self.to_string(),
        }));
        (status, body).into_response()
    }
}

// Make our own error that wraps `anyhow::Error`.
pub struct AnyError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AnyError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AnyError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
