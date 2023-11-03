use axum::{routing::post, Router, http::{StatusCode, Uri}, extract::State, body::Body, Json, Extension, response::{Redirect, IntoResponse}, Form};
use serde::{Serialize, Deserialize};
use sqlx::{Pool, MySql};

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
    println!("{:?}", query_result);
    Redirect::permanent("/login")
}

pub fn create_auth_router() -> Router {
    Router::new()
        .route("/register", post(register))
}
