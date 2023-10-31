use totp_rs::{Algorithm, Secret, TOTP};

use crate::model::result::{Error, Result};
use crate::model::totp::{CreateTotp, DetailResponse, Model as Totp, UpdateRequest};
use crate::model::user::{CurrentUser, Model as User};
use crate::repository::totp;

pub async fn all(user: User) -> Result<Vec<DetailResponse>> {
    let totp = totp::all(user).await?;

    Ok(totp.into_iter().map(|t| t.into()).collect())
}

pub async fn detail(current_user: CurrentUser, id: i64) -> Result<DetailResponse> {
    let t = totp::find(id).await?;

    if current_user.id != t.id {
        return Err(Error::TotpNotFound);
    }

    Ok(t.into())
}

pub async fn create(current_user: CurrentUser, uri: String) -> Result<()> {
    let t = TOTP::from_url_unchecked(uri.as_str()).map_err(|e| {
        println!("totp parse error: {}", e);

        Error::TotpParse
    })?;

    totp::insert(CreateTotp {
        user_id: current_user.id,
        username: t.account_name,
        issuer: t.issuer,
        secret: Secret::Raw(t.secret).to_encoded().to_string(),
    })
    .await?;

    Ok(())
}

pub async fn update(current_user: CurrentUser, params: UpdateRequest) -> Result<()> {
    let model = totp::find(params.id).await?;

    if current_user.id != model.user_id {
        return Err(Error::TotpNotFound);
    }

    totp::update(model, params.into()).await?;

    Ok(())
}

pub async fn delete(current_user: CurrentUser, id: i64) -> Result<()> {
    let model = totp::find(id).await?;

    if current_user.id != model.user_id {
        return Err(Error::TotpNotFound);
    }

    totp::delete(model).await?;

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
