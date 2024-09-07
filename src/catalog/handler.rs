use crate::catalog::error::CatalogError;
use crate::catalog::service::{CatalogService, HasCatalogService};
use crate::common::entity::Pagination;
use crate::common::entity::Pokemon;
use crate::common::entity::{AppState, QueryName};
use anyhow::Context;
use async_trait::async_trait;
use axum::extract::Query;
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
            .route("/items", get(Self::show_items))
            .route("/items/:id", put(Self::show_item))
    }

    async fn show_items(
        State(pool): State<PgPool>,
        q_name: Option<QueryName>,
        pagination: Option<Query<Pagination>>,
    ) -> Result<Json<Vec<<Self::Service as HasCatalogService>::Item>>, CatalogError> {
        let items = if let Some(QueryName { name }) = q_name {
            Self::Service::query_items_by_name(&pool, &name)
                .await
                .context("Failed to get items")?
        } else {
            Self::Service::query_items(&pool, pagination.unwrap_or_default())
                .await
                .context("Failed to get items")?
        };
        Ok(Json(items))
    }

    async fn show_item(
        State(pool): State<PgPool>,
        Path(id): Path<u32>,
    ) -> Result<Json<<Self::Service as HasCatalogService>::Item>, CatalogError> {
        let item = Self::Service::query_item(&pool, id)
            .await
            .context("Failed to get item")?;
        Ok(Json(item))
    }
}
