use axum::{Extension, Json};

use crate::api::response::Resp;
use crate::model::result::Response;
use crate::model::shortlink::{CreateRequest, CreateResponse};
use crate::model::user::CurrentUser;
use crate::service;

pub async fn create(
    Extension(current_user): Extension<CurrentUser>,
    Json(params): Json<CreateRequest>,
) -> Resp<CreateResponse> {
    let link = params.link;
    let shortlink = service::shortlink::create(current_user, link.clone()).await?;

    Ok(Response::success(CreateResponse::from(shortlink)))
}
