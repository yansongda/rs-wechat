use tracing::error;

use crate::model::result::{Error, Result};
use crate::model::totp::{CreateTotp, Totp, UpdateTotp};
use crate::model::user::User;
use crate::repository::Pool;

pub async fn all(current_user: User) -> Result<Vec<Totp>> {
    sqlx::query_as(
        "select * from yansongda.totp where user_id = $1",
    )
        .bind(current_user.id)
        .fetch_all(Pool::default())
        .await
        .map_err(|e| {
            error!("查询用户所有的 Totp 失败: {:?}", e);

            Error::Database(None)
        })
}

pub async fn fetch(id: i64) -> Result<Totp> {
    let result: Option<Totp> = sqlx::query_as(
        "select * from yansongda.totp where id = $1 limit 1",
    )
        .bind(id)
        .fetch_optional(Pool::default())
        .await
        .map_err(|e| {
            error!("查询 Totp 失败: {:?}", e);

            Error::Database(None)
        })?;

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::TotpNotFound(None))
}

pub async fn insert(totp: CreateTotp) -> Result<Totp> {
    sqlx::query_as(
        "insert into yansongda.totp (user_id, username, issuer, secret) values ($1, $2, $3, $4) returning *",
    )
        .bind(open_id)
        .fetch_one(Pool::default())
        .await
        .map_err(|e| {
            error!("插入 Totp 失败: {:?}", e);

            Error::DatabaseInsert(None)
        })
}

pub async fn update(model: Totp, updated: UpdateTotp) -> Result<()> {
    let mut active_model = updated.into_active_model();

    active_model.id = Set(model.id);

    active_model
        .update(Pool::get("default"))
        .await
        .map_err(|e| {
            error!("更新 Totp 失败: {:?}", e);

            Error::DatabaseInsert(None)
        })?;

    Ok(())
}

pub async fn delete(model: Totp) -> Result<()> {
    model.delete(Pool::get("default")).await.map_err(|e| {
        error!("删除 Totp 失败: {:?}", e);

        Error::DatabaseDelete(None)
    })?;

    Ok(())
}
