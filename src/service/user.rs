use crate::http;

pub async fn login(code: &str) {
    http::wechat::login(code)
}
