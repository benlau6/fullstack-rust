use crate::configuration::get_configuration;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::OnceCell;

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_postgres_pool() -> &'static PgPool {
    POOL.get_or_init(|| async {
        let configuration = get_configuration().expect("Failed to read configuration.");
        PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(configuration.database.with_db())
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Connection;

    #[sqlx::test]
    async fn test_postgres_connection(pool: PgPool) {
        let mut conn = pool.acquire().await.expect("Failed to acquire connection");
        let result = conn.ping().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_postgres_pool() {
        let pool = get_postgres_pool().await;
        let mut conn = pool.acquire().await.expect("Failed to acquire connection");
        let result = conn.ping().await;

        assert!(result.is_ok());
    }
}
