use std::{collections::HashMap, time::Duration};

use axum::{routing::post, Router, Extension, response::Redirect, Form, http::{Request, HeaderValue, HeaderMap}, TypedHeader, headers::{Referer, Header}};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use sqlx::{types::time::OffsetDateTime, Pool, MySql};
use crate::{app_state::AppState, community::{self, structs::Tag}};

use super::structs::Post;

pub async fn create(
    Extension(state): Extension<AppState>, 
    jar: CookieJar,
    header: HeaderMap,
    Form(body): Form<HashMap<String, String>>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);

    let referer = match header.get("referer") {
        Some(url) => url.to_str().unwrap_or("/"),
        None => "/",
    };
    
    if !body.contains_key("community_id") || !body.contains_key("titulo") || !body.contains_key("conteudo") {
        let mut cookie_ob = Cookie::new("error_msg", "Erro ao criar postagem.");
        cookie_ob.set_path("/");
        cookie_ob.set_expires(now);
        let jar = jar.add(cookie_ob);
        
        Err(
            (jar,
            Redirect::to(referer))
        )
    }else{

        let comunity_id = body.get("community_id").unwrap();
        let titulo = body.get("titulo").unwrap();
        let conteudo = body.get("conteudo").unwrap();

        let query_result = sqlx::query("INSERT INTO posts (titulo, conteudo, usuario_id, comunidade_id) VALUES (?, ?, ?, ?)")
            .bind(titulo)
            .bind(conteudo)
            .bind(comunity_id)
            .bind(comunity_id)
            .execute(&state.db)
            .await;

        match query_result {
            Ok(_) => {
                let mut cookie_ob = Cookie::new("success_msg", "Postagem criada com sucesso.");
                cookie_ob.set_path("/");
                cookie_ob.set_expires(now);
                let jar = jar.add(cookie_ob);
                Ok((jar, Redirect::to(referer)))
            },
            Err(_) => {
                let mut cookie_ob = Cookie::new("error_msg", "Erro ao criar postagem.");
                cookie_ob.set_path("/");
                cookie_ob.set_expires(now);
                let jar = jar.add(cookie_ob);
                
                Err(
                    (jar,
                    Redirect::to(referer))
                )
            },
        }

        
    }

}

pub async fn get_post_data(db: &Pool<MySql>, community_id: i64) -> Vec<Post> {
    let result = sqlx::query_as::<_, Post>(
    "SELECT posts.id, posts.titulo, posts.body, usuarios.nome AS user_name, posts.created_at FROM posts JOIN usuarios ON posts.user_id = usuarios.id WHERE posts.comunidade_id = ?")
    .bind(community_id)
    .fetch_all(db)
    .await;

    match result {
        Ok(vec) => vec,
        Err(_) => [].to_vec(),
    }
}

pub fn create_post_router() -> Router {
    Router::new()
        .route("/post", post(create))
        //.layer(middleware::is_logged_in)
}
