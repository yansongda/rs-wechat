use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter};
use crate::model::user::{ActiveModel, Column, Entity, Model as User};
use crate::model::result::Error;
use crate::repository::default_pool;

pub async fn find_one(open_id: String) -> Result<User, Error> {
    let result: Option<User> = Entity::find().filter(
        Column::OpenId.eq(open_id)
    )
        .one(default_pool())
        .await
        .map_err(|_| Error::Database)?;

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::UserNotFound)
}

pub async fn create(user: &User) -> Result<User, Error> {
    let active_model: ActiveModel = user.into();

    let result = active_model.insert(default_pool())
        .await
        .map_err(|_| Error::Database)?;

    Ok(result)
}
