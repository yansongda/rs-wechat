use axum::Extension;

use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::model::access_token::AccessToken;
use crate::model::result::Response;
use crate::request::user::{DetailResponse, UpdateRequest};
use crate::request::Validator;
use crate::service;

pub async fn detail(Extension(access_token): Extension<AccessToken>) -> Resp<DetailResponse> {
    let user = service::user::detail(access_token.data.open_id.as_str()).await?;

    Ok(Response::success(user.into()))
}

pub async fn update(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<UpdateRequest>,
) -> Resp<()> {
    let params = request.validate()?;

    service::user::update(access_token.user_id, params).await?;

    Ok(Response::success(()))
}
