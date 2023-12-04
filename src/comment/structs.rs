use serde::Deserialize;


#[derive(Deserialize)]
pub struct CommentForm {
    pub body: String
}

#[derive(Deserialize)]
pub struct CommentFormEdit {
    pub body: String,
    pub post_id: i64
}

#[derive(sqlx::FromRow)]
pub struct CommentRanking {
    pub comentario_id: i64, 
    pub usuario_id: i64, 
    pub gostou: bool
}

#[derive(sqlx::FromRow)]
pub struct CommentEdit {
    pub id: i64, 
    pub usuario_id: i64, 
    pub post_id: i64, 
    pub body: String
}