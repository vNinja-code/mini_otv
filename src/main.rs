use tower_http::services::ServeDir;
use axum::{Router, routing::get};
use tracing_subscriber;

mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .nest_service(
            "/hls",
            ServeDir::new("hls"),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
