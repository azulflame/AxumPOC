use axum::{Router};
use axum::body::Body;
use axum::http::Request;
use axum::routing::{get, post};
use deadpool_diesel::postgres::{Manager, Object};
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::info_span;
use crate::routes::{health_check, subscribe};
pub fn router(pool: deadpool_diesel::Pool<Manager, Object>) -> Router {

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
        .with_state(pool)
}

pub fn pool(conn_string: String) -> deadpool_diesel::Pool<Manager, Object> {
    let manager = Manager::new(conn_string, deadpool_diesel::Runtime::Tokio1);
    deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("Failed to create pool.")
}