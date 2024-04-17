use crate::model::access_token::AccessToken;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::model::result::{Error, Result};

pub async fn authorization(mut request: Request, next: Next) -> Response {
    let authorization = request.headers().get("Authorization");

    if authorization.is_none() {
        return Error::AuthorizationMissing(None).into_response();
    }

    let auth = authorization
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "");

    let access_token: Result<AccessToken> =
        crate::repository::access_token::fetch(auth.as_str()).await;

    if access_token.is_err() {
        return Error::AuthorizationNotFound(None).into_response();
    }

    request.extensions_mut().insert(access_token.unwrap());

    next.run(request).await
}
