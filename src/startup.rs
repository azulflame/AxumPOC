use axum::{Router};
use axum::handler::Handler;
use axum::routing::{get, post};
use deadpool_diesel::postgres::{Manager, Object};
use crate::routes::{health_check, subscribe};
pub fn router(pool: deadpool_diesel::Pool<Manager, Object>) -> Router {

    Router::new()
        .route("/healthcheck", get(health_check))
        .route("/subscribe", post(subscribe))
        .with_state(pool)
}

pub fn pool(conn_string: String) -> deadpool_diesel::Pool<Manager, Object> {
    let manager = Manager::new(conn_string, deadpool_diesel::Runtime::Tokio1);
    deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("Failed to create pool.")
}