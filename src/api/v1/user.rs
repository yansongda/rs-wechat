use crate::model::result::{Response, Result};
use crate::model::user::{LoginRequest, Model as User};
use crate::service;
use axum::extract::Json;

pub async fn login(Json(params): Json<LoginRequest>) -> Result<Response<User>> {
    Ok(Response::success(service::user::login(&params.code).await?))
}

pub async fn detail() {}

pub async fn update() {}
