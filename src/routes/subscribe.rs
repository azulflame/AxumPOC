use anyhow::anyhow;
use axum::body::Body;
use axum::{Form};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait, InsertResult};
use sea_orm::prelude::DateTimeUtc;
use sea_orm::sqlx::PgPool;
use tracing::error;
use uuid::Uuid;
use entity::prelude::*;
use entity::subscriptions;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, pool),
fields(
subscriber_email = %form.email,
subscriber_name = %form.name
)
)]
pub async fn subscribe(
    State(pool): State<DatabaseConnection>,
    Form(form): Form<FormData>
) -> Response {

    let mut new_subscription = subscriptions::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        name: ActiveValue::Set(form.name),
        email: ActiveValue::Set(form.email),
        subscribed_at: ActiveValue::Set(NaiveDateTime::from(Utc::now().naive_utc())),
    };


    match insert_subscriber_to_database(pool, new_subscription).await {
        Ok(_) => Response::builder().status(StatusCode::OK).body(Body::empty()).expect("Unable to create new subscriber response"),
        Err(_) => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).expect("Unable to create new subscriber response"),
    }
}

#[tracing::instrument(
name = "Saving a new subscriber",
skip(pool, sub)
)]
pub async fn insert_subscriber_to_database(
    pool: DatabaseConnection,
    sub: subscriptions::ActiveModel
) -> anyhow::Result<subscriptions::Model> {
    sub.insert(&pool).await.map_err(|database_error| {
        error!("Database Error: {:?}", database_error);
        anyhow!(database_error)
    })
}