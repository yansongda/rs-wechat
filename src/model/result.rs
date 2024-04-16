use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

pub type Result<D> = std::result::Result<D, Error>;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Error {
    AuthorizationMissing(Option<&'static str>),
    AuthorizationNotFound(Option<&'static str>),
    Params(Option<&'static str>),
    UserNotFound(Option<&'static str>),
    TotpNotFound(Option<&'static str>),
    TotpParse(Option<&'static str>),
    ShortlinkNotFound(Option<&'static str>),
    AccessTokenNotFound(Option<&'static str>),
    Database(Option<&'static str>),
    DatabaseInsert(Option<&'static str>),
    DatabaseUpdate(Option<&'static str>),
    DatabaseDelete(Option<&'static str>),
    Http(Option<&'static str>),
    HttpResponse(Option<&'static str>),
    HttpWechat(Option<&'static str>),
    HttpWechatResponse(Option<&'static str>),
    HttpWechatResponseParse(Option<&'static str>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<D: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<D>,
}

impl Error {
    pub fn get_code_message(&self) -> (u16, &'static str) {
        match self {
            Error::AuthorizationMissing(message) => {
                (1000, message.unwrap_or_else(|| "缺少认证信息，认证失败"))
            }
            Error::AuthorizationNotFound(message) => {
                (1001, message.unwrap_or_else(|| "认证信息不正确，认证失败"))
            }
            Error::Params(message) => (
                2000,
                message.unwrap_or_else(|| "参数错误，请确认您的参数是否符合规范"),
            ),
            Error::UserNotFound(message) => (2001, message.unwrap_or_else(|| "用户未找到")),
            Error::TotpNotFound(message) => (2002, message.unwrap_or_else(|| "TOTP 信息未找到")),
            Error::TotpParse(message) => (2003, message.unwrap_or_else(|| "TOTP 链接解析失败")),
            Error::ShortlinkNotFound(message) => (2004, message.unwrap_or_else(|| "短连接未找到")),
            Error::AccessTokenNotFound(message) => {
                (2005, message.unwrap_or_else(|| "Access Token 未找到"))
            }
            Error::Database(message) => (
                5000,
                message.unwrap_or_else(|| "发生了一些问题，请联系管理员"),
            ),
            Error::DatabaseInsert(message) => (
                5001,
                message.unwrap_or_else(|| "保存数据出现了一些问题，请联系管理员"),
            ),
            Error::DatabaseUpdate(message) => (
                5002,
                message.unwrap_or_else(|| "更新数据出现了一些问题，请联系管理员"),
            ),
            Error::DatabaseDelete(message) => (
                5003,
                message.unwrap_or_else(|| "删除数据出现了一些问题，请联系管理员"),
            ),
            Error::Http(message) => (
                9800,
                message.unwrap_or_else(|| "第三方 API 请求出错，请联系管理员"),
            ),
            Error::HttpResponse(message) => (
                9801,
                message.unwrap_or_else(|| "第三方 API 响应出错，请联系管理员"),
            ),
            Error::HttpWechat(message) => (
                9802,
                message.unwrap_or_else(|| "微信 API 请求出错，请联系管理员"),
            ),
            Error::HttpWechatResponse(message) => (
                9803,
                message.unwrap_or_else(|| "微信 API 解析出错，请联系管理员"),
            ),
            Error::HttpWechatResponseParse(message) => (
                9804,
                message.unwrap_or_else(|| "微信 API 结果出错，请联系管理员"),
            ),
        }
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
        let (code, message) = error.get_code_message();

        Response::new(Some(code), Some(message.to_string()), None)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
