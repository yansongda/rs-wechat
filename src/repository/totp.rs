use sqlx::types::Json;
use sqlx::{Execute, Postgres, QueryBuilder};
use std::time::Instant;
use tracing::{error, info};

use crate::model::result::{Error, Result};
use crate::model::totp::{CreateTotp, Totp, TotpConfig, UpdateTotp};
use crate::repository::Pool;

pub async fn all(user_id: i64) -> Result<Vec<Totp>> {
    let sql = "select * from miniprogram.totp where user_id = $1";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(user_id)
        .fetch_all(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("查询用户所有的 Totp 失败: {:?}", e);

            Error::Database(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, user_id);

    result
}

pub async fn fetch(id: i64) -> Result<Totp> {
    let sql = "select * from miniprogram.totp where id = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<Totp> = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("查询 Totp 失败: {:?}", e);

            Error::Database(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::TotpNotFound(None))
}

pub async fn insert(totp: CreateTotp) -> Result<Totp> {
    let sql = "insert into miniprogram.totp (user_id, username, issuer, secret, config) values ($1, $2, $3, $4, $5) returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(totp.user_id)
        .bind(&totp.username)
        .bind(&totp.issuer)
        .bind(&totp.secret)
        .bind(Json(TotpConfig {
            period: totp.period,
        }))
        .fetch_one(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("插入 Totp 失败: {:?}", e);

            Error::DatabaseInsert(None)
        });

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, ?totp);

    result
}

pub async fn update(updated: UpdateTotp) -> Result<()> {
    let mut builder = QueryBuilder::<Postgres>::new("update miniprogram.totp set updated_at = now()");

    if let Some(ref issuer) = updated.issuer {
        builder.push(", issuer = ").push_bind(issuer);
    }
    if let Some(ref username) = updated.username {
        builder.push(", username = ").push_bind(username);
    }

    builder.push(" where id = ").push_bind(updated.id);

    let query = builder.build();
    let sql = query.sql();
    let started_at = Instant::now();

    query
        .execute(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("更新 Totp 失败: {:?}", e);

            Error::DatabaseUpdate(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, ?updated);

    Ok(())
}

pub async fn delete(id: i64) -> Result<()> {
    let sql = "delete from miniprogram.totp where id = $1";
    let started_at = Instant::now();

    sqlx::query(sql)
        .bind(id)
        .execute(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("删除 Totp 失败: {:?}", e);

            Error::DatabaseDelete(None)
        })?;

    let elapsed = started_at.elapsed().as_secs_f32();

    info!(elapsed, sql, id);

    Ok(())
}
