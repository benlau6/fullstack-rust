use crate::common::entity::AppState;
use crate::user_mgmt::auth::CurrentUser;
use crate::user_mgmt::error::AuthError;
use askama_axum::Template;
use axum::routing::get;
use axum::Router;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn hello_world() -> HelloTemplate<'static> {
    HelloTemplate { name: "world" }
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate;

#[derive(Template)]
#[template(path = "me.html")]
struct MeTemplate {
    pub name: String,
    pub email: String,
    pub role: String,
}

async fn me_page(user: CurrentUser) -> Result<MeTemplate, AuthError> {
    Ok(MeTemplate {
        name: user.name.clone(),
        email: user.email.clone(),
        role: user.role.to_string(),
    })
}

pub fn create_frontend_router() -> Router<AppState> {
    Router::new()
        .route("/", get(hello_world))
        .route("/login", get(|| async { LoginTemplate }))
        .route("/register", get(|| async { RegisterTemplate }))
        .route("/me", get(me_page))
}
