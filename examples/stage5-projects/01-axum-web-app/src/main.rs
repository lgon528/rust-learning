mod config;
mod error;
mod models;
mod auth;
mod database;
mod services;
mod handlers;
mod state;

use axum::{
    routing::{get, post},
    Router,
    http::Method,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    services::ServeDir,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use database::DatabaseService;
use auth::JwtService;
use services::{UserService, PostService};
use handlers::{
    health_handlers::{health_check, readiness_check, liveness_check},
    user_handlers::{register, login},
    post_handlers::{get_post, get_posts},
};
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::from_env()?;

    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", config.app.name).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    let db_service = DatabaseService::new(&config).await?;

    // Test database connection
    db_service.test_connection().await?;

    // Initialize services
    let jwt_service = JwtService::new(config.jwt_secret());
    let user_service = UserService::new(db_service.pool().clone(), jwt_service.clone());
    let post_service = PostService::new(db_service.pool().clone());

    // Build CORS layer
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .allow_origin(Any);

    // Create application state
    let app_state = AppState {
        user_service,
        post_service,
        jwt_service,
    };

    // Build application router
    let app = Router::new()
        // Health checks
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        .route("/live", get(liveness_check))

        // Public routes
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))

        // Public posts routes
        .route("/api/posts", get(get_posts))
        .route("/api/posts/:id", get(get_post))

        // Static files (for serving API documentation, etc.)
        .nest_service("/static", ServeDir::new("static"))

        // Add middleware and state
        .with_state(app_state.clone())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        )
        .fallback_service(ServeDir::new("static").append_index_html_on_directories(true));

    // Start server
    let server_address = config.server_address();
    tracing::info!("🚀 Starting {} v{} on {}", config.app.name, config.app.version, server_address);
    tracing::info!("📚 API Documentation available at http://{}/api-docs", server_address);
    tracing::info!("🏥 Health check available at http://{}/health", server_address);

    let listener = tokio::net::TcpListener::bind(&server_address).await?;
    axum::serve(listener, app).await?;


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_health_check() {
        let app = Router::new()
            .route("/health", get(health_check));

        let server = TestServer::new(app).unwrap();

        let response = server.get("/health").await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = response.json();
        assert_eq!(body["status"], "healthy");
        assert_eq!(body["service"], "axum-web-app");
        assert_eq!(body["version"], "0.1.0");
    }

    #[tokio::test]
    async fn test_readiness_check() {
        let app = Router::new()
            .route("/ready", get(readiness_check));

        let server = TestServer::new(app).unwrap();

        let response = server.get("/ready").await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = response.json();
        assert_eq!(body["status"], "ready");
    }

    #[tokio::test]
    async fn test_liveness_check() {
        let app = Router::new()
            .route("/live", get(liveness_check));

        let server = TestServer::new(app).unwrap();

        let response = server.get("/live").await;

        assert_eq!(response.status_code(), 200);

        let body: serde_json::Value = response.json();
        assert_eq!(body["status"], "alive");
    }
}
