use std::time::Duration;

use axum::{Extension, Form, response::Redirect, Router, middleware, routing::post};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use sqlx::{Pool, MySql, types::time::OffsetDateTime};

use crate::{app_state::AppState, auth::{structs::UserJWT, middleware::logged_in}, component::structs::Referer};

use super::structs::{Community, CommunityData, Tag, CommunityBody};


pub async fn get_community_data(db: &Pool<MySql>, name: &String) -> Option<Community> {
    let query_result = sqlx::query_as::<_, CommunityData>("SELECT id, nome, `desc` FROM comunidades WHERE nome=?")
    .bind(name)
    .fetch_one(db)
    .await;
    match query_result {
        Ok(result) => {
            let tags_query = sqlx::query_as::<_, Tag>(
                r#"
                SELECT id, nome
                FROM tags
                WHERE comunidade_id = ? AND status = TRUE
                "#,
            )
            .bind(result.id)
            .fetch_all(db)
            .await;
            
            let tags: Vec<Tag> = match tags_query {
                Ok(vec) => vec,
                Err(_) => [].to_vec(),
            };

            Some(Community {
                id: result.id,
                nome: result.nome,
                desc: result.desc,
                tags
            })
        }
        Err(_) => {
            None
        },
    }
}

pub async fn create(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Form(body): Form<CommunityBody>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);

    let query_result = sqlx::query("INSERT INTO comunidades (nome, desc) VALUES (?, ?)")
        .bind(body.nome)
        .bind(body.desc)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(com) => {
            let _ = sqlx::query("INSERT INTO incricoes (usuario_id, comunidade_id, admin) VALUES (?, ?, TRUE)")
            .bind(user.id)
            .bind(com.last_insert_id())
            .execute(&state.db)
            .await;
            let mut cookie_ob = Cookie::new("success_msg", "Comunidade criada com sucesso.");
            cookie_ob.set_path("/");
            cookie_ob.set_expires(now);
            let jar = jar.add(cookie_ob);
            Ok((jar, Redirect::to(referer.url.as_str())))
        },
        Err(_err) => {
            let mut cookie_ob = Cookie::new("error_msg", "Erro ao criar comunidade.");
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

pub fn create_community_router() -> Router {
    Router::new()
        .route("/", post(create))
        .route_layer(middleware::from_fn(
            |req, next| logged_in(req, next),
        ))
}