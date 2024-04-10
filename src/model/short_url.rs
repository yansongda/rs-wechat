use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShortUrl {
    pub id: i64,
    pub short: String,
    pub link: String,
    pub visit: i64,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug)]
pub struct CreateShortlink {
    pub link: String,
    pub short: String,
}
