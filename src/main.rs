use axum::{routing::get, Router, http::{StatusCode, Uri}, Extension, middleware};
use maud::{html, Markup};
use sqlx::mysql::MySqlPoolOptions;

use static_rust::{
    home::home_page, 
    profile::profile_page, 
    auth::{
        view::{login_page, regiter_page}, 
        api::create_auth_router
    }, 
    app_state::AppState, 
    community::community_page, post::api::create_post_router, component::{middleware::get_referer, page::is_logged_in}
};

async fn fallback(uri: Uri) -> (StatusCode, Markup) {
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
        db: pool
    };

    let app = Router::new()
        .route_layer(middleware::from_fn(
            |req, next| get_referer(req, next),
        ))
        .route("/",get(home_page))
        .route("/perfil", get(profile_page))
        .route("/f/:name", get(community_page))
        .route("/login",get(login_page))
        .route("/register",get(regiter_page))
        .nest("/api/auth", create_auth_router())
        .nest("/api/post", create_post_router())
        .layer(Extension(state))
        .fallback(fallback);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}