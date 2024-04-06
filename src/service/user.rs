use crate::model::result::Result;
use crate::model::user::User;
use crate::repository;
use crate::request::user::UpdateRequest;
use crate::service::wechat;

pub async fn login(code: &str) -> Result<User> {
    let open_id = wechat::login(code).await?.openid.unwrap();

    let user = repository::user::fetch_optional(&open_id).await;

    if user.is_ok() {
        return user;
    }

    repository::user::insert(&open_id).await
}

pub async fn detail(open_id: &str) -> Result<User> {
    repository::user::fetch_optional(open_id).await
}

pub async fn update(current_user: User, update_request: UpdateRequest) -> Result<User> {
    repository::user::update(current_user, update_request).await
}
