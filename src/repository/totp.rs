use sqlx::types::Json;
use sqlx::{Execute, Postgres, QueryBuilder};
use tracing::error;

use crate::model::result::{Error, Result};
use crate::model::totp::{CreateTotp, Totp, TotpConfig, UpdateTotp};
use crate::model::user::User;
use crate::repository::Pool;

pub async fn all(current_user: User) -> Result<Vec<Totp>> {
    sqlx::query_as("select * from yansongda.totp where user_id = $1")
        .bind(current_user.id)
        .fetch_all(Pool::default())
        .await
        .map_err(|e| {
            error!("查询用户所有的 Totp 失败: {:?}", e);

            Error::Database(None)
        })
}

pub async fn fetch(id: i64) -> Result<Totp> {
    let result: Option<Totp> = sqlx::query_as("select * from yansongda.totp where id = $1 limit 1")
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
        "insert into yansongda.totp (user_id, username, issuer, secret, config) values ($1, $2, $3, $4, $5) returning *",
    )
        .bind(totp.user_id)
        .bind(totp.username)
        .bind(totp.issuer)
        .bind(totp.secret)
        .bind(Json(TotpConfig {
            period: totp.period,
        }))
        .fetch_one(Pool::default())
        .await
        .map_err(|e| {
            error!("插入 Totp 失败: {:?}", e);

            Error::DatabaseInsert(None)
        })
}

pub async fn update(updated: UpdateTotp) -> Result<()> {
    let mut builder =
        QueryBuilder::<Postgres>::new("update yansongda.totp set updated_at = now(), ");

    let mut separated = builder.separated(", ");
    if let Some(issuer) = updated.issuer {
        separated.push("issuer = ").push_bind(issuer);
    }
    if let Some(username) = updated.username {
        separated.push("username = ").push_bind(username);
    }
    separated
        .push_unseparated("where id = ")
        .push_bind(updated.id);

    sqlx::query(builder.build().sql())
        .execute(Pool::default())
        .await
        .map_err(|e| {
            error!("更新 Totp 失败: {:?}", e);

            Error::DatabaseUpdate(None)
        })?;

    Ok(())
}

pub async fn delete(totp: Totp) -> Result<()> {
    sqlx::query("delete from yansongda.totp where id = $1")
        .bind(totp.id)
        .execute(Pool::default())
        .await
        .map_err(|e| {
            error!("删除 Totp 失败: {:?}", e);

            Error::DatabaseDelete(None)
        })?;

    Ok(())
}
