use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use axum::http::{Method, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use myapp::frontend::create_frontend_router;
use myapp::{
    catalog::handler::{CatalogHandlers, HasCatalogHandlers},
    common::{
        db::postgres::get_postgres_pool,
        entity::{AppState, Pokemon, Service},
    },
    configuration::get_configuration,
    user_mgmt::{
        auth::{login, logout, me_handler},
        handler::{create_user, show_users},
    },
};
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tower_livereload::LiveReloadLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[shuttle_runtime::main]
async fn shuttle_main() -> shuttle_axum::ShuttleAxum {
    let app = create_main_router().await;
    Ok(app.into())
}

/// This is the original main function before the shuttle runtime and deployment was added.
/// It is kept here for reference.
/// Remember to use it with #[tokie::main] instead of #[shuttle_runtime::main]
#[allow(dead_code)]
async fn local_axum_without_shuttle() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let app = create_main_router().await;

    // For macos, listen to IPV4 and IPV6
    let addr_str = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let addr = addr_str.parse::<std::net::SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();

    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Cannot start the server");
}

async fn create_main_router() -> Router {
    let configuration = get_configuration().expect("Failed to read configuration.");

    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            configuration.application.rust_log,
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = get_postgres_pool().await.clone();
    let ping = sqlx::query("SELECT 1").fetch_one(&pool).await;

    match ping {
        Ok(_) => {
            tracing::info!("Connected to postgres on {}", configuration.database.host);
        }
        Err(e) => {
            tracing::error!("Failed to connect to postgres: {}", e);
            panic!("Failed to connect to postgres");
        }
    }

    // Run migrations inside the app so it can use the database url from the configuration
    // instead of the DATABASE_URL environment variable
    // and it also makes sure that the migrations are run before the app starts
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Setup app state for the entire app
    let state = AppState { pool };

    // states the origins that are allowed to make requests to the server
    // not for the app itself but for any frontend that wants to make requests to the server
    // currently it is meaningless because the frontend is served by the same server
    let origins = ["http://localhost:5173".parse().unwrap()];

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_methods([
            Method::OPTIONS,
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT])
        .allow_origin(origins)
        .allow_credentials(true);

    let user_routes = Router::new()
        .route("/", post(create_user))
        .route("/", get(show_users));

    // Note that the middleware is only applied to existing routes.
    // So you have to first add your routes (and / or fallback)
    // and then call layer afterwards.
    // Additional routes added after layer is called will not have the middleware added.

    let pokemon_handlers = create_module_router::<Pokemon>();

    let base_api_app = Router::new()
        .route("/", get(root))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/me", get(me_handler))
        .nest("/users", user_routes)
        .nest(format!("/{}", Service::Pokemon).as_str(), pokemon_handlers)
        // timeout requests after 10 secs, returning 408 status code
        .layer(TimeoutLayer::new(Duration::from_secs(20)))
        .layer(RequestBodyLimitLayer::new(4096));

    let base_frontend_app = create_frontend_router();

    Router::new()
        // serve the file in the "assets" directory under `/assets`
        .nest("/", base_frontend_app)
        .nest("/api/v1", base_api_app)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/favicon.ico", ServeFile::new("assets/favicon.ico"))
        .fallback(fallback) // it must be placed before the live reload layer
        .layer(LiveReloadLayer::new())
}

fn create_module_router<M>() -> Router<AppState>
where
    CatalogHandlers<M>: HasCatalogHandlers,
{
    Router::new().merge(CatalogHandlers::<M>::create_router())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    let message = format!("No route for {}", uri);
    tracing::debug!(message);
    (StatusCode::NOT_FOUND, message)
}
