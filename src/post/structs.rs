use std::collections::HashMap;

use serde::Deserialize;
use sqlx::{types::time::OffsetDateTime, Decode};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct PostPreview {
    pub id: i64,
    pub titulo: String,
    pub body: String,
    pub user_name: String,
    pub community_name: String,
    pub tag_name: Option<String>,
    pub created_at: OffsetDateTime
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Post {
    pub id: i64,
    pub titulo: String,
    pub body: String,
    pub user_name: String,
    pub community_name: String,
    pub tag_name: Option<String>,
    pub created_at: OffsetDateTime,
    pub comments: Vec<Comment>,
    pub answers: HashMap<i64, Comment>
}

#[derive(sqlx::FromRow, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Comment {
    pub id: i64,
    pub body: String,
    pub user_name: String,
    pub created_at: OffsetDateTime
}

pub struct CommentView {
    pub body: String,
    pub user_name: String,
    pub create_at: OffsetDateTime,
    pub answers_id_list: Vec<String>
}

#[derive(Deserialize)]
pub struct PostBody {
    pub titulo: String,
    pub body: String,
    pub community_id: i64,
    pub tag_id: String
}