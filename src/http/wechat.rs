use std::collections::HashMap;
use std::sync::OnceLock;
use reqwest::Client;
use crate::config::Config;

static _CLIENT: Client = reqwest::Client::new();

pub async fn login(code: &str) {
    let json = HashMap::from([
        ("appid", Config::get::<String>("wechat.app_id")),
        ("secret", Config::get::<String>("wechat.app_secret")),
        ("js_code", code.to_string()),
        ("grant_type", "authorization_code".to_string())
    ]);

    let client = reqwest::Client::new();
    let res = client.post()
        .json(&json)
        .send()
        .await?;
}
