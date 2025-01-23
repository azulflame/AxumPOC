use secrecy::ExposeSecret;
use tokio::net::TcpListener;
use AxumPOC::configuration::get_configuration;
use AxumPOC::startup::{pool, router};
use AxumPOC::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("AxumPOC".into(), "info".into());
    init_subscriber(subscriber);


    let settings = get_configuration().expect("Failed to read configuration");
    let pool = pool(settings.database.connection_string().expose_secret().to_string());
    let router = router(pool);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", settings.app_port)).await.expect("Failed to bind listener");
    axum::serve(listener, router).await.expect("Failed to run server");
}

