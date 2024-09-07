use crate::catalog::error::CatalogError;
use crate::catalog::service::HasCatalogService;
use crate::common::entity::{AppState, Pagination, QueryName};
use askama_axum::Template;
use async_trait::async_trait;
use axum::extract::Query;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use sqlx::PgPool;

pub struct CatalogPages<T> {
    _service: std::marker::PhantomData<T>,
}

#[async_trait]
pub trait HasCatalogPages: 'static + Send + Sync {
    type Service: HasCatalogService + Send;
    type ItemsPage: Template + IntoResponse;
    type ItemPage: Template + IntoResponse;

    fn create_router() -> Router<AppState> {
        Router::new()
            .route("/", get(Self::show_items))
            .route("/:id", get(Self::show_item))
    }

    async fn show_items(
        State(pool): State<PgPool>,
        q_name: Option<QueryName>,
        pagination: Option<Query<Pagination>>,
    ) -> Result<Self::ItemsPage, CatalogError>;

    async fn show_item(
        State(pool): State<PgPool>,
        Path(id): Path<u32>,
    ) -> Result<Self::ItemPage, CatalogError>;
}
