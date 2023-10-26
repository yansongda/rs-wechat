use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub session_key: Option<String>,
    pub unionid: Option<String>,
    pub errmsg: String,
    pub openid: Option<String>,
    pub errcode: i32,
}
