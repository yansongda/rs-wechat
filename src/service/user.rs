use crate::model::result::Result;
use crate::model::user::{UpdateUser, User};
use crate::repository;

pub async fn detail(open_id: &str) -> Result<User> {
    repository::user::fetch(open_id).await
}

pub async fn update(id: i64, update_user: UpdateUser) -> Result<User> {
    repository::user::update(id, update_user).await
}
