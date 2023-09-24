use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Error {
    Unknown,
    Params,
    UserNotFound,
    Database,
    Insert,
}

pub fn error_code_message() -> &'static HashMap<Error, (u16, &'static str)> {
    static INSTANCE: OnceLock<HashMap<Error, (u16, &'static str)>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        HashMap::from([
            (Error::Unknown, (9999, "未知错误，请联系管理员")),
            (Error::Params, (2000, "参数错误，请确认您的参数是否符合规范")),
            (Error::UserNotFound, (2001, "用户未找到")),
            (Error::Database, (5000, "发生了一些问题，请联系管理员")),
            (Error::Insert, (5001, "保存数据出现了一些问题，请联系管理员")),
        ])
    })
}

#[derive(Serialize, Deserialize)]
pub struct Response<D: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<D>,
}

impl<D: Serialize> Response<D> {
    pub fn new(code: Option<u16>, message: Option<String>, data: Option<D>) -> Self {
        Response {
            code: code.unwrap_or(0),
            message: message.unwrap_or_else(|| "success".to_string()),
            data,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}