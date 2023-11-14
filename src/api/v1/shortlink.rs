use axum::extract::Path;
use axum::response::Redirect;
use axum::Extension;
use garde::Validate;

use crate::api::extra::Json;
use crate::api::response::Resp;
use crate::model::result::{Response, Result};
use crate::model::shortlink::{CreateRequest, CreateResponse, DetailRequest, DetailResponse};
use crate::model::user::CurrentUser;
use crate::service;

pub async fn create(
    Extension(current_user): Extension<CurrentUser>,
    Json(params): Json<CreateRequest>,
) -> Resp<CreateResponse> {
    params.validate(&())?;

    let link = params.link;
    let shortlink = service::shortlink::create(current_user, &link).await?;

    Ok(Response::success(CreateResponse::from(shortlink)))
}

pub async fn detail(Json(params): Json<DetailRequest>) -> Resp<DetailResponse> {
    params.validate(&())?;

    let shortlink = service::shortlink::detail(&(params.short)).await?;

    Ok(Response::success(DetailResponse::from(shortlink)))
}

pub async fn redirect(Path(short): Path<String>) -> Result<Redirect> {
    let shortlink = service::shortlink::detail(short.as_str()).await?;

    Ok(Redirect::temporary(shortlink.link.as_str()))
}
