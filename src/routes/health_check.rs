use axum::body::Body;
use axum::http::StatusCode;
use axum::response::Response;

pub async fn health_check() -> Response {
    Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap()
}