use super::entity::Monster;
use crate::catalog::service::CatalogService;
use crate::catalog::service::HasCatalogService;
use crate::common::entity::{Pagination, Pokemon};
use async_trait::async_trait;
use axum::extract::Query;
use sqlx::PgPool;

#[async_trait]
impl HasCatalogService for CatalogService<Pokemon> {
    type Item = Monster;

    async fn query_items_count(pool: &PgPool) -> Result<usize, sqlx::Error> {
        sqlx::query!("SELECT COUNT(*) FROM pokemon")
            .fetch_one(pool)
            .await
            .map(|row| row.count)
            .inspect_err(|e| tracing::error!("Failed to fetch count: {}", e))
            .map(|count| count.unwrap_or(0) as usize)
    }

    async fn query_items(
        pool: &PgPool,
        pagination: Query<Pagination>,
    ) -> Result<Vec<Self::Item>, sqlx::Error> {
        sqlx::query_as!(
            Monster,
            r#"
            SELECT id, name, height, weight, types, image_url, image_url_game_front, image_url_game_back, image_url_game_front_shiny, image_url_game_back_shiny
            FROM pokemon
            ORDER BY id
            LIMIT $1
            OFFSET $2
            "#,
            pagination.page_size as i64,
            pagination.offset() as i64
        ).fetch_all(pool)
            .await
            .inspect_err(|e| tracing::error!("Failed to fetch monsters: {}", e))
    }

    #[allow(unused_variables)]
    async fn query_item(pool: &PgPool, id: u32) -> Result<Self::Item, sqlx::Error> {
        sqlx::query_as!(
            Monster,
            r#"
            SELECT id, name, height, weight, types, image_url, image_url_game_front, image_url_game_back, image_url_game_front_shiny, image_url_game_back_shiny
            FROM pokemon
            WHERE id = $1
            "#,
            id as i64
        ).fetch_one(pool)
            .await
            .inspect_err(|e| tracing::error!("Failed to fetch monsters: {}", e))
    }

    #[allow(unused_variables)]
    async fn query_item_by_name(pool: &PgPool, name: String) -> Result<Self::Item, sqlx::Error> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    async fn query_items_by_name(
        pool: &PgPool,
        name: &str,
    ) -> Result<Vec<Self::Item>, sqlx::Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::db::postgres::get_postgres_pool;

    #[tokio::test]
    async fn get_pokemon_count() {
        let pool = get_postgres_pool().await;
        let has_count = CatalogService::<Pokemon>::query_items_count(pool)
            .await
            .is_ok();

        assert!(has_count);
    }
}
