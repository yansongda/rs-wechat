use serde::{Deserialize, Serialize};
use url::Url;

use crate::config::Config;
use crate::model::result::Error;
use crate::model::short_url::ShortUrl;
use crate::request::Validator;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRequest {
    pub link: Option<String>,
}

impl Validator for CreateRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.link.is_none() {
            return Err(Error::Params(Some("URL 链接不能为空")));
        }

        Url::parse(self.link.clone().unwrap().as_str())
            .map_err(|_| Error::Params(Some("URL 链接格式不正确")))?;

        Ok(self.link.clone().unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    pub link: String,
    pub short: String,
}

impl From<ShortUrl> for CreateResponse {
    fn from(model: ShortUrl) -> Self {
        Self {
            link: model.link,
            short: format!(
                "{}/{}",
                Config::get::<String>("short_url.domain"),
                model.short
            ),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DetailRequest {
    pub short: Option<String>,
}

impl Validator for DetailRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.short.is_none() {
            return Err(Error::Params(Some("短链不能为空")));
        }

        Ok(self.short.clone().unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub link: String,
    pub short: String,
}

impl From<ShortUrl> for DetailResponse {
    fn from(model: ShortUrl) -> Self {
        Self {
            link: model.link,
            short: format!(
                "{}/{}",
                Config::get::<String>("short_url.domain"),
                model.short
            ),
        }
    }
}
