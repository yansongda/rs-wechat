use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct DetailRequest {
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct CreateRequest {
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct  UpdateRequest {
    pub issuer: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct DeleteRequest {
    pub id: i32,
}

#[derive(Serialize, Debug)]
pub struct AllResponse {
    pub results: Vec<DetailResponse>,
}

#[derive(Serialize, Debug)]
pub struct DetailResponse {
    pub id: i32,
    pub issuer: String,
    pub username: String,
    pub code: String,
}
