use axum::routing::{get, post};
use axum::{middleware, Router};
use tower::ServiceBuilder;

use crate::api::middleware::authorization;
use crate::api::v1;

pub fn api_v1() -> Router {
    let unauthorized = Router::new()
        .route("/users/login", post(v1::users::login))
        .route("/shortlink/redirect/:short", get(v1::shortlink::redirect));

    let authorized = Router::new()
        .nest(
            "/users",
            Router::new()
                .route("/detail", post(v1::users::detail))
                .route("/update", post(v1::users::update)),
        )
        .nest(
            "/totp",
            Router::new()
                .route("/all", post(v1::totp::all))
                .route("/detail", post(v1::totp::detail))
                .route("/create", post(v1::totp::create))
                .route("/update", post(v1::totp::update))
                .route("/delete", post(v1::totp::delete)),
        )
        .nest(
            "/shortlink",
            Router::new()
                .route("/create", post(v1::shortlink::create))
                .route("/detail", post(v1::shortlink::detail)),
        )
        .layer(ServiceBuilder::new().layer(middleware::from_fn(authorization)));

    authorized.merge(unauthorized)
}
