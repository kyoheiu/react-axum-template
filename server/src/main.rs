mod error;

use axum::{debug_handler, routing::get, Router};
use error::Error;
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let static_dir = ServeDir::new("static");
    let app = Router::new()
        .route("/health", get(check_health))
        .nest_service("/", static_dir.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[debug_handler]
async fn check_health() -> String {
    info!("Health check.");
    "Hello from axum.".to_string()
}
