use axum::extract::Path;
use axum::response::Redirect;
use axum::Extension;

use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::model::result::{Response, Result};
use crate::model::shortlink::{CreateResponse, DetailResponse};
use crate::model::user::CurrentUser;
use crate::request::shortlink::{CreateRequest, DetailRequest};
use crate::request::Validator;
use crate::service;

pub async fn create(
    Extension(current_user): Extension<CurrentUser>,
    Json(request): Json<CreateRequest>,
) -> Resp<CreateResponse> {
    let link = request.validate()?;

    let shortlink = service::shortlink::create(current_user, &link).await?;

    Ok(Response::success(CreateResponse::from(shortlink)))
}

pub async fn detail(Json(request): Json<DetailRequest>) -> Resp<DetailResponse> {
    let short = request.validate()?;

    let shortlink = service::shortlink::detail(&short).await?;

    Ok(Response::success(DetailResponse::from(shortlink)))
}

pub async fn redirect(Path(short): Path<String>) -> Result<Redirect> {
    let shortlink = service::shortlink::detail(short.as_str()).await?;

    Ok(Redirect::temporary(shortlink.link.as_str()))
}
