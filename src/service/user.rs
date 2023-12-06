use crate::model::result::Result;
use crate::model::user::{CreateUser, CurrentUser, Model as User, UpdateParams};
use crate::repository;
use crate::service::wechat;

pub async fn login(code: &str) -> Result<User> {
    let open_id = wechat::login(code).await?.openid.unwrap();

    let user = repository::user::find(&open_id).await;

    if user.is_ok() {
        return user;
    }

    repository::user::insert(CreateUser { open_id }).await
}

pub async fn detail(open_id: &str) -> Result<User> {
    repository::user::find(open_id).await
}

pub async fn update(current_user: CurrentUser, update_params: UpdateParams) -> Result<User> {
    repository::user::update(current_user.into(), update_params.into()).await
}
