use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProfileBody {
    pub nome: String,
    pub email: String,
    pub senha: String
}


#[derive(sqlx::FromRow, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub nome: String
}