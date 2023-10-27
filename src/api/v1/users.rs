use axum::extract::Json;
use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::TypedHeader;

use crate::api::response::Resp;
use crate::model::result::Response;
use crate::model::user::{DetailResponse, LoginRequest, LoginResponse, Model as User, UpdateRequest};
use crate::service;

pub async fn login(Json(params): Json<LoginRequest>) -> Resp<LoginResponse> {
    let user: User = service::user::login(&params.code).await?;

    Ok(Response::success(user.into()))
}

pub async fn detail(TypedHeader(open_id): TypedHeader<Authorization<Bearer>>) -> Resp<DetailResponse> {
    let user: User = service::user::detail(open_id.token()).await?;

    Ok(Response::success(user.into()))
}

pub async fn update(TypedHeader(open_id): TypedHeader<Authorization<Bearer>>, Json(params): Json<UpdateRequest>) -> Resp<()> {
    service::user::update(open_id.token(), params).await?;

    Ok(Response::success(()))
}
