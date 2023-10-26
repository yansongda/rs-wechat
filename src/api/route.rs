use axum::routing::{get, post};
use axum::Router;

use crate::api::v1;

pub fn health() -> Router {
    Router::new().route("/", get(|| async { "success" }))
}

pub fn api_v1() -> Router {
    Router::new()
        .nest(
            "/user",
            Router::new()
                .route("/login", post(v1::user::login))
                .route("/detail", get(v1::user::detail))
                .route("/update", post(v1::user::update)),
        )
        .nest(
            "/totp",
            Router::new()
                .route("/all", get(v1::totp::all))
                .route("/detail", get(v1::totp::detail))
                .route("/updateOrCreate", post(v1::totp::update_or_create))
                .route("/delete", post(v1::totp::delete)),
        )
}
