use totp_rs::{Algorithm, Secret, TOTP};
use tracing::error;

use crate::model::result::{Error, Result};
use crate::model::totp::{CreateTotp, Totp, UpdateTotp};
use crate::model::user::User;
use crate::repository;
use crate::request::totp::DetailResponse;

pub async fn all(current_user: User) -> Result<Vec<DetailResponse>> {
    let totp = repository::totp::all(current_user).await?;

    Ok(totp.into_iter().map(|t| t.into()).collect())
}

pub async fn detail(current_user: User, id: i64) -> Result<DetailResponse> {
    let totp = repository::totp::fetch(id).await?;

    if current_user.id != totp.user_id {
        return Err(Error::TotpNotFound(None));
    }

    Ok(totp.into())
}

pub async fn create(current_user: User, uri: String) -> Result<()> {
    let totp = TOTP::from_url_unchecked(uri.as_str()).map_err(|e| {
        error!("TOTP 链接解析失败: {}", e);

        Error::TotpParse(None)
    })?;

    repository::totp::insert(CreateTotp {
        user_id: current_user.id,
        username: totp.account_name,
        issuer: totp.issuer,
        period: totp.step as i64,
        secret: Secret::Raw(totp.secret).to_encoded().to_string(),
    })
    .await?;

    Ok(())
}

pub async fn update(current_user: User, params: UpdateTotp) -> Result<()> {
    let totp = repository::totp::fetch(params.id).await?;

    if current_user.id != totp.user_id {
        return Err(Error::TotpNotFound(None));
    }

    repository::totp::update(params).await?;

    Ok(())
}

pub async fn delete(current_user: User, id: i64) -> Result<()> {
    let totp = repository::totp::fetch(id).await?;

    if current_user.id != totp.user_id {
        return Err(Error::TotpNotFound(None));
    }

    repository::totp::delete(totp).await?;

    Ok(())
}

pub fn generate_code(totp: Totp) -> String {
    let config = totp.config.unwrap_or_default();

    let totp = TOTP::new_unchecked(
        Algorithm::SHA1,
        6,
        1,
        config.period as u64,
        Secret::Encoded(totp.secret).to_bytes().unwrap(),
        totp.issuer,
        totp.username,
    );

    totp.generate_current().unwrap()
}
