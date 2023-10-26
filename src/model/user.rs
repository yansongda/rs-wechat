use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Eq, PartialEq, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub open_id: String,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
    #[sea_orm(column_type = "DateTime", nullable)]
    pub created_at: Option<NaiveDateTime>,
    #[sea_orm(column_type = "DateTime", nullable)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub code: String,
}


#[derive(Serialize)]
pub struct LoginResponse {
    pub open_id: String,
}

impl From<Model> for LoginResponse {
    fn from(user: Model) -> Self {
        Self {
            open_id: user.open_id,
        }
    }
}

#[derive(Serialize)]
pub struct DetailResponse {
    pub id: i32,
    pub open_id: String,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Model> for DetailResponse {
    fn from(user: Model) -> Self {
        Self {
            id: user.id as i32,
            open_id: user.open_id,
            avatar: user.avatar,
            nickname: user.nickname,
            slogan: user.slogan,
            created_at: user.created_at.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: user.updated_at.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}