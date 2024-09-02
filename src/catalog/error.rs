use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("Resource not found")]
    NotFound,
    #[error("Resource not ready")]
    NotImplemented,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for CatalogError {
    fn into_response(self) -> Response {
        let status = match self {
            CatalogError::NotFound | CatalogError::NotImplemented => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(json!({
            "message": self.to_string(),
        }));
        (status, body).into_response()
    }
}
