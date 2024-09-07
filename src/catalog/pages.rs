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

pub mod pokemon {
    use crate::catalog::error::CatalogError;
    use crate::catalog::pages::CatalogPages;
    use crate::catalog::pages::HasCatalogPages;
    use crate::catalog::pokemon::entity::Monster;
    use crate::catalog::service::{CatalogService, HasCatalogService};
    use crate::common::entity::PaginationNavigation;
    use crate::common::entity::{Pagination, Pokemon, QueryName};
    use crate::common::filters;
    use anyhow::Context;
    use askama_axum::Template;
    use async_trait::async_trait;
    use axum::extract::{Path, Query, State};
    use sqlx::PgPool;

    #[derive(Template)]
    #[template(path = "pokemon/items.html")]
    pub struct PokemonItemsTemplate {
        pub pokemon: Vec<Monster>,
        pub total_pages: usize,
        pub page_size: usize,
        pub current_page: usize,
        pub pagination: PaginationNavigation,
    }

    #[derive(Template)]
    #[template(path = "pokemon/item.html")]
    pub struct PokemonItemTemplate {
        pub pokemon: Monster,
    }

    #[async_trait]
    impl HasCatalogPages for CatalogPages<Pokemon> {
        type Service = CatalogService<Pokemon>;
        type ItemsPage = PokemonItemsTemplate;
        type ItemPage = PokemonItemTemplate;

        async fn show_items(
            State(pool): State<PgPool>,
            q_name: Option<QueryName>,
            pagination: Option<Query<Pagination>>,
        ) -> Result<Self::ItemsPage, CatalogError> {
            let count = Self::Service::query_items_count(&pool)
                .await
                .context("Failed to get items count")?;

            let pagination = pagination.unwrap_or_default();

            let items = if let Some(QueryName { name }) = q_name {
                Self::Service::query_items_by_name(&pool, &name)
                    .await
                    .context("Failed to get items")?
            } else {
                Self::Service::query_items(&pool, pagination.clone())
                    .await
                    .context("Failed to get items")?
            };

            let total_pages = pagination.get_total_pages(count);
            Ok(PokemonItemsTemplate {
                pokemon: items,
                current_page: pagination.page,
                page_size: pagination.page_size,
                total_pages,
                pagination: pagination.get_navigation(total_pages, 5),
            })
        }

        async fn show_item(
            State(pool): State<PgPool>,
            Path(id): Path<u32>,
        ) -> Result<Self::ItemPage, CatalogError> {
            let item = Self::Service::query_item(&pool, id)
                .await
                .context("Failed to get item")?;
            Ok(PokemonItemTemplate { pokemon: item })
        }
    }
}
