use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use sea_orm::ActiveValue::Set;

use crate::model::result::{Result, Error};
use crate::model::user::{Column, Entity, Model as User};
use crate::repository::Pool;

pub async fn find_one(open_id: String) -> Result<User> {
    let result: Option<User> = Entity::find()
        .filter(Column::OpenId.eq(open_id))
        .one(Pool::get("default"))
        .await.map_err(|_| Error::Database)?;

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::UserNotFound)
}

pub async fn create(user: User) -> Result<User> {
    let mut active_model = user.into_active_model();

    active_model.created_at = Set(Some(chrono::Local::now().naive_local()));
    active_model.updated_at = Set(Some(chrono::Local::now().naive_local()));

    Ok(active_model.insert(Pool::get("default")).await.map_err(|_| Error::Insert)?)
}
