use axum::extract::Json;
use crate::model::result::{Result, Response};
use crate::model::user::{Model as User, LoginRequest};
use crate::service;

pub async fn login(Json(params): Json<LoginRequest>) -> Result<Response<User>> {
    Ok(Response::success(service::user::login(&params.code).await?))
}

pub async fn detail() {}

pub async fn update() {}

