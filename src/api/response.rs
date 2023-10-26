use axum::response::IntoResponse;
use serde::Serialize;

use crate::model::result::{Error, Response};

impl<D: Serialize> IntoResponse for Response<D> {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap();

        axum::response::Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
            .into_response()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Response::<String>::error(self).into_response()
    }
}
