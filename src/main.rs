use sea_orm::DatabaseConnection;
use secrecy::ExposeSecret;
use tokio::net::TcpListener;
use AxumPOC::configuration::get_configuration;
use AxumPOC::startup::{pool, router};
use AxumPOC::telemetry::{get_subscriber, init_subscriber};
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("AxumPOC".into(), "info".into());
    init_subscriber(subscriber);

    let settings = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}", settings.application.host , settings.application.port);
    let pool = pool(settings.database.connection_string().expose_secret().to_string()).await;
    migrate_database(&pool).await;
    let router = router(pool);
    let listener = TcpListener::bind(address).await.expect("Failed to bind listener");
    axum::serve(listener, router).await.expect("Failed to run server");
}

async fn migrate_database(pool: &DatabaseConnection) {
    Migrator::up(pool, None).await.expect("TODO: panic message");
}

