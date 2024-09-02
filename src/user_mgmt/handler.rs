use super::encryption::hash;
use super::entity::{CreateUser, User};
use super::error::AuthError;
use axum::extract::{Json, State};
use sqlx::PgConnection;
use sqlx::PgPool;

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(user): Json<CreateUser>,
) -> Result<(), AuthError> {
    let mut tx = pool.begin().await?;

    insert_user(&mut tx, user)
        .await
        .inspect_err(|e| tracing::error!("Failed to create user: {e}"))?;
    tx.commit().await?;

    Ok(())
}

pub async fn query_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT users.id, users.name, users.email, 
            users.is_active, users.is_verified, users.is_superuser
            FROM users"#
    )
    .fetch_all(pool)
    .await
}

pub async fn query_user(pool: &PgPool, user_id: uuid::Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"SELECT users.id, users.name, users.email, 
            users.is_active, users.is_verified, users.is_superuser
            FROM users
            WHERE users.id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn show_users(State(pool): State<PgPool>) -> Result<Json<Vec<User>>, AuthError> {
    let users = query_users(&pool).await?;

    Ok(Json(users))
}

pub async fn insert_user(
    tx: &mut PgConnection,
    payload: CreateUser,
) -> Result<uuid::Uuid, AuthError> {
    let hashed_password = hash(payload.password).await?;

    let row = sqlx::query!(
        "INSERT INTO users (email, hashed_password, name) VALUES ($1, $2, $3) RETURNING id",
        payload.email,
        hashed_password,
        payload.name,
    )
    .fetch_one(tx)
    .await
    .inspect_err(|e| tracing::error!("Failed to insert user: {e}"))?;

    Ok(row.id)
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
}
