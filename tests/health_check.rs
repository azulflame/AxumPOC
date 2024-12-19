use axum::body::Body;
use axum::http::Request;
use AxumPOC::router;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_works() {
    let router = router();

    let response = router.oneshot(Request::builder().uri("/healthcheck").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert!(response.status().is_success());
}