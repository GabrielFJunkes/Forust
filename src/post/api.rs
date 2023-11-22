use std::time::Duration;
use axum::{routing::post, Router, Extension, response::Redirect, Form, http::HeaderMap, middleware};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use sqlx::{types::time::OffsetDateTime, Pool, MySql, Error};
use crate::{app_state::AppState, auth::{middleware::logged_in, structs::UserJWT}, component::{structs::Referer, middleware::get_referer}};

use super::structs::{Post, PostBody};

pub async fn create(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Form(body): Form<PostBody>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);

    let query_result = sqlx::query("INSERT INTO posts (titulo, body, usuario_id, comunidade_id, tag_id) 
                                        VALUES (?, ?, ?, ?, CASE WHEN ? = 'NULL' THEN NULL ELSE ? END)")
        .bind(body.titulo)
        .bind(body.body)
        .bind(user.id)
        .bind(body.community_id)
        .bind(&body.tag_id)
        .bind(&body.tag_id)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(_) => {
            let mut cookie_ob = Cookie::new("success_msg", "Postagem criada com sucesso.");
            cookie_ob.set_path("/");
            cookie_ob.set_expires(now);
            let jar = jar.add(cookie_ob);
            Ok((jar, Redirect::to(referer.url.as_str())))
        },
        Err(err) => {
            println!("{err}");
            let mut cookie_ob = Cookie::new("error_msg", "Erro ao criar postagem.");
            cookie_ob.set_path("/");
            cookie_ob.set_expires(now);
            let jar = jar.add(cookie_ob);
            Err(
                (jar,
                Redirect::to(referer.url.as_str()))
            )
        },
    }   
}

pub async fn get_posts_data(db: &Pool<MySql>, community_id: Option<i64>) -> Vec<Post> {
    let query = "SELECT posts.id, posts.titulo, 
    CASE
        WHEN LENGTH(posts.body) <= 100 THEN posts.body
        ELSE CONCAT(SUBSTRING(posts.body, 1, 100), '...')
    END as body, 
    usuarios.nome AS user_name, comunidades.nome as community_name, tags.nome as tag_name, posts.created_at FROM posts JOIN usuarios ON posts.usuario_id = usuarios.id JOIN comunidades ON posts.comunidade_id = comunidades.id LEFT JOIN tags ON tags.id = posts.tag_id";
    let result: Result<Vec<Post>, Error>;
    if let Some(community_id) = community_id {
        result = sqlx::query_as::<_, Post>(
        &(query.to_owned()+" WHERE posts.comunidade_id = ?"))
        .bind(community_id)
        .fetch_all(db)
        .await;
    } else {
        result = sqlx::query_as::<_, Post>(
        query)
        .bind(community_id)
        .fetch_all(db)
        .await;
    }

    match result {
        Ok(vec) => {
            vec
        },
        Err(err) => {
            println!("{:?}", err);
            [].to_vec()},
    }
}

pub fn create_post_router() -> Router {
    Router::new()
        .route("/", post(create))
        .route_layer(middleware::from_fn(
            |req, next| logged_in(req, next),
        ))
}
