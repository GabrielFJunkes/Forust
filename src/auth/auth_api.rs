use axum::{routing::post, Router, Extension, response::{Redirect, IntoResponse}, Form};
use serde::Deserialize;

use crate::app_state::AppState;

#[derive(Deserialize)]
pub struct RegisterBody {
    nome: String,
    email: String,
    senha: String
}

pub async fn register(Extension(state): Extension<AppState>, Form(body): Form<RegisterBody>) -> impl IntoResponse {

    let query_result = sqlx::query("INSERT INTO usuarios (nome, email, senha) VALUES (?, ?, ?)")
        .bind(body.nome)
        .bind(body.email)
        .bind(body.senha)
        .execute(&state.db)
        .await;
    Redirect::to("/login")
}

pub fn create_auth_router() -> Router {
    Router::new()
        .route("/register", post(register))
}
