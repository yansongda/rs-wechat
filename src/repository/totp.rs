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
        .map_err(|e| {
            println!("查询用户所有的 Totp 失败: {:?}", e);

            Error::Database
        })
}

pub async fn find(id: i64) -> Result<Totp> {
    let result = Entity::find_by_id(id)
        .one(Pool::get("default"))
        .await
        .map_err(|e| {
            println!("查询 Totp 失败: {:?}", e);

            Error::Database
        })?;

    if let Some(result) = result {
        return Ok(result);
    }

    Err(Error::TotpNotFound)
}

pub async fn insert(totp: CreateTotp) -> Result<Totp> {
    let result = totp
        .into_active_model()
        .insert(Pool::get("default"))
        .await
        .map_err(|e| {
            println!("插入 Totp 失败: {:?}", e);

            Error::DatabaseInsert
        })?;

    Ok(result)
}

pub async fn update(model: Totp, updated: UpdateTotp) -> Result<()> {
    let mut active_model = updated.into_active_model();

    active_model.id = Set(model.id);

    active_model
        .update(Pool::get("default"))
        .await
        .map_err(|e| {
            println!("更新 Totp 失败: {:?}", e);

            Error::DatabaseInsert
        })?;

    Ok(())
}

pub async fn delete(model: Totp) -> Result<()> {
    model
        .delete(Pool::get("default"))
        .await
        .map_err(|e| {
            println!("删除 Totp 失败: {:?}", e);

            Error::DatabaseDelete
        })?;

    Ok(())
}
