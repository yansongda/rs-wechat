use crate::model::result::Result;
use crate::model::short_url::{CreateShortlink, ShortUrl};
use crate::repository;
use fasthash::murmur3;

pub async fn create(link: &str) -> Result<ShortUrl> {
    let short = base62::encode(murmur3::hash32(link.as_bytes()));

    let result = repository::short_url::find(&short).await;
    if result.is_ok() {
        return result;
    }

    repository::short_url::insert(CreateShortlink {
        link: link.to_string(),
        short,
    })
    .await
}

pub async fn detail(link: &str) -> Result<ShortUrl> {
    let result = repository::short_url::find(link).await;

    if result.is_ok() {
        repository::short_url::update_count(result.clone().unwrap()).await;
    }

    result
}
