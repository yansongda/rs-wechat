use crate::model::result::Result;
use crate::model::user::{CreateUser, Model as User, UpdateRequest};
use crate::repository;

use crate::service::wechat;

pub async fn login(code: &str) -> Result<User> {
    repository::user::create(CreateUser {
        open_id: wechat::login(code).await?.openid.unwrap(),
    })
    .await
}

pub async fn detail(open_id: &str) -> Result<User> {
    repository::user::find_one(open_id).await
}

pub async fn update(open_id: &str, updated: UpdateRequest) -> Result<User> {
    repository::user::find_one(open_id).await
}
