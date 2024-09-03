use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Unverified user")]
    UnverifiedUser,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Email already exists")]
    EmailExists,
    #[error(transparent)]
    CommonError(#[from] crate::common::error::CommonError),
    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    TokioRecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::WrongCredentials
            | AuthError::InvalidToken
            | AuthError::MissingCredentials
            | AuthError::UnverifiedUser => StatusCode::UNAUTHORIZED,
            AuthError::JwtError(ref e) => match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AuthError::EmailExists => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Html(format!("<span>{}</span>", self));
        (status, body).into_response()
    }
}
