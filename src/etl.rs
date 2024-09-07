use crate::catalog::pokemon::entity::ApiResponse;
use crate::catalog::pokemon::entity::Monster;
use crate::catalog::pokemon::entity::MonsterFromApi;
use crate::common::entity::Pagination;
use async_trait::async_trait;
use reqwest;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sqlx::PgPool;

pub struct Pokemon;

#[async_trait]
pub trait Scraping {
    type ApiResponse;
    type FromApi: DeserializeOwned;
    type ToDb: Serialize;

    async fn has_table(pool: &PgPool) -> bool;
    async fn get_scrap_links(pagination: Pagination) -> Vec<String>;
    async fn extract_data(link: impl IntoUrl + Send) -> Self::FromApi;
    async fn transform_data(data: Self::FromApi) -> Self::ToDb;
    async fn load_data(pool: &PgPool, data: Self::ToDb);
}

#[async_trait]
impl Scraping for Pokemon {
    type ApiResponse = ApiResponse;
    type FromApi = MonsterFromApi;
    type ToDb = Monster;

    async fn has_table(pool: &PgPool) -> bool {
        sqlx::query("SELECT 1 FROM pokemon LIMIT 1")
            .fetch_optional(pool)
            .await
            .is_ok()
    }

    async fn get_scrap_links(pagination: Pagination) -> Vec<String> {
        let url = format!(
            "https://pokeapi.co/api/v2/pokemon?limit={}&offset={}",
            pagination.limit(),
            pagination.offset()
        );
        let response = reqwest::get(&url)
            .await
            .expect("Failed to send links request")
            .json::<Self::ApiResponse>()
            .await
            .expect("Failed to parse links response");

        response
            .results
            .iter()
            .map(|item| item.url.clone())
            .collect::<Vec<String>>()
    }

    async fn extract_data(link: impl IntoUrl + Send) -> Self::FromApi {
        reqwest::get(link)
            .await
            .expect("Failed to send item request")
            .json::<Self::FromApi>()
            .await
            .expect("Failed to parse item response")
    }

    async fn transform_data(data: Self::FromApi) -> Self::ToDb {
        data.into()
    }

    async fn load_data(pool: &PgPool, data: Self::ToDb) {
        sqlx::query!(
            r#"
            INSERT INTO pokemon (id, name, height, weight, types, image_url, image_url_game_front, image_url_game_back, image_url_game_front_shiny, image_url_game_back_shiny)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE
                SET name = EXCLUDED.name,
                    height = EXCLUDED.height,
                    weight = EXCLUDED.weight,
                    types = EXCLUDED.types,
                    image_url = EXCLUDED.image_url,
                    image_url_game_front = EXCLUDED.image_url_game_front,
                    image_url_game_back = EXCLUDED.image_url_game_back,
                    image_url_game_front_shiny = EXCLUDED.image_url_game_front_shiny,
                    image_url_game_back_shiny = EXCLUDED.image_url_game_back_shiny
            "#,
            data.id,
            data.name,
            data.height,
            data.weight,
            &data.types,
            data.image_url,
            data.image_url_game_front,
            data.image_url_game_back,
            data.image_url_game_front_shiny,
            data.image_url_game_back_shiny
        ).execute(pool).await.expect("Failed to insert data");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_links() {
        let pagination = Pagination {
            page: 0,
            page_size: 5,
        };
        let links = Pokemon::get_scrap_links(pagination).await;
        assert_eq!(links.len(), 5);
    }
}
