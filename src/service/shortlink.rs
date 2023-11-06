use crate::model::result::Result;
use crate::model::shortlink::{CreateShortlink, Model as Shortlink};
use crate::model::user::CurrentUser;
use crate::repository;
use fasthash::murmur3;

pub async fn create(current_user: CurrentUser, link: &str) -> Result<Shortlink> {
    let short = base62::encode(murmur3::hash32(link.as_bytes()));

    let result = repository::shortlink::find(&short).await;
    if result.is_ok() {
        return result;
    }

    repository::shortlink::insert(CreateShortlink {
        user_id: current_user.id,
        link: link.to_string(),
        short,
    })
    .await
}

pub async fn detail(link: &str) -> Result<Shortlink> {
    let result = repository::shortlink::find(link).await;

    if result.is_ok() {
        repository::shortlink::update_count(result.clone().unwrap()).await;
    }

    result
}
