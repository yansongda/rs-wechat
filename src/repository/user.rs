use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};

use crate::model::result::Error;
use crate::model::user::{Column, Entity, Model as User};
use crate::repository::{Pool, POOL};

pub async fn find_one(open_id: String) -> Result<User, Error> {
    let result: Option<User> = Entity::find().filter(
        Column::OpenId.eq(open_id)
    )
        .one(&POOL.get().unwrap().default)
        .await
        .map_err(|_| Error::Database)?;

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::UserNotFound)
}

pub async fn create(user: User) -> Result<User, Error> {
    let result = user.into_active_model().insert(POOL.default)
        .await
        .map_err(|_| Error::Database)?;

    Ok(result)
}
