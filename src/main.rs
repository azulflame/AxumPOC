use axum::Router;
use axum::serve::Serve;
use tokio::net::TcpListener;
use AxumPOC::router;
#[tokio::main]
async fn main() {
    let router = router();
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind listener");
    axum::serve(listener, router).await.expect("Failed to run server");
}

