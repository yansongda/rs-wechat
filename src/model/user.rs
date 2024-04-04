use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::request::user::{LoginRequest, UpdateRequest};

#[derive(Debug, Clone, Serialize, Eq, PartialEq, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub open_id: String,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct LoginParams {
    pub code: String,
}

impl From<&LoginRequest> for LoginParams {
    fn from(value: &LoginRequest) -> Self {
        Self {
            code: value.code.clone().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct CreateUser {
    pub open_id: String,
}

pub struct UpdateParams {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

impl From<&UpdateRequest> for UpdateParams {
    fn from(value: &UpdateRequest) -> Self {
        Self {
            avatar: value.avatar.clone(),
            nickname: value.nickname.clone(),
            slogan: value.slogan.clone(),
        }
    }
}

#[derive(Debug)]
pub struct UpdateUser {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

impl From<UpdateParams> for UpdateUser {
    fn from(value: UpdateParams) -> Self {
        Self {
            avatar: value.avatar,
            nickname: value.nickname,
            slogan: value.slogan,
        }
    }
}
