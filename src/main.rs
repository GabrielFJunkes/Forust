use axum::{routing::get, Router, http::{StatusCode, Uri}, Extension, middleware};
use maud::{html, Markup};
use sqlx::mysql::MySqlPoolOptions;

use static_rust::{
    home::home_page, 
    profile::{view::profile_page, api::create_profile_router}, 
    auth::{
        view::{login_page, regiter_page}, 
        api::create_auth_router, middleware::logged_in
    }, 
    app_state::AppState, 
    community::{community_page, api::create_community_router}, post::{api::create_post_router, view::post_page}, component::middleware::get_referer, comment::api::create_comment_router
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

    let api = Router::new()
    .nest("/auth", create_auth_router())
    .nest("/post", create_post_router())
    .nest("/comunidade", create_community_router())
    .nest("/comentario", create_comment_router())
    .nest("/perfil", create_profile_router());

    let app = Router::new()
        .route("/perfil", get(profile_page))
        .route_layer(middleware::from_fn(
            |req, next| logged_in(req, next),
        ))
        .route("/",get(home_page))
        .route("/f/:name", get(community_page))
        .route("/login",get(login_page))
        .route("/register",get(regiter_page))
        .route("/p/:id", get(post_page))
        .nest("/api", api)
        .layer(Extension(state))
        .route_layer(middleware::from_fn(
            |req, next| get_referer(req, next),
        ))
        .fallback(fallback);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}