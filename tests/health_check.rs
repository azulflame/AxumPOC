use std::sync::Arc;
use axum::body::Body;
use axum::http::Request;
use secrecy::ExposeSecret;
use tower::ServiceExt;
use AxumPOC::configuration::get_configuration;
use AxumPOC::startup::{pool, router};

#[tokio::test]
async fn health_check_works() {
    let settings = get_configuration().expect("Failed to read configuration");
    let pool = pool(settings.database.connection_string().expose_secret().to_string());
    let router = router(pool);

    let response = router.oneshot(Request::builder().uri("/healthcheck").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert!(response.status().is_success());
}