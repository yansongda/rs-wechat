use axum::extract::Json;
use crate::model::user::LoginRequest;

pub async fn login(Json(params): Json<LoginRequest>) {
    println!("code: {}", params.code);
}

pub async fn detail() {}

pub async fn update() {}

