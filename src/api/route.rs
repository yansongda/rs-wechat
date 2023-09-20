use axum::routing::{get, post};
use axum::Router;

use crate::api::v1;

pub(crate) fn health() -> Router {
    Router::new().route("/", get(|| async { "success" }))
}

pub(crate) fn api_v1() -> Router {
    Router::new().nest(
        "/totp",
        Router::new()
            .route("/all", get(v1::totp::all))
            .route("/detail", get(v1::totp::detail))
            .route("/updateOrCreate", post(v1::totp::update_or_create))
            .route("/delete", post(v1::totp::delete)),
    )
}
