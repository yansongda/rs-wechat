use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

pub fn error_code_message() -> &'static HashMap<Error, (u16, &'static str)> {
    static INSTANCE: OnceLock<HashMap<Error, (u16, &'static str)>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        HashMap::from([
            // 未知错误
            (Error::Unknown, (9999, "未知错误，请联系管理员")),
            // 参数错误
            (
                Error::Params,
                (2000, "参数错误，请确认您的参数是否符合规范"),
            ),
            // 必要参数不存在
            (
                Error::MissingParams,
                (2001, "必要参数不存在，请确认您的参数是否符合规范"),
            ),
            // 必要参数为空
            (
                Error::ParticipleEmptyParams,
                (2002, "待分词参数为空，请检查"),
            ),
            // 手机号码为空
            (Error::LocationEmptyPhone, (2003, "手机号码为空，请检查")),
            // 手机号格式错误
            (
                Error::LocationWrongPhone,
                (2004, "手机号码格式错误，请检查"),
            ),
            // 未知手机号
            (Error::LocationUnknownPhone, (2005, "手机号归属地未知")),
            // 2fa
            (Error::Tfa, (2006, "2fa 服务错误，请联系管理员")),
            // 2fa 秘钥为空
            (Error::TfaEmptySecret, (2007, "2fa 秘钥为空，请检查")),
            // 2fa 秘钥格式错误
            (Error::TfaWrongSecret, (2008, "2fa 秘钥应该为32位，请检查")),
            // 2fa 服务商为空
            (Error::TfaEmptyIssuer, (2009, "2fa 服务商为空，请检查")),
            // 2fa 用户名为空
            (Error::TfaEmptyUsername, (2010, "2fa 用户名为空，请检查")),
            // 2fa 验证码为空
            (Error::TfaEmptyCode, (2011, "2fa 验证码为空，请检查")),
            // 2fa 验证码格式错误
            (Error::TfaWrongCode, (2012, "2fa 验证码格式错误，请检查")),
            // 数据库错误
            (Error::Database, (5000, "发生了一些问题，请联系管理员")),
        ])
    })
}

#[derive(Serialize, Deserialize)]
pub struct Response<D: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<D>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Error {
    Unknown,
    Params,
    MissingParams,
    ParticipleEmptyParams,
    LocationEmptyPhone,
    LocationWrongPhone,
    LocationUnknownPhone,
    Tfa,
    TfaEmptySecret,
    TfaWrongSecret,
    TfaEmptyIssuer,
    TfaEmptyUsername,
    TfaEmptyCode,
    TfaWrongCode,
    Database,
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