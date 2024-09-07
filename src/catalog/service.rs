use crate::common::entity::Pagination;
use async_trait::async_trait;
use axum::extract::Query;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::PgPool;

pub struct CatalogService<T> {
    _service: std::marker::PhantomData<T>,
}

#[async_trait]
pub trait HasCatalogService: 'static {
    // Send is required for async future to be pass around
    type Item: Send + DeserializeOwned + Serialize;

    async fn query_items_count(pool: &PgPool) -> Result<usize, sqlx::Error>;

    async fn query_items(
        pool: &PgPool,
        pagination: Query<Pagination>,
    ) -> Result<Vec<Self::Item>, sqlx::Error>;

    async fn query_items_by_name(pool: &PgPool, name: &str)
        -> Result<Vec<Self::Item>, sqlx::Error>;

    async fn query_item(pool: &PgPool, id: u32) -> Result<Self::Item, sqlx::Error>;

    async fn query_item_by_name(pool: &PgPool, name: String) -> Result<Self::Item, sqlx::Error>;
}
