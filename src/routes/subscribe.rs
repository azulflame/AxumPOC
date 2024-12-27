use std::sync::Arc;
use axum::body::Body;
use axum::{Extension, Form};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, PgConnection, RunQueryDsl, SelectableHelper};
use uuid::Uuid;
use crate::database::models::{NewSubscription, Subscription};
use crate::database::schema::subscriptions::dsl::subscriptions;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    Form(form): Form<FormData>
) -> Response {
    let new_subscription = NewSubscription {id: Uuid::new_v4(), email: form.email.clone(), name: form.name.clone(), subscribed_at: Utc::now() };

    let conn = pool.get().await.expect("Failed to get connection from pool");

    let returning = conn.interact(|c| {


        diesel::insert_into(subscriptions)
        .values(new_subscription)
        .returning(Subscription::as_returning())
        .get_result(c)
        .expect("unable to insert the query")
    })
        .await
        .expect("Failed to insert subscription");
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .expect("failed to build response")
}