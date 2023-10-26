use crate::model::result::Result;
use crate::model::user::Model as User;
use crate::repository;

use crate::service::wechat;

pub async fn login(code: &str) -> Result<User> {
    Ok(repository::user::create(User {
        id: 0,
        open_id: wechat::login(code).await?.openid.unwrap(),
        avatar: None,
        nickname: None,
        slogan: None,
        created_at: None,
        updated_at: None,
    }).await?)
}
