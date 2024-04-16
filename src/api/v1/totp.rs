use axum::Extension;

use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::model::access_token::AccessToken;
use crate::model::result::Response;
use crate::request::totp::{
    CreateRequest, DeleteRequest, DetailRequest, DetailResponse, UpdateRequest,
};
use crate::request::Validator;
use crate::service;

pub async fn all(Extension(access_token): Extension<AccessToken>) -> Resp<Vec<DetailResponse>> {
    Ok(Response::success(
        service::totp::all(access_token.user_id).await?,
    ))
}

pub async fn detail(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<DetailRequest>,
) -> Resp<DetailResponse> {
    let id = request.validate()?;

    Ok(Response::success(
        service::totp::detail(access_token.user_id, id).await?,
    ))
}

pub async fn create(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<CreateRequest>,
) -> Resp<()> {
    let uri = request.validate()?;

    service::totp::create(access_token.user_id, uri).await?;

    Ok(Response::success(()))
}

pub async fn update(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<UpdateRequest>,
) -> Resp<()> {
    let params = request.validate()?;

    service::totp::update(access_token.user_id, params).await?;

    Ok(Response::success(()))
}

pub async fn delete(
    Extension(access_token): Extension<AccessToken>,
    Json(request): Json<DeleteRequest>,
) -> Resp<()> {
    let id = request.validate()?;

    service::totp::delete(access_token.user_id, id).await?;

    Ok(Response::success(()))
}
