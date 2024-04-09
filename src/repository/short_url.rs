use tracing::error;

use crate::model::result::{Error, Result};
use crate::model::short_url::{CreateShortlink, ShortUrl};
use crate::repository::Pool;

pub async fn find(short: &str) -> Result<ShortUrl> {
    let result = Entity::find()
        .filter(Column::Short.eq(short))
        .one(Pool::get("default"))
        .await
        .map_err(|e| {
            error!("查询 短连接 失败: {:?}", e);

            Error::Database(None)
        })?;

    if let Some(result) = result {
        return Ok(result);
    }

    Err(Error::ShortlinkNotFound(None))
}

pub async fn insert(link: CreateShortlink) -> Result<ShortUrl> {
    let result = link
        .into_active_model()
        .insert(Pool::get("default"))
        .await
        .map_err(|e| {
            error!("插入 短连接 失败: {:?}", e);

            Error::DatabaseInsert(None)
        })?;

    Ok(result)
}

pub async fn update_count(link: ShortUrl) {
    let visit = link.visit;
    let mut active_model = link.into_active_model();

    // 这里有并发问题，但是不影响使用
    active_model.visit = sea_orm::ActiveValue::Set(visit + 1);

    let _ = active_model
        .update(Pool::get("default"))
        .await
        .map_err(|e| {
            error!("更新 短连接 访问次数 失败: {:?}", e);

            Error::DatabaseUpdate(None)
        });
}
