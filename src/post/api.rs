use std::{time::Duration, collections::HashMap};
use axum::{routing::post, Router, Extension, response::Redirect, Form, http::HeaderMap, middleware};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use sqlx::{types::time::OffsetDateTime, Pool, MySql, Error};
use crate::{app_state::AppState, auth::{middleware::logged_in, structs::UserJWT}, component::{structs::Referer, middleware::get_referer}, post::structs::Comment};

use super::structs::{PostPreview, PostBody, Post};

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
        Err(_err) => {
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

pub async fn get_posts_data(db: &Pool<MySql>, community_id: Option<i64>) -> Vec<PostPreview> {
    let query = "SELECT posts.id, posts.titulo, 
    CASE
        WHEN LENGTH(posts.body) <= 100 THEN posts.body
        ELSE CONCAT(SUBSTRING(posts.body, 1, 100), '...')
    END as body, 
    usuarios.nome AS user_name, comunidades.nome as community_name, tags.nome as tag_name, posts.created_at FROM posts JOIN usuarios ON posts.usuario_id = usuarios.id JOIN comunidades ON posts.comunidade_id = comunidades.id LEFT JOIN tags ON tags.id = posts.tag_id";
    let result: Result<Vec<PostPreview>, Error>;
    if let Some(community_id) = community_id {
        result = sqlx::query_as::<_, PostPreview>(
        &(query.to_owned()+" WHERE posts.comunidade_id = ?"))
        .bind(community_id)
        .fetch_all(db)
        .await;
    } else {
        result = sqlx::query_as::<_, PostPreview>(
        query)
        .bind(community_id)
        .fetch_all(db)
        .await;
    }

    match result {
        Ok(vec) => {
            vec
        },
        Err(_err) => {
            [].to_vec()},
    }
}

pub async fn get_post_data(db: &Pool<MySql>, post_id: String) -> Option<Post> {
    let query = "SELECT posts.id, posts.titulo, posts.body, usuarios.nome AS user_name, 
    comunidades.nome as community_name, tags.nome as tag_name, posts.created_at 
    FROM posts JOIN usuarios ON posts.usuario_id = usuarios.id 
    JOIN comunidades ON posts.comunidade_id = comunidades.id 
    LEFT JOIN tags ON tags.id = posts.tag_id WHERE posts.id = ?";
    let result = sqlx::query_as::<_, PostPreview>(
        query)
        .bind(&post_id)
        .fetch_one(db)
        .await;

    match result {
        Ok(post) => {
            let comments_query = "SELECT comentarios.id, comentarios.body, usuarios.nome as user_name, comentarios.created_at FROM comentarios JOIN usuarios ON usuarios.id = comentarios.usuario_id WHERE post_id = ? AND comentario_id IS NULL";
            let result = sqlx::query_as::<_, Comment>(comments_query)
            .bind(&post_id)
            .fetch_all(db)
            .await;
            let comments = match result {
                Ok(comments) => comments,
                Err(_) => [].to_vec(),
            };
            let answers_query = "SELECT comentarios.id, comentarios.body, usuarios.nome as user_name, comentarios.created_at FROM comentarios JOIN usuarios ON usuarios.id = comentarios.usuario_id WHERE post_id = ? AND comentario_id IS NOT NULL";
            let result = sqlx::query_as::<_, Comment>(answers_query)
            .bind(post_id)
            .fetch_all(db)
            .await;
            let answers = match result {
                Ok(comments) => {
                    let answers: HashMap<i64, Comment> = comments.iter().enumerate().map(|(i, x)| (x.id, x.clone())).collect();
                    answers
                }
                Err(_) => {
                    HashMap::new()
                }
            };

            Some(Post{
                id: post.id,
                titulo: post.titulo,
                body: post.body,
                user_name: post.user_name,
                community_name: post.community_name,
                tag_name: post.tag_name,
                created_at: post.created_at,
                comments,
                answers
            })
        },
        Err(_err) => {None},
    }
}


pub fn create_post_router() -> Router {
    Router::new()
        .route("/", post(create))
        .route_layer(middleware::from_fn(
            |req, next| logged_in(req, next),
        ))
}
