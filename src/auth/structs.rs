use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct RegisterBody {
    pub nome: String,
    pub email: String,
    pub senha: String
}

#[derive(Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub senha: String
}

#[derive(sqlx::FromRow)]
pub struct UserLogin {
    pub id: i64,
    pub nome: String,
    pub email: String,
    pub senha: String
}

#[derive(sqlx::FromRow)]
pub struct UserName {
    pub nome: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserJWT {
    pub exp: i64,
    pub id: i64,
    pub nome: String,
    pub email: String
}

