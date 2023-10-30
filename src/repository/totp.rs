use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait};

use crate::model::result::{Error, Result};
use crate::model::totp::{CreateTotp, Entity, Model as Totp, UpdateTotp};
use crate::model::user::Model as User;
use crate::repository::Pool;

pub async fn all(user: User) -> Result<Vec<Totp>> {
    user.find_related(Entity)
        .all(Pool::get("default"))
        .await
        .map_err(|_| Error::Database)
}

pub async fn find(id: i64) -> Result<Totp> {
    let result = Entity::find_by_id(id)
        .one(Pool::get("default"))
        .await
        .map_err(|_| Error::Database)?;

    if let Some(result) = result {
        return Ok(result);
    }

    Err(Error::TotpNotFound)
}

pub async fn insert(totp: CreateTotp) -> Result<Totp> {
    let mut active_model = totp.into_active_model();

    active_model.created_at = Set(Some(chrono::Local::now().naive_local()));
    active_model.updated_at = Set(Some(chrono::Local::now().naive_local()));

    let result = active_model
        .insert(Pool::get("default"))
        .await
        .map_err(|_| Error::DatabaseInsert)?;

    Ok(result)
}

pub async fn update(model: Totp, updated: UpdateTotp) -> Result<()> {
    let mut active_model = updated.into_active_model();

    active_model.id = Set(model.id);
    active_model.updated_at = Set(Some(chrono::Local::now().naive_local()));

    active_model
        .update(Pool::get("default"))
        .await
        .map_err(|_| Error::DatabaseInsert)?;

    Ok(())
}

pub async fn delete(model: Totp) -> Result<()> {
    model
        .delete(Pool::get("default"))
        .await
        .map_err(|_| Error::DatabaseDelete)?;

    Ok(())
}
