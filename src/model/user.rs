use chrono::NaiveDateTime;

pub struct DetailResponse {
    pub id: i32,
    pub open_id: String,
    pub avatar: String,
    pub nickname: String,
    pub slogan: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
