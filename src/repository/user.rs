use sqlx::{Execute, Postgres, QueryBuilder};
use tracing::error;

use crate::model::result::{Error, Result};
use crate::model::user::{UpdateUser, User};
use crate::repository::Pool;

pub async fn fetch(open_id: &str) -> Result<User> {
    let result: Option<User> =
        sqlx::query_as("select * from yansongda.user where open_id = $1 limit 1")
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
    sqlx::query_as("insert into yansongda.user (open_id) values ($1) returning *")
        .bind(open_id)
        .fetch_one(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("插入用户失败: {:?}", e);

            Error::DatabaseInsert(None)
        })
}

pub async fn update(current_user: User, update_user: UpdateUser) -> Result<User> {
    let mut builder =
        QueryBuilder::<Postgres>::new("update yansongda.user set updated_at = now(), ");

    let mut separated = builder.separated(", ");
    if let Some(nickname) = update_user.nickname {
        separated.push("nickname = ").push_bind(nickname);
    }
    if let Some(avatar) = update_user.avatar {
        separated.push("avatar = ").push_bind(avatar);
    }
    if let Some(slogan) = update_user.slogan {
        separated.push("slogan = ").push_bind(slogan);
    }
    separated
        .push_unseparated("where id = ")
        .push_bind(current_user.id);

    sqlx::query_as(builder.push(" returning *").build().sql())
        .fetch_one(Pool::postgres("default"))
        .await
        .map_err(|e| {
            error!("更新用户失败: {:?}", e);

            Error::DatabaseUpdate(None)
        })
}
