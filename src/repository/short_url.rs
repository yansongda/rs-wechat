use tracing::error;

use crate::model::result::{Error, Result};
use crate::model::short_url::{CreateShortUrl, ShortUrl};
use crate::repository::Pool;

pub async fn fetch(short: &str) -> Result<ShortUrl> {
    let result: Option<ShortUrl> =
        sqlx::query_as("select * from yansongda.short_url where short = $1 limit 1")
            .bind(short)
            .fetch_optional(Pool::postgres("default"))
            .await
            .map_err(|e| {
                error!("查询短连接失败: {:?}", e);

                Error::Database(None)
            })?;

    if let Some(short_url) = result {
        return Ok(short_url);
    }

    Err(Error::ShortlinkNotFound(None))
}

pub async fn insert(url: CreateShortUrl) -> Result<ShortUrl> {
    sqlx::query_as("insert into yansongda.short_url (short, url) values ($1, $2) returning *")
        .bind(url.short)
        .bind(url.url)
        .fetch_one(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("插入短连接失败: {:?}", e);

            Error::DatabaseInsert(None)
        })
}

pub async fn update_count(short_url: ShortUrl) {
    let _ = sqlx::query(
        "update yansongda.short_url set visit = visit + 1, updated_at = now() where id = $1",
    )
    .bind(short_url.id)
    .execute(Pool::postgres("default"))
    .await
    .map_err(|e| {
        error!("更新短连接访问次数失败: {:?}", e);

        Error::DatabaseUpdate(None)
    });
}
