use totp_rs::{Algorithm, Secret, TOTP};
use tracing::error;

use crate::model::result::{Error, Result};
use crate::model::totp::{CreateTotp, DetailResponse, Model as Totp, UpdateTotp};
use crate::model::user::{CurrentUser, Model as User};
use crate::repository;

pub async fn all(user: User) -> Result<Vec<DetailResponse>> {
    let totp = repository::totp::all(user).await?;

    Ok(totp.into_iter().map(|t| t.into()).collect())
}

pub async fn detail(current_user: CurrentUser, id: i64) -> Result<DetailResponse> {
    let t = repository::totp::find(id).await?;

    if current_user.id != t.user_id {
        return Err(Error::TotpNotFound(None));
    }

    Ok(t.into())
}

pub async fn create(current_user: CurrentUser, uri: String) -> Result<()> {
    let t = TOTP::from_url_unchecked(uri.as_str()).map_err(|e| {
        error!("TOTP 链接解析失败: {}", e);

        Error::TotpParse(None)
    })?;

    repository::totp::insert(CreateTotp {
        user_id: current_user.id,
        username: t.account_name,
        issuer: t.issuer,
        period: t.step,
        secret: Secret::Raw(t.secret).to_encoded().to_string(),
    })
    .await?;

    Ok(())
}

pub async fn update(current_user: CurrentUser, params: UpdateTotp) -> Result<()> {
    let model = repository::totp::find(params.id).await?;

    if current_user.id != model.user_id {
        return Err(Error::TotpNotFound(None));
    }

    repository::totp::update(model, params).await?;

    Ok(())
}

pub async fn delete(current_user: CurrentUser, id: i64) -> Result<()> {
    let model = repository::totp::find(id).await?;

    if current_user.id != model.user_id {
        return Err(Error::TotpNotFound(None));
    }

    repository::totp::delete(model).await?;

    Ok(())
}

pub fn generate_code(totp: Totp) -> String {
    let totp = TOTP::new_unchecked(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(totp.secret).to_bytes().unwrap(),
        totp.issuer,
        totp.username,
    );

    totp.generate_current().unwrap()
}
