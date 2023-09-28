use crate::model::result;
use axum::response::IntoResponse;
use serde::Serialize;

pub(super) struct Response<D: Serialize>(result::Response<D>);
pub(super) struct Error(result::Error);

impl<D: Serialize> Response<D> {
    pub fn success(data: D) -> Self {
        Response(result::Response::new(None, None, Some(data)))
    }

    pub fn error(error: result::Error) -> Self {
        let (code, message) = error.code_message();

        Response(result::Response::new(
            Some(code),
            Some(message.to_string()),
            None,
        ))
    }
}

impl<D: Serialize> IntoResponse for Response<D> {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self.0).unwrap();

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
        Response::<String>::error(self.0).into_response()
    }
}

impl From<result::Error> for Error {
    fn from(error: result::Error) -> Self {
        Self(error)
    }
}
