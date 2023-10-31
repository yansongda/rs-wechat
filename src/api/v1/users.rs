use axum::extract::Json;
use axum::Extension;

use crate::api::response::Resp;
use crate::model::result::Response;
use crate::model::user::{
    CurrentUser, DetailResponse, LoginRequest, LoginResponse, Model as User, UpdateRequest,
};
use crate::service;

pub async fn login(Json(params): Json<LoginRequest>) -> Resp<LoginResponse> {
    let user: User = service::user::login(&params.code).await?;

    Ok(Response::success(user.into()))
}

pub async fn detail(Extension(current_user): Extension<CurrentUser>) -> Resp<DetailResponse> {
    Ok(Response::success(current_user.into()))
}

pub async fn update(
    Extension(current_user): Extension<CurrentUser>,
    Json(params): Json<UpdateRequest>,
) -> Resp<()> {
    service::user::update(current_user, params).await?;

    Ok(Response::success(()))
}