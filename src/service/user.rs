use crate::model::result::Result;
use crate::model::user::{UpdateUser, User};
use crate::repository;

pub async fn detail_by_open_id(open_id: &str) -> Result<User> {
    repository::user::fetch_by_open_id(open_id).await
}

pub async fn update(id: i64, update_user: UpdateUser) -> Result<User> {
    repository::user::update(id, update_user).await
}
