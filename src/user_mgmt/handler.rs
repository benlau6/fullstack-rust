use super::encryption::hash;
use super::entity::{CreateUser, User};
use super::error::AuthError;
use axum::extract::{Json, State};
use axum::Form;
use axum_htmx::HxRedirect;
use sqlx::PgConnection;
use sqlx::PgPool;

pub async fn create_user(
    State(pool): State<PgPool>,
    Form(user): Form<CreateUser>,
) -> Result<(HxRedirect, ()), AuthError> {
    let mut tx = pool.begin().await?;

    insert_user(&mut tx, user)
        .await
        .inspect_err(|e| tracing::error!("Failed to create user: {e}"))?;
    tx.commit().await?;

    Ok((HxRedirect("/login".parse().unwrap()), ()))
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
    .inspect_err(|e| tracing::error!("Failed to insert user: {e}"))
    .map_err(|e| match e {
        sqlx::Error::Database(ref e) if e.constraint() == Some("users_email_key") => {
            AuthError::EmailExists
        }
        _ => AuthError::DatabaseError(e),
    })?;

    Ok(row.id)
}
