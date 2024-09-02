use super::entity::Item;
use crate::catalog::service::CatalogService;
use crate::catalog::service::HasCatalogService;
use crate::common::entity::Pokemon;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
impl HasCatalogService for CatalogService<Pokemon> {
    type CreateItem = Item;
    type Item = Item;

    async fn query_items(_pool: &PgPool) -> Result<Vec<Self::Item>, sqlx::Error> {
        unimplemented!();
    }

    async fn query_items_by_name(
        _pool: &PgPool,
        _name: &str,
    ) -> Result<Vec<Self::Item>, sqlx::Error> {
        unimplemented!();
    }

    async fn query_item(_pool: &PgPool, _id: uuid::Uuid) -> Result<Self::Item, sqlx::Error> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::db::postgres::get_postgres_pool;

    #[tokio::test]
    async fn get_professionals() {
        let pool = get_postgres_pool().await;
        let professionals = CatalogService::<Pokemon>::query_items(pool)
            .await
            .expect("Failed to get firms");
        println!("{:?}", professionals);
    }
}
