use crate::catalog::error::CatalogError;
use crate::catalog::service::{CatalogService, HasCatalogService};
use crate::common::entity::Pokemon;
use crate::common::entity::{AppState, QueryName};
use anyhow::Context;
use async_trait::async_trait;
use axum::{
    extract::{Json, Path, State},
    routing::{get, put},
    Router,
};
use sqlx::PgPool;

pub struct CatalogHandlers<T> {
    _service: std::marker::PhantomData<T>,
}

impl HasCatalogHandlers for CatalogHandlers<Pokemon> {
    type Service = CatalogService<Pokemon>;
}

#[async_trait]
pub trait HasCatalogHandlers: 'static + Send + Sync {
    type Service: HasCatalogService + Send;

    fn create_router() -> Router<AppState> {
        Router::new()
            .route("/professionals", get(Self::show_professionals))
            .route("/professionals/:id", put(Self::show_professional))
    }

    async fn show_professionals(
        State(pool): State<PgPool>,
        q_name: Option<QueryName>,
    ) -> Result<Json<Vec<<Self::Service as HasCatalogService>::Item>>, CatalogError> {
        let professionals = if let Some(QueryName { name }) = q_name {
            Self::Service::query_items_by_name(&pool, &name)
                .await
                .context("Failed to get professionals")?
        } else {
            Self::Service::query_items(&pool)
                .await
                .context("Failed to get professionals")?
        };
        Ok(Json(professionals))
    }

    async fn show_professional(
        State(pool): State<PgPool>,
        Path(id): Path<uuid::Uuid>,
    ) -> Result<Json<<Self::Service as HasCatalogService>::Item>, CatalogError> {
        let professional = Self::Service::query_item(&pool, id)
            .await
            .context("Failed to get professional")?;
        Ok(Json(professional))
    }
}
