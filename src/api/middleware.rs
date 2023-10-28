use axum::headers::Authorization;
use axum::headers::authorization::Bearer;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::TypedHeader;

use crate::model::result::{Error, Result};
use crate::model::user::{CurrentUser, Model as User};

pub async fn authorization<B>(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, mut request: Request<B>, next: Next<B>) -> Response {
    let user: Result<User> = crate::service::user::detail(auth.token()).await;

    if user.is_err() {
        return Error::AuthorizationNotFound.into_response();
    }

    request.extensions_mut().insert(CurrentUser::from(user.unwrap()));

    next.run(request).await
}
