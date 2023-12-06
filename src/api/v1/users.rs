use axum::Extension;

use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::model::result::Response;
use crate::model::user::{CurrentUser, DetailResponse, LoginResponse, Model as User};
use crate::request::user::{LoginRequest, UpdateRequest};
use crate::request::Validator;
use crate::service;

pub async fn login(Json(request): Json<LoginRequest>) -> Resp<LoginResponse> {
    let params = request.validate()?;

    let user: User = service::user::login(params.code.as_str()).await?;

    Ok(Response::success(user.into()))
}

pub async fn detail(Extension(current_user): Extension<CurrentUser>) -> Resp<DetailResponse> {
    Ok(Response::success(current_user.into()))
}

pub async fn update(
    Extension(current_user): Extension<CurrentUser>,
    Json(request): Json<UpdateRequest>,
) -> Resp<()> {
    let params = request.validate()?;

    service::user::update(current_user, params).await?;

    Ok(Response::success(()))
}
