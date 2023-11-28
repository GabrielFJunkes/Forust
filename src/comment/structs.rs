use serde::Deserialize;


#[derive(Deserialize)]
pub struct CommentForm {
    pub body: String
}

#[derive(sqlx::FromRow)]
pub struct CommentRanking {
    pub comentario_id: i64, 
    pub usuario_id: i64, 
    pub gostou: bool
}