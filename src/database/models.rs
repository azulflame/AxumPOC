use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::database::schema::subscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subscription {
    id: Uuid,
    email: String,
    name: String,
    subscribed_at: NaiveDateTime
}

#[derive(Insertable, Clone)]
#[diesel(table_name = crate::database::schema::subscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSubscription {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub subscribed_at: DateTime<Utc>
}