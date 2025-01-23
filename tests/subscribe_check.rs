use axum::body::Body;
use axum::http::Request;
use diesel::{Connection, PgConnection};
use tower::ServiceExt;
use AxumPOC::configuration::get_configuration;
use AxumPOC::database::schema::subscriptions::dsl::subscriptions;
use AxumPOC::database::schema::subscriptions::{name, email};
use diesel::prelude::*;
use secrecy::ExposeSecret;
use uuid::Uuid;
use AxumPOC::database::models::Subscription;
use AxumPOC::startup::{pool, router};

#[tokio::test]
async fn subscribe_returns_200_with_valid_information() {
    let settings = get_configuration().expect("Failed to read configuration");
    let pool = pool(settings.database.connection_string().expose_secret().to_string());
    let router = router(pool);
    let person_name = format!("John Doe-{}", Uuid::new_v4().to_string());
    let person_email = format!("{}@example.com", Uuid::new_v4().to_string());
    let response = router.oneshot(
        Request::builder()
            .uri("/subscribe")
            .method("POST")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(format!("name={}&email={}", person_name, person_email)))
            .unwrap()
    ).await.expect("Unable to make the request");

    let configuration = get_configuration().expect("Unable to get a configuration");
    let conn_string = configuration.database.connection_string().expose_secret().to_string();
    let conn = &mut PgConnection::establish(&conn_string)
        .expect("Error connecting to postgres");

    assert_eq!(response.status().as_u16(), 200);

    let saved = subscriptions.filter(name.eq(person_name).and(email.eq(person_email))).limit(1).select(Subscription::as_select()).load(conn).expect("Unable to retrieve the subscription");
    assert_eq!(saved.len(), 1);
}
#[tokio::test]
async fn subscribe_returns_422_with_bad_fields() {
    let settings = get_configuration().expect("Failed to read configuration");
    let pool = pool(settings.database.connection_string().expose_secret().to_string());
    let router = router(pool);
    let requests = vec![
        ("name=John%20Doe", "Missing_email"),
        ("email=test%40example.com", "missing_name"),
        ("", "Missing_name_and_email")
    ];

    for (invalid_body, error_message) in requests {
        let response = router.clone().oneshot(
            Request::builder()
                .uri("/subscribe")
                .method("POST")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(invalid_body.to_string())).unwrap()
        ).await.unwrap();
        assert_eq!(422, response.status().as_u16());
    }
}