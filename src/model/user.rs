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
pub enum Relation {
    #[sea_orm(
        has_many = "crate::model::totp::Entity",
        from = "Column::Id",
        to = "crate::model::totp::Column::UserId"
    )]
    Totp,
}

impl Related<crate::model::totp::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Totp.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<CurrentUser> for Model {
    fn from(value: CurrentUser) -> Self {
        Self {
            id: value.id,
            open_id: value.open_id,
            avatar: value.avatar,
            nickname: value.nickname,
            slogan: value.slogan,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: i64,
    pub open_id: String,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<Model> for CurrentUser {
    fn from(user: Model) -> Self {
        Self {
            id: user.id,
            open_id: user.open_id,
            avatar: user.avatar,
            nickname: user.nickname,
            slogan: user.slogan,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

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

#[derive(DeriveIntoActiveModel)]
pub struct CreateUser {
    pub open_id: String,
}

#[derive(Serialize)]
pub struct DetailResponse {
    pub id: i64,
    pub open_id: String,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<CurrentUser> for DetailResponse {
    fn from(user: CurrentUser) -> Self {
        Self {
            id: user.id,
            open_id: user.open_id,
            avatar: user.avatar,
            nickname: user.nickname,
            slogan: user.slogan,
            created_at: user
                .created_at
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: user
                .updated_at
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateRequest {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

#[derive(DeriveIntoActiveModel)]
pub struct UpdateUser {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

impl From<UpdateRequest> for UpdateUser {
    fn from(value: UpdateRequest) -> Self {
        Self {
            avatar: value.avatar,
            nickname: value.nickname,
            slogan: value.slogan,
        }
    }
}
