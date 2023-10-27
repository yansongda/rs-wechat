use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use sea_orm::ActiveValue::Set;

use crate::model::result::{Error, Result};
use crate::model::user;
use crate::model::user::{Column, CreateUser, Entity, Model as User, UpdateUser};
use crate::repository::Pool;

pub async fn find_one(open_id: &str) -> Result<User> {
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

pub async fn create(user: CreateUser) -> Result<User> {
    let active_model = user::ActiveModel {
        open_id: Set(user.open_id),
        created_at: Set(Some(chrono::Local::now().naive_local())),
        updated_at: Set(Some(chrono::Local::now().naive_local())),
        ..Default::default()
    };

    Ok(active_model
        .insert(Pool::get("default"))
        .await
        .map_err(|_| Error::Insert)?)
}

pub async fn update(user: UpdateUser) -> Result<User> {
    let mut active_model = user.into_active_model();

    active_model.updated_at = Set(Some(chrono::Local::now().naive_local()));

    Ok(active_model
        .update(Pool::get("default"))
        .await
        .map_err(|_| Error::Update)?)
}
