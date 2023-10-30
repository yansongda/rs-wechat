use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::service::totp;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "totp")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub issuer: Option<String>,
    pub secret: String,
    #[sea_orm(column_type = "DateTime", nullable)]
    pub created_at: Option<NaiveDateTime>,
    #[sea_orm(column_type = "DateTime", nullable)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::model::user::Entity",
        from = "Column::UserId",
        to = "crate::model::user::Column::Id"
    )]
    User,
}

impl Related<crate::model::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize, Debug)]
pub struct DetailRequest {
    pub id: i64,
}

#[derive(Serialize, Debug)]
pub struct DetailResponse {
    pub id: i64,
    pub issuer: String,
    pub username: String,
    pub code: String,
}

impl From<Model> for DetailResponse {
    fn from(totp: Model) -> Self {
        Self {
            id: totp.id,
            issuer: totp.issuer.clone().unwrap_or("未知发行方".to_string()),
            username: totp.username.clone(),
            code: totp::generate_code(totp.clone()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateRequest {
    pub uri: String,
}

#[derive(DeriveIntoActiveModel)]
pub struct CreateTotp {
    pub user_id: i64,
    pub username: String,
    pub issuer: Option<String>,
    pub secret: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateRequest {
    pub id: i64,
    pub issuer: Option<String>,
    pub username: Option<String>,
}

#[derive(DeriveIntoActiveModel)]
pub struct UpdateTotp {
    pub username: Option<String>,
    pub issuer: Option<String>,
}

impl From<UpdateRequest> for UpdateTotp {
    fn from(params: UpdateRequest) -> Self {
        Self {
            username: params.username,
            issuer: params.issuer,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct DeleteRequest {
    pub id: i64,
}
