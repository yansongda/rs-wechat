use crate::model::result::Result;
use crate::model::shortlink::{CreateShortlink, Model as Shortlink};
use crate::model::user::CurrentUser;
use crate::repository;
use fasthash::murmur3;

pub async fn create(current_user: CurrentUser, link: String) -> Result<Shortlink> {
    let short = base62::encode(murmur3::hash32(link.as_bytes()));

    repository::shortlink::insert(CreateShortlink {
        user_id: current_user.id,
        link: link.clone(),
        short,
    })
    .await
}
