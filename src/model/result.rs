use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

static G_ERROR_CODE_MESSAGE: OnceLock<HashMap<Error, (u16, &'static str)>> = OnceLock::new();

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Error {
    Unknown,
    Params,
    UserNotFound,
    Database,
    Insert,
    Update,
    Http,
    HttpResponse,
    WechatHttp,
    WechatHttpResponse,
    WechatHttpResponseParse,
}

#[derive(Serialize, Deserialize)]
pub struct Response<D: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<D>,
}

pub type Result<D> = std::result::Result<D, Error>;

impl Error {
    pub fn code_message(&self) -> (u16, &'static str) {
        let messages = G_ERROR_CODE_MESSAGE.get_or_init(|| {
            HashMap::from([
                (Self::Unknown, (9999, "未知错误，请联系管理员")),
                (Self::Params, (2000, "参数错误，请确认您的参数是否符合规范")),
                (Self::UserNotFound, (2001, "用户未找到")),
                (Self::Database, (5000, "发生了一些问题，请联系管理员")),
                (Self::Insert, (5001, "保存数据出现了一些问题，请联系管理员")),
                (Self::Update, (5002, "更新数据出现了一些问题，请联系管理员")),
                (Self::Http, (9800, "第三方 API 请求出错，请联系管理员")),
                (
                    Self::HttpResponse,
                    (9801, "第三方 API 响应出错，请联系管理员"),
                ),
                (Self::WechatHttp, (9802, "微信 API 请求出错，请联系管理员")),
                (
                    Self::WechatHttpResponseParse,
                    (9803, "微信 API 解析出错，请联系管理员"),
                ),
                (
                    Self::WechatHttpResponse,
                    (9804, "微信 API 结果出错，请联系管理员"),
                ),
            ])
        });

        messages
            .get(self)
            .unwrap_or_else(|| messages.get(&Self::Unknown).unwrap())
            .to_owned()
    }
}

impl<D: Serialize> Response<D> {
    pub fn new(code: Option<u16>, message: Option<String>, data: Option<D>) -> Self {
        Response {
            code: code.unwrap_or(0),
            message: message.unwrap_or_else(|| "success".to_string()),
            data,
        }
    }

    pub fn success(data: D) -> Self {
        Response::new(None, None, Some(data))
    }

    pub fn error(error: Error) -> Self {
        let (code, message) = error.code_message();

        Response::new(Some(code), Some(message.to_string()), None)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
