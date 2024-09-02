use async_trait::async_trait;
use axum::extract::{FromRef, FromRequestParts, Query};
use axum::http::request::Parts;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::error::CommonError;

pub struct Pokemon {}

pub trait HasService {
    const SERVICE: Service;
}

// `#[derive(FromRef)]` makes them sub states so they can be extracted independently
#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
// sql type
#[sqlx(type_name = "service")]
pub enum Service {
    #[serde(rename = "pokemon")]
    #[sqlx(rename = "pokemon")]
    Pokemon,
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Service::Pokemon => write!(f, "pokemon"),
        }
    }
}

impl std::str::FromStr for Service {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pokemon" => Ok(Service::Pokemon),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryName {
    pub name: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for QueryName
where
    S: Send + Sync,
{
    type Rejection = CommonError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let query = Query::<QueryName>::from_request_parts(parts, state)
            .await
            .map_err(|_| CommonError::ValidationError("Cannot get query param".into()))?;

        let name = query.name.clone();
        if name.len() < 2 {
            return Err(CommonError::ValidationError(
                "Name must be at least 2 characters long".into(),
            ));
        }
        Ok(Self { name })
    }
}
