use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Post {
    pub id: i64,
    pub titulo: String,
    pub body: String,
    pub user_name: String,
    pub community_name: String,
    pub tag_name: Option<String>,
    pub created_at: OffsetDateTime
}

#[derive(Deserialize)]
pub struct PostBody {
    pub titulo: String,
    pub body: String,
    pub community_id: i64,
    pub tag_id: String
}