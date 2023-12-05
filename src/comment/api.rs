use std::time::Duration;

use axum::{Extension, Form, response::Redirect, extract::Path, Router, middleware, routing::{post, get}};
use axum_extra::extract::CookieJar;
use sqlx::{types::time::OffsetDateTime, Pool, MySql};

use crate::{app_state::AppState, auth::{structs::UserJWT, middleware::logged_in}, component::{structs::Referer, cookie::create_cookie}, comment::structs::CommentEdit};

use super::structs::{CommentForm, CommentRanking};

pub async fn get_comment_data(db: &Pool<MySql>, id: String) -> Option<CommentEdit> {
    let query_result = sqlx::query_as::<_, CommentEdit>("SELECT id, body, usuario_id, post_id FROM comentarios WHERE id=?")
    .bind(id)
    .fetch_one(db)
    .await;
    match query_result {
        Ok(result) => {
            Some(result)
        }
        Err(_) => {
            None
        }
    }
}

pub async fn edit_comment(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    Path((post_id, id)): Path<(String,String)>,
    jar: CookieJar,
    Form(body): Form<CommentForm>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;
    
    let query_result = sqlx::query("UPDATE comentarios SET body = ? WHERE usuario_id = ? AND id = ?")
        .bind(body.body)
        .bind(user.id)
        .bind(&id)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(_) => {
            let path = format!("/p/{post_id}");
            let jar = jar.add(create_cookie("success_msg", "Comentário editado com sucesso.", path.clone()));
            Ok((jar, Redirect::to(&path)))
        },
        Err(err) => {
            println!("{err}");
            println!("{referer}");
            let jar = jar.add(create_cookie("error_msg", "Erro ao editar comentário.", url));
            Err(
                (jar,
                Redirect::to(referer))
            )
        },
    }   
}

async fn create(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Path(id): Path<String>,
    Form(body): Form<CommentForm>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;
    
    if body.body=="[Removido]" {
        let jar = jar.add(create_cookie("error_msg", "Erro ao criar comentário. Comentário não pode ser \"[Removido]\".", url));
        return Err((jar,Redirect::to(referer)))
    }

    if body.body.len()<2 {
        let jar = jar.add(create_cookie("error_msg", "Erro ao criar comentário. Comentário deve conter pelo menos 3 caracteres.", url));
        return Err((jar,Redirect::to(referer)))
    }

    let query_result = sqlx::query(
        "INSERT INTO comentarios (post_id, usuario_id, body) VALUES (?, ?, ?)")
        .bind(id)
        .bind(user.id)
        .bind(body.body)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(_) => {
            let jar = jar.add(create_cookie("success_msg", "Comentário cadastrado com sucesso.", url));
            Ok((jar, Redirect::to(referer)))
        },
        Err(_err) => {
            let jar = jar.add(create_cookie("error_msg", "Erro ao cadastrar comentário.", url));
            Err(
                (jar,
                Redirect::to(referer))
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

async fn delete(
    Extension(state): Extension<AppState>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Path(id ): Path<String>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {
    
    let query_result = sqlx::query(
    "UPDATE comentarios SET body = '[Removido]' WHERE id = ?")
    .bind(id)
    .execute(&state.db)
    .await;

    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;
    
    match query_result {
        Ok(_) => {
            let jar = jar.add(create_cookie("success_msg", "Comentário removido com sucesso.", url));
            Ok((jar, Redirect::to(referer)))
        },
        Err(_) => {
            let jar = jar.add(create_cookie("error_msg", "Erro ao remover comentário.", url));
            Err((jar, Redirect::to(referer)))
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
        .route("/:id/excluir", get(delete))
        .route("/:id_post/responder/:id", post(create_answer))
        .route("/:id/avaliar/:ranking_type", get(avaliate))
        .route_layer(middleware::from_fn(
            |req, next| logged_in(req, next),
        ))
}