use axum::extract::Path;
use axum::response::Redirect;

use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::model::result::{Response, Result};
use crate::request::short_url::{CreateRequest, CreateResponse, DetailRequest, DetailResponse};
use crate::request::Validator;
use crate::service;

pub async fn create(Json(request): Json<CreateRequest>) -> Resp<CreateResponse> {
    let link = request.validate()?;

    let shortlink = service::short_url::create(&link).await?;

    Ok(Response::success(CreateResponse::from(shortlink)))
}

pub async fn detail(Json(request): Json<DetailRequest>) -> Resp<DetailResponse> {
    let short = request.validate()?;

    let shortlink = service::short_url::detail(&short).await?;

    Ok(Response::success(DetailResponse::from(shortlink)))
}

pub async fn redirect(Path(short): Path<String>) -> Result<Redirect> {
    let shortlink = service::short_url::detail(short.as_str()).await?;

    Ok(Redirect::temporary(shortlink.link.as_str()))
}
