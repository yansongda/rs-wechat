use sqlx::{Execute, Postgres, QueryBuilder};
use tracing::error;

use crate::model::result::{Error, Result};
use crate::model::user::User;
use crate::repository::Pool;
use crate::request::user::UpdateRequest;

pub async fn fetch_optional(open_id: &str) -> Result<User> {
    let result: Option<User> = sqlx::query_as(
        "select * from yansongda.user where open_id = $1 limit 1",
    )
        .bind(open_id)
        .fetch_optional(Pool::default())
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
    sqlx::query_as(
        "insert into yansongda.user (open_id) values ($1) returning *",
    )
        .bind(open_id)
        .fetch_one(Pool::default())
        .await
        .map_err(|e| {
            error!("插入用户失败: {:?}", e);

            Error::DatabaseInsert(None)
        })
}

pub async fn update(current_user: User, update_request: UpdateRequest) -> Result<User> {
    if update_request.nickname.is_none() && update_request.avatar.is_none() && update_request.slogan.is_none() {
        return Ok(current_user);
    }

    let mut builder = QueryBuilder::<Postgres>::new("update yansongda.user set updated_at = now() ");

    let mut separated = builder.separated(", ");

    if let Some(nickname) = update_request.nickname {
        separated.push("nickname = ").push_bind(nickname);
    }

    if let Some(avatar) = update_request.avatar {
        separated.push("avatar = ").push_bind(avatar);
    }

    if let Some(slogan) = update_request.slogan {
        separated.push("slogan = ").push_bind(slogan);
    }

    separated.push_unseparated("where id = ").push_bind(current_user.id);

    sqlx::query_as(builder.build().sql())
        .execute(Pool::default())
        .await
        .map_err(|e| {
            error!("更新用户失败: {:?}", e);

            Error::DatabaseUpdate(None)
        })?
}
