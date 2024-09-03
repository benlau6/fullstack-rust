use crate::configuration::{get_environment, Environment};

use super::encryption::verify;
use super::error::AuthError;
use super::handler::query_user;
use super::jwt::{decode, encode};
pub use super::jwt::{Claims, Role};
use axum::Form;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
    Json, RequestPartsExt,
};
use axum_extra::{
    extract::cookie::{Cookie, CookieJar, SameSite},
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use cookie::time::Duration;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub async fn me_handler(user: CurrentUser) -> Result<String, AuthError> {
    Ok(user.to_string())
}

// TODO: should it be in auth.rs? or handler.rs? messy code separation
// tracing::instrument is a wrapper
// it shows only if there are logs inside.
#[tracing::instrument(name="Logging in", skip(jar, pool, payload), fields(username = %payload.email))]
pub async fn login(
    jar: CookieJar,
    State(pool): State<PgPool>,
    // Json must be placed at the end of the parameters
    Form(payload): Form<AuthPayload>,
    // Json must be placed at the end of the Result tuple
) -> Result<(CookieJar, Json<AuthBody>), AuthError> {
    // Check if the user sent the credentials
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    // debug payload
    let (user_id, role) = validate_user(&pool, payload).await?;
    // Validated, now create jwt claims
    let claims = Claims::new(user_id, role);
    // Create the authorization token
    let token = encode(&claims).map_err(|_| AuthError::TokenCreation)?;

    // check env for local client to bypass the secure flag, cuz we don't need https on localhost
    let env = get_environment();
    // Create a http_only cookie to store the token
    let cookie = Cookie::build(("access_token", token.clone()))
        .http_only(true)
        .secure(env != Environment::Local)
        .same_site(SameSite::None)
        .max_age(Duration::hours(1))
        .path("/")
        .build();
    // Store and Send the authorized token
    Ok((jar.add(cookie), Json(AuthBody::new(token))))
}

pub async fn logout(jar: CookieJar) -> CookieJar {
    jar.remove(Cookie::from("access_token"))
}

async fn validate_user(
    pool: &PgPool,
    credentials: AuthPayload,
) -> Result<(uuid::Uuid, Role), AuthError> {
    let user = sqlx::query_as!(
        UserToClaim,
        "SELECT id, hashed_password, is_superuser, is_verified from users
        WHERE email = $1
        ",
        credentials.email
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AuthError::WrongCredentials)?;

    let role = if user.is_superuser {
        Role::Admin
    } else {
        Role::User
    };

    if !verify(credentials.password, user.hashed_password).await? {
        return Err(AuthError::WrongCredentials);
    }

    if !user.is_verified {
        return Err(AuthError::UnverifiedUser);
    }

    Ok((user.id, role))
}

// Extract the Claim from a request body
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // The token might come from a cookie or from the authorization header
        // Note: It is invalid if the token from cookie is correct
        // but the token from the header is not
        let token = if let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        {
            // Extract the token from the authorization header
            bearer.token().to_string()
        } else if let Ok(jar) = parts.extract::<CookieJar>().await {
            // Extract the token from the cookie
            jar.get("access_token")
                .ok_or(AuthError::MissingCredentials)?
                .value()
                .to_string()
        } else {
            return Err(AuthError::MissingCredentials);
        };

        let claims = decode(&token)?.claims;
        Ok(claims)
    }
}

// Extract the Claim from a request body
#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
    // From AppState, which implements FromRef
    PgPool: FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let claims = Claims::from_request_parts(parts, state).await?;
        let pool = PgPool::from_ref(state);

        let user = query_user(&pool, claims.sub)
            .await
            .inspect_err(|e| tracing::error!("Failed to query current user from jwt: {e}"))?;

        let current_user = Self {
            id: user.id,
            name: user.name,
            email: user.email,
            role: claims.role,
        };

        Ok(current_user)
    }
}

#[derive(Debug, Serialize)]
pub struct CurrentUser {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub role: Role,
}

impl std::fmt::Display for CurrentUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: {},\n Email: {}", self.name, self.email)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    #[allow(dead_code)]
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct UserToClaim {
    id: uuid::Uuid,
    hashed_password: String,
    is_superuser: bool,
    #[allow(dead_code)]
    is_verified: bool,
}

pub async fn get_current_user_from_id(pool: &PgPool, user_id: &uuid::Uuid) -> CurrentUser {
    let user = query_user(pool, *user_id)
        .await
        .inspect_err(|e| tracing::error!("Failed to query current user from jwt: {e}"))
        .expect("Failed to query current user");

    let role = if user.is_superuser {
        Role::Admin
    } else {
        Role::User
    };

    CurrentUser {
        id: user.id,
        name: user.name,
        email: user.email,
        role,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::db::postgres::get_postgres_pool;

    #[tokio::test]
    async fn test_validate_user_from_db() {
        let pool = get_postgres_pool().await;
        let payload = AuthPayload {
            email: "admin@example.com".to_string(),
            password: "password".to_string(),
        };
        let result = validate_user(pool, payload).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_authorize() {
        // https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs
        todo!("write the test to test the authorize function")
    }
}
