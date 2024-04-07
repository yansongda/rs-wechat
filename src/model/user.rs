use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::request::user::UpdateRequest;

#[derive(Debug, Clone, Serialize, Eq, PartialEq, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub open_id: String,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UpdateUser {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

impl From<UpdateRequest> for UpdateUser {
    fn from(request: UpdateRequest) -> Self {
        Self {
            avatar: request.avatar,
            nickname: request.nickname,
            slogan: request.slogan,
        }
    }
}
