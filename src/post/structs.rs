use sqlx::types::time::OffsetDateTime;

#[derive(sqlx::FromRow, Clone)]
pub struct Post {
    pub id: i64,
    pub titulo: String,
    pub body: String,
    pub user_name: String,
    pub created_at: OffsetDateTime
}