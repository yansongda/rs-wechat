use crate::api::extract::Json;
use crate::api::response::Resp;
use crate::model::result::Response;
use crate::request::access_token::{LoginRequest, LoginResponse};
use crate::request::Validator;
use crate::service;

pub async fn login(Json(request): Json<LoginRequest>) -> Resp<LoginResponse> {
    let code = request.validate()?;

    let token = service::access_token::login(code.as_str()).await?;

    Ok(Response::success(token.into()))
}
