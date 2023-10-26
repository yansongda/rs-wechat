use crate::model::result::{Response, Result};
use crate::model::user::{LoginRequest, LoginResponse, Model as User};
use crate::service;
use axum::extract::Json;

pub async fn login(Json(params): Json<LoginRequest>) -> Result<Response<LoginResponse>> {
    let user: User = service::user::login(&params.code).await?;

    Ok(Response::success(user.into()))
}

pub async fn detail() {}

pub async fn update() {}
