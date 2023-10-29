use axum::{middleware, Router};
use axum::routing::{get, post};
use tower::ServiceBuilder;

use crate::api::middleware::authorization;
use crate::api::v1;

pub fn health() -> Router {
    Router::new().route("/", get(|| async { "success" }))
}

pub fn api_v1() -> Router {
    let unauthorized = Router::new()
        .route("/users/login", post(v1::users::login));

    let users =  Router::new()
        .nest(
            "/users",
            Router::new()
                .route("/detail", get(v1::users::detail))
                .route("/update", post(v1::users::update)),
        )
        .layer(ServiceBuilder::new().layer(middleware::from_fn(authorization)));

    let totp = Router::new()
        .nest(
            "/totp",
            Router::new()
                .route("/all", get(v1::totp::all))
                .route("/detail", get(v1::totp::detail))
                .route("/create", post(v1::totp::create))
                .route("/update", post(v1::totp::update))
                .route("/delete", post(v1::totp::delete)),
        )
        .layer(ServiceBuilder::new().layer(middleware::from_fn(authorization)));

    unauthorized.merge(users).merge(totp)
}
