use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter,
};

use crate::model::result::{Error, Result};
use crate::model::shortlink::{Column, CreateShortlink, Entity, Model as Shortlink};
use crate::model::user::Model as User;
use crate::repository::Pool;

pub async fn all(user: User) -> Result<Vec<Shortlink>> {
    user.find_related(Entity)
        .all(Pool::get("default"))
        .await
        .map_err(|e| {
            println!("查询用户所有的 短连接 失败: {:?}", e);

            Error::Database
        })
}

pub async fn find(short: String) -> Result<Shortlink> {
    let result = Entity::find()
        .filter(Column::Short.eq(short))
        .one(Pool::get("default"))
        .await
        .map_err(|e| {
            println!("查询 短连接 失败: {:?}", e);

            Error::Database
        })?;

    if let Some(result) = result {
        return Ok(result);
    }

    Err(Error::ShortlinkNotFound)
}

pub async fn insert(totp: CreateShortlink) -> Result<Shortlink> {
    let result = totp
        .into_active_model()
        .insert(Pool::get("default"))
        .await
        .map_err(|e| {
            println!("插入 短连接 失败: {:?}", e);

            Error::DatabaseInsert
        })?;

    Ok(result)
}

pub async fn delete(model: Shortlink) -> Result<()> {
    model.delete(Pool::get("default")).await.map_err(|e| {
        println!("删除 短连接 失败: {:?}", e);

        Error::DatabaseDelete
    })?;

    Ok(())
}
