use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub open_id: String,
    pub avatar: String,
    pub nickname: String,
    pub slogan: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Totp {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub issuer: String,
    pub secret: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
