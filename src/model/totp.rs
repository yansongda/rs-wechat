use chrono::NaiveDateTime;
use garde::Validate;
use sea_orm::entity::prelude::*;
use sea_orm::{prelude::async_trait::async_trait, ActiveValue, IntoActiveModel};
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

#[derive(Debug, Copy, Clone, EnumIter, DeriveRelation)]
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

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.id = ActiveValue::NotSet;
            self.created_at = ActiveValue::Set(Some(chrono::Local::now().naive_local()));
        };

        self.updated_at = ActiveValue::Set(Some(chrono::Local::now().naive_local()));

        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
pub struct DetailRequest {
    pub id: i64,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRequest {
    #[garde(ascii, length(min = 20))]
    pub uri: String,
}

#[derive(Debug, DeriveIntoActiveModel)]
pub struct CreateTotp {
    pub user_id: i64,
    pub username: String,
    pub issuer: Option<String>,
    pub secret: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRequest {
    #[garde(skip)]
    pub id: i64,
    #[garde(ascii, length(min = 1, max = 10))]
    pub issuer: Option<String>,
    #[garde(ascii, length(min = 1, max = 25))]
    pub username: Option<String>,
}

#[derive(Debug)]
pub struct UpdateTotp {
    pub issuer: Option<String>,
    pub username: Option<String>,
}

impl IntoActiveModel<ActiveModel> for UpdateTotp {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            issuer: ActiveValue::Set(self.issuer),
            username: if let Some(username) = self.username {
                ActiveValue::Set(username)
            } else {
                ActiveValue::NotSet
            },
            ..::std::default::Default::default()
        }
    }
}

impl From<UpdateRequest> for UpdateTotp {
    fn from(params: UpdateRequest) -> Self {
        Self {
            username: params.username,
            issuer: params.issuer,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    pub id: i64,
}
