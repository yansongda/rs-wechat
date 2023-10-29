use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};

use crate::model::result::{Error, Result};
use crate::model::user;
use crate::model::user::{Column, CreateUser, Entity, Model as User, UpdateUser};
use crate::repository::Pool;

pub async fn find(open_id: &str) -> Result<User> {
    let result: Option<User> = Entity::find()
        .filter(Column::OpenId.eq(open_id))
        .one(Pool::get("default"))
        .await
        .map_err(|_| Error::Database)?;

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::UserNotFound)
}

pub async fn insert(user: CreateUser) -> Result<User> {
    let active_model = user::ActiveModel {
        open_id: Set(user.open_id),
        created_at: Set(Some(chrono::Local::now().naive_local())),
        updated_at: Set(Some(chrono::Local::now().naive_local())),
        ..Default::default()
    };

    active_model
        .insert(Pool::get("default"))
        .await
        .map_err(|_| Error::DatabaseInsert)
}

pub async fn update(model: User, updated: UpdateUser) -> Result<User> {
    let mut active_model = model.into_active_model();

    if updated.avatar.is_some() {
        active_model.avatar = Set(updated.avatar);
    }

    if updated.nickname.is_some() {
        active_model.nickname = Set(updated.nickname);
    }

    if updated.slogan.is_some() {
        active_model.slogan = Set(updated.slogan);
    }

    active_model.updated_at = Set(Some(chrono::Local::now().naive_local()));

    active_model
        .update(Pool::get("default"))
        .await
        .map_err(|_| Error::DatabaseUpdate)
}
