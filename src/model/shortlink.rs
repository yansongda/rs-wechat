use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::{prelude::async_trait::async_trait, ActiveValue};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "shortlink")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub short: String,
    pub link: String,
    pub visit: i64,
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

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    pub link: String,
    pub short: String,
}

impl From<Model> for CreateResponse {
    fn from(model: Model) -> Self {
        Self {
            link: model.link,
            short: format!(
                "{}/{}",
                Config::get::<String>("shortlink.domain"),
                model.short
            ),
        }
    }
}

#[derive(Debug, DeriveIntoActiveModel)]
pub struct CreateShortlink {
    pub user_id: i64,
    pub link: String,
    pub short: String,
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub link: String,
    pub short: String,
}

impl From<Model> for DetailResponse {
    fn from(model: Model) -> Self {
        Self {
            link: model.link,
            short: format!(
                "{}/{}",
                Config::get::<String>("shortlink.domain"),
                model.short
            ),
        }
    }
}
