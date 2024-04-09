use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShortUrl {
    pub id: i64,
    pub user_id: i64,
    pub short: String,
    pub link: String,
    pub visit: i64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
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
                Config::get::<String>("shortlink.domain"),
                model.short
            ),
        }
    }
}

#[derive(Debug)]
pub struct CreateShortlink {
    pub user_id: i64,
    pub link: String,
    pub short: String,
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
                Config::get::<String>("shortlink.domain"),
                model.short
            ),
        }
    }
}
