use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProfileBody {
    pub nome: String,
    pub email: String,
    pub senha: String
}