use axum::routing::{get, post};
use axum::Router;
use crate::api::response::{Error, Response};

use crate::api::v1;
use crate::model::user::Model;
use crate::repository::user;

pub fn health() -> Router {
    Router::new().route("/", get(|| async { "success" }))
}

pub fn test() -> Router {
    Router::new().route("/", get(|| async {
        let user = user::find_one(String::from("test")).await?;

        Ok::<Response<Model>, Error>(Response::success(user))
    }))
}

pub fn api_v1() -> Router {
    Router::new().nest(
        "/totp",
        Router::new()
            .route("/all", get(v1::totp::all))
            .route("/detail", get(v1::totp::detail))
            .route("/updateOrCreate", post(v1::totp::update_or_create))
            .route("/delete", post(v1::totp::delete)),
    )
}
