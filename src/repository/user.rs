use sqlx::{Execute, Postgres, QueryBuilder};
use tracing::{error, info};

use crate::model::result::{Error, Result};
use crate::model::user::{UpdateUser, User};
use crate::repository::Pool;

pub async fn fetch(open_id: &str) -> Result<User> {
    let sql = "select * from yansongda.user where open_id = $1 limit 1";

    info!("{:?} --> {:?}", sql, open_id);

    let result: Option<User> = sqlx::query_as(sql)
        .bind(open_id)
        .fetch_optional(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("查询用户失败: {:?}", e);

            Error::Database(None)
        })?;

    if let Some(user) = result {
        return Ok(user);
    }

    Err(Error::UserNotFound(None))
}

pub async fn insert(open_id: &str) -> Result<User> {
    let sql = "insert into yansongda.user (open_id) values ($1) returning *";

    info!("{:?} --> {:?}", sql, open_id);

    sqlx::query_as(sql)
        .bind(open_id)
        .fetch_one(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("插入用户失败: {:?}", e);

            Error::DatabaseInsert(None)
        })
}

pub async fn update(id: i64, update_user: UpdateUser) -> Result<User> {
    let mut builder = QueryBuilder::<Postgres>::new("update yansongda.user set updated_at = now()");

    if let Some(ref nickname) = update_user.nickname {
        builder.push(", nickname = ").push_bind(nickname);
    }
    if let Some(ref avatar) = update_user.avatar {
        builder.push(", avatar = ").push_bind(avatar);
    }
    if let Some(ref slogan) = update_user.slogan {
        builder.push(", slogan = ").push_bind(slogan);
    }

    builder
        .push(" where id = ")
        .push_bind(id)
        .push(" returning *");

    let query = builder.build_query_as();

    info!("{:?} --> {:?}, {:?}", query.sql(), id, update_user);

    query
        .fetch_one(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("更新用户失败: {:?}", e);

            Error::DatabaseUpdate(None)
        })
}
