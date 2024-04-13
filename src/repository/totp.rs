use sqlx::types::Json;
use sqlx::{Postgres, QueryBuilder};
use tracing::{error, info};

use crate::model::result::{Error, Result};
use crate::model::totp::{CreateTotp, Totp, TotpConfig, UpdateTotp};
use crate::repository::Pool;

pub async fn all(user_id: i64) -> Result<Vec<Totp>> {
    let sql = "select * from yansongda.totp where user_id = $1";

    info!("{:?} --> {:?}", sql, user_id);

    sqlx::query_as("select * from yansongda.totp where user_id = $1")
        .bind(user_id)
        .fetch_all(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("查询用户所有的 Totp 失败: {:?}", e);

            Error::Database(None)
        })
}

pub async fn fetch(id: i64) -> Result<Totp> {
    let sql = "select * from yansongda.totp where id = $1 limit 1";

    info!("{:?} --> {:?}", sql, id);

    let result: Option<Totp> = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(Pool::postgres("default"))
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
    let sql = "insert into yansongda.totp (user_id, username, issuer, secret, config) values ($1, $2, $3, $4, $5) returning *";

    info!("{:?} --> {:?}", sql, totp);

    sqlx::query_as(sql)
        .bind(totp.user_id)
        .bind(totp.username)
        .bind(totp.issuer)
        .bind(totp.secret)
        .bind(Json(TotpConfig {
            period: totp.period,
        }))
        .fetch_one(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("插入 Totp 失败: {:?}", e);

            Error::DatabaseInsert(None)
        })
}

pub async fn update(updated: UpdateTotp) -> Result<()> {
    let mut builder = QueryBuilder::<Postgres>::new("update yansongda.totp set updated_at = now()");

    if let Some(ref issuer) = updated.issuer {
        builder.push(", issuer = ").push_bind(issuer);
    }
    if let Some(ref username) = updated.username {
        builder.push(", username = ").push_bind(username);
    }

    builder.push(" where id = ").push_bind(updated.id);

    info!("{:?} --> {:?}", builder.sql(), updated);

    builder
        .build()
        .execute(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("更新 Totp 失败: {:?}", e);

            Error::DatabaseUpdate(None)
        })?;

    Ok(())
}

pub async fn delete(id: i64) -> Result<()> {
    let sql = "delete from yansongda.totp where id = $1";

    info!("{:?} --> {:?}", sql, id);

    sqlx::query(sql)
        .bind(id)
        .execute(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("删除 Totp 失败: {:?}", e);

            Error::DatabaseDelete(None)
        })?;

    Ok(())
}
