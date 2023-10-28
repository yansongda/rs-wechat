use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};

use crate::model::result::{Error, Result};
use crate::model::totp::{ActiveModel, Column, Entity, Model as Totp};
use crate::repository::Pool;

pub async fn find(user_id: i64) -> Result<Vec<Totp>> {
    Entity::find()
        .filter(Column::UserId.eq(user_id))
        .all(Pool::get("default"))
        .await
        .map_err(|_| Error::Database)
}

pub async fn find_by_id(id: i64) -> Result<Totp> {
    let result = Entity::find_by_id(id)
        .one(Pool::get("default"))
        .await
        .map_err(|_| Error::Database)?;

    if let Some(result) = result {
        return Ok(result);
    }

    Err(Error::UserNotFound)
}

pub async fn create(totp: Totp) -> Result<Totp> {
    let mut active_model = totp.into_active_model();

    active_model.created_at = Set(Some(chrono::Local::now().naive_local()));
    active_model.updated_at = Set(Some(chrono::Local::now().naive_local()));

    let result = active_model
        .insert(Pool::get("default"))
        .await
        .map_err(|_| Error::DatabaseInsert)?;

    Ok(result)
}

pub async fn update(updated: ActiveModel) -> Result<()> {
    updated
        .update(Pool::get("default"))
        .await
        .map_err(|_| Error::DatabaseInsert)?;

    Ok(())
}
