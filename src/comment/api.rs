use std::time::Duration;

use axum::{Extension, Form, response::Redirect, extract::Path, Router, middleware, routing::{post, get}};
use axum_extra::extract::CookieJar;
use sqlx::types::time::OffsetDateTime;

use crate::{app_state::AppState, auth::{structs::UserJWT, middleware::logged_in}, component::{structs::Referer, cookie::create_cookie}};

use super::structs::{CommentForm, CommentRanking};



async fn create(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Path(id): Path<String>,
    Form(body): Form<CommentForm>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);

    let query_result = sqlx::query(
        "INSERT INTO comentarios (post_id, usuario_id, body) VALUES (?, ?, ?)")
        .bind(id)
        .bind(user.id)
        .bind(body.body)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(_) => {
            let jar = jar.add(create_cookie("success_msg", "Comentário cadastrado com sucesso.", referer.url.clone()));
            Ok((jar, Redirect::to(referer.url.as_str())))
        },
        Err(_err) => {
            let jar = jar.add(create_cookie("error_msg", "Erro ao cadastrar comentário.", referer.url.clone()));
            Err(
                (jar,
                Redirect::to(referer.url.as_str()))
            )
        },
    }   
}

async fn create_answer(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Path((id_post, id)): Path<(String, String)>,
    Form(body): Form<CommentForm>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);

    let query_result = sqlx::query(
        "INSERT INTO comentarios (post_id, usuario_id, comentario_id, body) VALUES (?, ?, ?, ?)")
        .bind(id_post)
        .bind(user.id)
        .bind(id)
        .bind(body.body)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(_) => {
            let jar = jar.add(create_cookie("success_msg", "Comentário respondido com sucesso.", referer.url.clone()));
            Ok((jar, Redirect::to(referer.url.as_str())))
        },
        Err(_err) => {
            let jar = jar.add(create_cookie("error_msg", "Erro ao responder comentário.", referer.url.clone()));
            Err(
                (jar,
                Redirect::to(referer.url.as_str()))
            )
        },
    }   
}

async fn avaliate(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Path((id, ranking_type)): Path<(String, String)>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);

    let ranking_type = if ranking_type=="like"{
        true
    }else{
        false
    };

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;

    let query_result = sqlx::query_as::<_, CommentRanking>
        ("SELECT * FROM usuarios_avaliam_comentarios WHERE comentario_id = ? AND usuario_id = ?")
        .bind(&id)
        .bind(user.id)
        .fetch_one(&state.db)
        .await;

    match query_result {
        Ok(comment) => {
            if comment.gostou==ranking_type {
                let _query_result = sqlx::query(
                    "DELETE FROM usuarios_avaliam_comentarios WHERE comentario_id = ? AND usuario_id = ?")
                    .bind(id)
                    .bind(user.id)
                    .execute(&state.db)
                    .await;
                let jar = jar.add(create_cookie("success_msg", "Avaliação apagada com sucesso.", url));
                Ok((jar, Redirect::to(referer)))
            }else{
                let _query_result = sqlx::query(
                    "UPDATE usuarios_avaliam_comentarios SET gostou = ? WHERE comentario_id = ? AND usuario_id = ?")
                    .bind(ranking_type)
                    .bind(id)
                    .bind(user.id)
                    .execute(&state.db)
                    .await;
                let jar = jar.add(create_cookie("success_msg", "Comentário avaliado com sucesso.", url));
                Ok((jar, Redirect::to(referer)))
            }
        },
        Err(_) => {
            let query_result = sqlx::query(
                "INSERT INTO usuarios_avaliam_comentarios (comentario_id, usuario_id, gostou) VALUES (?, ?, ?)")
                .bind(&id)
                .bind(user.id)
                .bind(ranking_type)
                .execute(&state.db)
                .await;
        
        
            match query_result {
                Ok(_) => {
                    let jar = jar.add(create_cookie("success_msg", "Comentário avaliado com sucesso.", url));
                    Ok((jar, Redirect::to(referer)))
                },
                Err(_) => {
                    let jar = jar.add(create_cookie("error_msg", "Erro ao avaliar comentário.", url));
                    Err(
                        (jar,
                        Redirect::to(referer))
                    )       
                }
            }  
        },
    }

     
}

pub fn create_comment_router() -> Router {
    Router::new()
        .route("/:id", post(create))
        .route("/:id_post/responder/:id", post(create_answer))
        .route("/:id/avaliar/:ranking_type", get(avaliate))
        .route_layer(middleware::from_fn(
            |req, next| logged_in(req, next),
        ))
}