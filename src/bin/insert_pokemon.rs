use myapp::common::db::postgres::get_postgres_pool;
use myapp::common::entity::Pagination;
use myapp::etl::{Pokemon, Scraping};

#[tokio::main]
async fn main() {
    // let configuration = get_configuration().expect("Failed to read configuration.");
    let pool = get_postgres_pool().await;
    if !Pokemon::has_table(pool).await {
        panic!("Table does not exist");
    }
    let pagination = Pagination {
        page: 0,
        page_size: 20000,
    };
    let links = Pokemon::get_scrap_links(pagination).await;
    dbg!("Links to scrap:", &links.len());
    for link in links {
        dbg!("Scraping data from: {}", &link);
        let data = Pokemon::extract_data(link).await;
        let transformed_data = Pokemon::transform_data(data).await;
        Pokemon::load_data(pool, transformed_data).await;
    }
}
