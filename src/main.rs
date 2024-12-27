use tokio::net::TcpListener;
use AxumPOC::configuration::get_configuration;
use AxumPOC::startup::{pool, router};

#[tokio::main]
async fn main() {
    let settings = get_configuration().expect("Failed to read configuration");
    let pool = pool(settings.database.connection_string());
    let router = router(pool);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.app_port)).await.expect("Failed to bind listener");
    axum::serve(listener, router).await.expect("Failed to run server");
}

