use axum::Extension;

use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::model::result::Response;
use crate::model::totp::{
    CreateRequest, DeleteRequest, DetailRequest, DetailResponse, UpdateRequest,
};
use crate::model::user::CurrentUser;
use crate::service;

pub async fn all(Extension(current_user): Extension<CurrentUser>) -> Resp<Vec<DetailResponse>> {
    Ok(Response::success(
        service::totp::all(current_user.into()).await?,
    ))
}

pub async fn detail(
    Extension(current_user): Extension<CurrentUser>,
    Json(params): Json<DetailRequest>,
) -> Resp<DetailResponse> {
    Ok(Response::success(
        service::totp::detail(current_user, params.id).await?,
    ))
}

pub async fn create(
    Extension(current_user): Extension<CurrentUser>,
    Json(params): Json<CreateRequest>,
) -> Resp<()> {
    // params.validate()?;

    service::totp::create(current_user, params.uri).await?;

    Ok(Response::success(()))
}

pub async fn update(
    Extension(current_user): Extension<CurrentUser>,
    Json(params): Json<UpdateRequest>,
) -> Resp<()> {
    // params.validate()?;

    service::totp::update(current_user, params).await?;

    Ok(Response::success(()))
}

pub async fn delete(
    Extension(current_user): Extension<CurrentUser>,
    Json(params): Json<DeleteRequest>,
) -> Resp<()> {
    service::totp::delete(current_user, params.id).await?;

    Ok(Response::success(()))
}
