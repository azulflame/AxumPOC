use axum::{Router};
use axum::routing::get;
use axum::serve::Serve;
use tokio::net::TcpListener;


pub fn router() -> Router {
    Router::new()
        .route("/healthcheck", get(health_check))
}

async fn health_check() {
}