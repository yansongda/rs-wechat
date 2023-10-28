use crate::model::result::Result;
use crate::model::user::{CreateUser, CurrentUser, Model as User, UpdateRequest};
use crate::repository;
use crate::service::wechat;

pub async fn login(code: &str) -> Result<User> {
    repository::user::insert(CreateUser {
        open_id: wechat::login(code).await?.openid.unwrap(),
    })
    .await
}

pub async fn detail(open_id: &str) -> Result<User> {
    repository::user::find(open_id).await
}

pub async fn update(current_user: CurrentUser, updated: UpdateRequest) -> Result<User> {
    repository::user::update(current_user.into(), updated.into()).await
}
