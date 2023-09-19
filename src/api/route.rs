use axum::routing::{get, post};
use axum::Router;

pub(crate) fn health() -> Router {
    Router::new().route("/", get(|| async { "success" }))
}

// pub(crate) fn api_v1() -> Router {
//     Router::new()
//         .nest(
//             "/2fa",
//             Router::new()
//                 .route("/init", get(v1::tfa::init))
//                 .route("/generate", get(v1::tfa::generate))
//                 .route("/verify", get(v1::tfa::verify)),
//         )
// }