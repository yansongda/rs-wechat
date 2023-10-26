use reqwest::{Method, Request, Url};

use crate::config::Config;
use crate::model::result::{Error, Result};
use crate::model::wechat::LoginResponse;
use crate::util::http;

pub async fn login(code: &str) -> Result<LoginResponse> {
    let url = format!("{}/sns/jscode2session", Config::get::<String>("wechat.url"));

    let query = [
        ("appid", Config::get::<String>("wechat.app_id")),
        ("secret", Config::get::<String>("wechat.app_secret")),
        ("js_code", code.to_string()),
        ("grant_type", "authorization_code".to_string()),
    ];

    let response = http::request(Request::new(
        Method::GET,
        Url::parse_with_params(url.as_str(), query).unwrap(),
    ))
    .await
    .map_err(|e| match e {
        Error::Http => Error::WechatHttp,
        Error::HttpResponse => Error::WechatHttpResponse,
        _ => Error::Http,
    })?;

    let result: LoginResponse =
        serde_json::from_str(response.body.as_str()).map_err(|_| Error::WechatHttpResponseParse)?;

    if result.errcode != 0 {
        println!("微信 API 结果出错: {:?}", result);

        return Err(Error::WechatHttpResponse);
    }

    Ok(result)
}
