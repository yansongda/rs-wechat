use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::request::totp::UpdateRequest;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
#[sea_orm(table_name = "totp")]
pub struct Totp {
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub issuer: Option<String>,
    pub period: i64,
    pub secret: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug)]
pub struct CreateTotp {
    pub user_id: i64,
    pub username: String,
    pub issuer: Option<String>,
    pub period: i64,
    pub secret: String,
}

#[derive(Debug)]
pub struct UpdateTotp {
    pub id: i64,
    pub issuer: Option<String>,
    pub username: Option<String>,
}

impl From<UpdateRequest> for UpdateTotp {
    fn from(request: UpdateRequest) -> Self {
        Self {
            id: request.id.unwrap(),
            username: request.username,
            issuer: request.issuer,
        }
    }
}
