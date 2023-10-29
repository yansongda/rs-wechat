use axum::{Extension, Json};
use crate::model::totp::DetailRequest;
use crate::model::user::CurrentUser;

pub async fn all(Extension(current_user): Extension<CurrentUser>) {

}

pub async fn detail(Extension(current_user): Extension<CurrentUser>, Json(params): Json<DetailRequest>) {

}

pub async fn update_or_create(Extension(current_user): Extension<CurrentUser>) {}

pub async fn delete(Extension(current_user): Extension<CurrentUser>) {}
