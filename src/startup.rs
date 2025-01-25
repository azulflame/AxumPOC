use std::time::Duration;
use axum::{Router};
use axum::body::Body;
use axum::http::Request;
use axum::routing::{get, post};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::{info, info_span};
use crate::routes::{health_check, subscribe};
pub fn router(pool: DatabaseConnection) -> Router {
    Router::new()
        .route("/healthcheck", get(health_check))
        .route("/subscribe", post(subscribe))
        .layer(TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                let trace_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(||"unknown".to_string());

                info_span!("request",
                trace_id = %trace_id,
                method = %request.method(),
                uri = %request.uri())
            }))
        .layer(RequestIdLayer)
        .with_state(pool.clone())
}

pub async fn pool(conn_string: String) -> DatabaseConnection {
    let mut options = ConnectOptions::new(conn_string);
    options.max_connections(100)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    info!("{:?}", options);
    Database::connect(options).await
        .expect("Unable to connect to the database")
}