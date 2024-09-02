use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::PgPool;

pub struct CatalogService<T> {
    _service: std::marker::PhantomData<T>,
}

#[async_trait]
pub trait HasCatalogService: 'static {
    // Send is required for async future to be pass around

    type CreateItem: Send + DeserializeOwned;
    type Item: Send + DeserializeOwned + Serialize;

    #[allow(unused_variables)]
    async fn insert_professional(pool: &PgPool, firm: Self::CreateItem) -> Result<(), sqlx::Error> {
        Err(sqlx::Error::RowNotFound)
    }
    #[allow(unused_variables)]
    async fn query_items(pool: &PgPool) -> Result<Vec<Self::Item>, sqlx::Error> {
        Err(sqlx::Error::RowNotFound)
    }
    #[allow(unused_variables)]
    async fn query_items_by_name(
        pool: &PgPool,
        name: &str,
    ) -> Result<Vec<Self::Item>, sqlx::Error> {
        Err(sqlx::Error::RowNotFound)
    }
    #[allow(unused_variables)]
    async fn query_professionals_by_ids(
        pool: &PgPool,
        ids: &[uuid::Uuid],
    ) -> Result<Vec<Self::Item>, sqlx::Error> {
        Err(sqlx::Error::RowNotFound)
    }
    #[allow(unused_variables)]
    async fn query_item(pool: &PgPool, id: uuid::Uuid) -> Result<Self::Item, sqlx::Error> {
        Err(sqlx::Error::RowNotFound)
    }
}
