use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, QueryFilter, ColumnTrait};
use sea_orm::ActiveValue::Set;

use crate::model::result::Error;
use crate::model::totp::{ActiveModel, Column, Entity, Model as Totp};
use crate::repository::Pool;

pub async fn find(user_id: i64) -> Result<Vec<Totp>, Error> {
    Entity::find()
        .filter(Column::UserId.eq(user_id))
        .all(Pool::get("default"))
        .await
        .map_err(|_| Error::Database)
}

pub async fn find_by_id(id: i64) -> Result<Totp, Error> {
    let result = Entity::find_by_id(id)
        .one(Pool::get("default"))
        .await
        .map_err(|_| Error::Database)?;

    if let Some(result) = result {
        return Ok(result);
    }

    Err(Error::UserNotFound)
}

pub async fn create(totp: Totp) -> Result<Totp, Error> {
    let mut active_model = totp.into_active_model();

    active_model.created_at = Set(Some(chrono::Local::now().naive_local()));
    active_model.updated_at = Set(Some(chrono::Local::now().naive_local()));

    let result = active_model.insert(Pool::get("default"))
        .await
        .map_err(|_| Error::Database)?;

    Ok(result)
}

pub async fn update(updated: ActiveModel) -> Result<(), Error> {
    updated
        .update(Pool::get("default"))
        .await
        .map_err(|_| Error::Database)?;

    Ok(())
}
