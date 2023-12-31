use serde::Deserialize;

#[derive(sqlx::FromRow, Clone)]
pub struct Tag {
    pub id: i64,
    pub nome: String,
    pub status: bool
}

#[derive(Deserialize)]
pub struct CommunityParams {
    pub tag: Option<String>,
    pub search: Option<String>,
    pub filter: Option<String>
}

#[derive(sqlx::FromRow)]
pub struct Community {
    pub id: i64,
    pub nome: String,
    pub desc: String,
    pub tags: Vec<Tag>
}

#[derive(sqlx::FromRow)]
pub struct AdminsCount {
    pub count: i64,
}



#[derive(sqlx::FromRow, Debug)]
pub struct TopCommunity {
    pub id: i64,
    pub nome: String,
    pub count: i64
}

#[derive(sqlx::FromRow)]
pub struct CommunityData {
    pub id: i64,
    pub nome: String,
    pub desc: String
}

#[derive(sqlx::FromRow, Clone)]
pub struct FollowedCommunityData {
    pub id: i64,
    pub nome: String,
    pub desc: String,
    pub admin: bool
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub nome: String,
    pub admin: bool
}

#[derive(sqlx::FromRow, Clone)]
pub struct Follow {
    pub usuario_id: i64,
    pub comunidade_id: i64,
    pub admin: bool
}

#[derive(Clone, Deserialize)]
pub struct CommunityBody {
    pub nome: String,
    pub desc: String
}

#[derive(Clone, Deserialize)]
pub struct CommunityBodyEdit {
    pub desc: String
}

#[derive(Clone, Deserialize)]
pub struct TagBody {
    pub nome: String
}

#[derive(Clone, Deserialize)]
pub struct TagBodyWithName {
    pub nome: String,
    pub nome_comunidade: String
}
