use anyhow::Result;
use axum::body::Body;
use axum::{Form};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use chrono::{Utc};
use deadpool_diesel::postgres::{Pool, PoolError};
use diesel::{RunQueryDsl, SelectableHelper};
use tracing::Instrument;
use uuid::Uuid;
use crate::database::models::{NewSubscription, Subscription};
use crate::database::schema::subscriptions::dsl::subscriptions;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, pool),
fields(
request_id=%uuid::Uuid::new_v4(),
subscriber_email = %form.email,
subscriber_name = %form.name
)
)]
pub async fn subscribe(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Form(form): Form<FormData>
) -> Response {


    let new_subscription = NewSubscription {id: Uuid::new_v4(), email: form.email.clone(), name: form.name.clone(), subscribed_at: Utc::now() };

    match insert_subscriber_to_database(&pool, new_subscription).await {
        Ok(_) => Response::builder().status(StatusCode::OK).body(Body::empty()).expect("Unable to create new subscriber response"),
        Err(_) => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).expect("Unable to create new subscriber response"),
    }
}

pub async fn insert_subscriber_to_database(
    pool: &Pool,
    sub: NewSubscription
) -> anyhow::Result<Subscription> {
    let conn = pool.get().await?;

    conn.interact(move |c| {
    diesel::insert_into(subscriptions)
        .values(sub)
        .returning(Subscription::as_returning())
        .get_result(c)
    })
        .await.map_err(|e| {tracing::error!("{:?}", e); e})
        .map_err(anyhow::Error::from(|e| format!("Failed to insert into the database: {}", e.description())))?
}