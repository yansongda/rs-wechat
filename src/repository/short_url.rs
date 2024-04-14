use std::time::Instant;
use tracing::{error, info};

use crate::model::result::{Error, Result};
use crate::model::short_url::{CreateShortUrl, ShortUrl};
use crate::repository::Pool;

pub async fn fetch(short: &str) -> Result<ShortUrl> {
    let sql = "select * from yansongda.short_url where short = $1 limit 1";
    let started_at = Instant::now();

    let result: Option<ShortUrl> = sqlx::query_as(sql)
        .bind(short)
        .fetch_optional(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("查询短连接失败: {:?}", e);

            Error::Database(None)
        })?;

    info!(
        "{:?}, {:?} --> {:?}",
        started_at.elapsed().as_secs_f32(),
        sql,
        short
    );

    if let Some(short_url) = result {
        return Ok(short_url);
    }

    Err(Error::ShortlinkNotFound(None))
}

pub async fn insert(url: CreateShortUrl) -> Result<ShortUrl> {
    let sql = "insert into yansongda.short_url (short, url) values ($1, $2) returning *";
    let started_at = Instant::now();

    let result = sqlx::query_as(sql)
        .bind(&url.short)
        .bind(&url.url)
        .fetch_one(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("插入短连接失败: {:?}", e);

            Error::DatabaseInsert(None)
        });

    info!(
        "{:?}, {:?} --> {:?}",
        started_at.elapsed().as_secs_f32(),
        sql,
        url
    );

    result
}

pub async fn update_count(id: i64) {
    let sql = "update yansongda.short_url set visit = visit + 1, updated_at = now() where id = $1";
    let started_at = Instant::now();

    let _ = sqlx::query(
        "update yansongda.short_url set visit = visit + 1, updated_at = now() where id = $1",
    )
    .bind(id)
    .execute(Pool::postgres("default"))
    .await
    .map_err(|e| {
        error!("更新短连接访问次数失败: {:?}", e);

        Error::DatabaseUpdate(None)
    });

    info!(
        "{:?}, {:?} --> {:?}",
        started_at.elapsed().as_secs_f32(),
        sql,
        id
    );
}
