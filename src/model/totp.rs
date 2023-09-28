use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "totp")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub issuer: String,
    pub secret: String,
    #[sea_orm(column_type = "DateTime", nullable)]
    pub created_at: Option<NaiveDateTime>,
    #[sea_orm(column_type = "DateTime", nullable)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Deserialize, Debug)]
pub struct DetailRequest {
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct CreateRequest {
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateRequest {
    pub issuer: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct DeleteRequest {
    pub id: i32,
}

#[derive(Serialize, Debug)]
pub struct AllResponse {
    pub results: Vec<DetailResponse>,
}

#[derive(Serialize, Debug)]
pub struct DetailResponse {
    pub id: i32,
    pub issuer: String,
    pub username: String,
    pub code: String,
}
