use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Duration;

use reqwest::{Client, Request};

use crate::model::http::HttpResponse;
use crate::model::result::{Error, Result};

static _CLIENT: OnceLock<Client> = OnceLock::new();

pub async fn request(request: Request) -> Result<HttpResponse> {
    let client = _CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent("yansongda/miniprogram")
            .connect_timeout(Duration::from_secs(1))
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap()
    });

    println!("请求参数: {:?}", request);

    let response = client.execute(request).await.map_err(|_| Error::Http)?;

    let status = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
        .collect::<HashMap<String, String>>();
    let body = response.text().await.map_err(|_| Error::HttpResponse)?;

    println!("请求结果: {:?}", body);

    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}
