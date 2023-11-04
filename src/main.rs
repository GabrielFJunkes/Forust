use axum::{routing::get, Router, http::{StatusCode, Uri}, Extension};
use maud::{html, PreEscaped};
use sqlx::mysql::MySqlPoolOptions;

use static_rust::{home::home_page, auth::{login_page, regiter_page, auth_api::create_auth_router}, app_state::AppState};

async fn fallback(uri: Uri) -> (StatusCode, PreEscaped<String>) {
    (StatusCode::NOT_FOUND, html!(
        h1 { "Rota " (uri) " inválida." }
    ))
}

#[tokio::main]
async fn main() {

    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://root:password@localhost/forust")
        .await
    {
        Ok(pool) => {
            println!("Banco de dados conectado com sucesso");
            let _ = sqlx::migrate!()
            .run(&pool)
            .await;
            pool
        }
        Err(err) => {
            println!("Falha ao conectar ao banco de dados: {:?}", err);
            std::process::exit(1);
        }
    };

    let state = AppState {
        db: pool,
    };

    let app = Router::new()
        .route("/",get(home_page))
        .route("/login",get(login_page))
        .route("/register",get(regiter_page))
        .nest("/api", create_auth_router())
        .layer(Extension(state))
        .fallback(fallback);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}