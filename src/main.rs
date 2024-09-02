use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use axum::http::{Method, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use myapp::frontend::home::hello_world;
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
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
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

    // Setup app state for the entire app
    let state = AppState { pool };

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

    let legal_handlers = create_module_router::<Pokemon>();

    let base_api_app = Router::new()
        .route("/", get(root))
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/me", get(me_handler))
        .nest("/users", user_routes)
        .nest(format!("/{}", Service::Pokemon).as_str(), legal_handlers)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        // timeout requests after 10 secs, returning 408 status code
        .layer(TimeoutLayer::new(Duration::from_secs(20)))
        .layer(RequestBodyLimitLayer::new(4096))
        .with_state(state);

    let base_frontend_app = Router::new().route("/", get(hello_world));

    let app = Router::new()
        .nest("/", base_frontend_app)
        .nest("/api/v1", base_api_app)
        .fallback(fallback);

    // For macos, listen to IPV4 and IPV6
    let addr_str = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let addr = addr_str.parse::<std::net::SocketAddr>().unwrap();
    tracing::debug!("Listening on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Cannot start the server");
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
