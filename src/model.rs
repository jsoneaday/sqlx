use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct EntityId {
    pub id: i64
}

#[derive(Serialize, FromRow, Debug)]
pub struct Profile {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub full_name: String,
    pub email: String
}

#[derive(FromRow, Debug)]
pub struct Message {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: i64,
    pub body: String,
    pub likes: i32
}