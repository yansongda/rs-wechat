use crate::model::access_token::{AccessToken, AccessTokenData};
use crate::model::result::{Error, Result};
use crate::repository;
use crate::service::wechat;

pub async fn login(code: &str) -> Result<AccessToken> {
    let wechat_response = wechat::login(code).await?;
    let open_id = wechat_response.openid.unwrap();
    let session_key = wechat_response.session_key.unwrap();
    let user_id = get_login_user_id(open_id.as_str()).await?;

    let exist = repository::access_token::fetch_by_user_id(user_id).await;

    if exist.is_ok() {
        return repository::access_token::update(
            exist.unwrap().id,
            AccessTokenData {
                open_id,
                session_key,
            },
        )
        .await;
    }

    match exist.unwrap_err() {
        Error::AccessTokenNotFound(_) => {
            repository::access_token::insert(
                user_id,
                AccessTokenData {
                    open_id,
                    session_key,
                },
            )
            .await
        }
        e => Err(e),
    }
}

async fn get_login_user_id(open_id: &str) -> Result<i64> {
    let result = repository::user::fetch(open_id).await;

    if let Ok(user) = result {
        return Ok(user.id);
    }

    match result.unwrap_err() {
        Error::UserNotFound(_) => repository::user::insert(open_id).await.map(|u| u.id),
        e => Err(e),
    }
}
