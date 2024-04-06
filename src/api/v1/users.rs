use axum::Extension;

use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::model::result::Response;
use crate::model::user::User;
use crate::request::user::{DetailResponse, LoginRequest, LoginResponse, UpdateRequest};
use crate::request::Validator;
use crate::service;

pub async fn login(Json(request): Json<LoginRequest>) -> Resp<LoginResponse> {
    request.validate()?;

    let user: User = service::user::login(request.code.unwrap_or_else(|| String::new()).as_str()).await?;

    Ok(Response::success(user.into()))
}

pub async fn detail(Extension(current_user): Extension<User>) -> Resp<DetailResponse> {
    Ok(Response::success(current_user.into()))
}

pub async fn update(
    Extension(current_user): Extension<User>,
    Json(request): Json<UpdateRequest>,
) -> Resp<()> {
    request.validate()?;

    service::user::update(current_user, request).await?;

    Ok(Response::success(()))
}
