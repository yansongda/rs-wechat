use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

static ERROR_CODE_MESSAGE: OnceLock<HashMap<Error, (u16, &'static str)>> = OnceLock::new();

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Error {
    Unknown,
    Params,
    UserNotFound,
    Database,
    Insert,
}

impl Error {
    pub fn code_message(&self) -> (u16, &'static str) {
        let messages = ERROR_CODE_MESSAGE.get_or_init(|| {
            HashMap::from([
                (Self::Unknown, (9999, "未知错误，请联系管理员")),
                (Self::Params, (2000, "参数错误，请确认您的参数是否符合规范")),
                (Self::UserNotFound, (2001, "用户未找到")),
                (Self::Database, (5000, "发生了一些问题，请联系管理员")),
                (Self::Insert, (5001, "保存数据出现了一些问题，请联系管理员")),
            ])
        });

        messages.get(self).unwrap_or_else(|| messages.get(&Self::Unknown).unwrap()).to_owned()
    }
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