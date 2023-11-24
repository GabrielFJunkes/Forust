use std::time::Duration;

use axum::{Extension, Form, response::{Redirect, IntoResponse}, Router, middleware, routing::{post, get}, extract::{path, Path}};
use axum_extra::extract::{CookieJar, cookie::{Cookie, self}};
use sqlx::{Pool, MySql, types::time::OffsetDateTime};

use crate::{app_state::AppState, auth::{structs::UserJWT, middleware::logged_in}, component::{structs::Referer, cookie::create_cookie}};

use super::structs::{Community, CommunityData, Tag, CommunityBody, FollowedCommunityData, Follow, TagBody};


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
    now += Duration::from_secs(5);

    let query_result = sqlx::query("INSERT INTO comunidades (comunidades.nome, comunidades.desc) VALUES (?, ?)")
        .bind(body.nome)
        .bind(body.desc)
        .execute(&state.db)
        .await;

    match query_result {
        Ok(com) => {
            let _ = sqlx::query("INSERT INTO inscricoes (usuario_id, comunidade_id, admin) VALUES (?, ?, TRUE)")
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

pub async fn create_tag(
    Extension(state): Extension<AppState>,
    jar: CookieJar, 
    Extension(referer): Extension<Referer>,
    Path(id): Path<String>,
    Form(body): Form<TagBody>, ) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {
    let query_result = sqlx::query("INSERT INTO tags (nome, comunidade_id) VALUES (?, ?)")
        .bind(body.nome)
        .bind(id)
        .execute(&state.db)
        .await;
    match query_result {
        Ok(_) => {
            let url = referer.url;
            let cookie = jar.add(create_cookie("success_msg", "Categoria cadastrada com sucesso.", url.clone()));
            Ok((cookie, Redirect::to(&url)))
        },
        Err(_err) => {
            let url = referer.url;
            let cookie = jar.add(create_cookie("error_msg", "Falha ao cadastrar categoria.", url.clone()));
            Err((cookie, Redirect::to(&url)))},
    }
}

pub async fn get_user_followed_communities(db: &Pool<MySql>, user_id: i64) -> Vec<FollowedCommunityData> {
    let query_result = sqlx::query_as::<_, FollowedCommunityData>("SELECT comunidades.id, comunidades.nome, comunidades.desc, inscricoes.admin FROM inscricoes JOIN comunidades ON comunidades.id = inscricoes.comunidade_id WHERE usuario_id=?")
    .bind(user_id)
    .fetch_all(db)
    .await;
    match query_result {
        Ok(vec) => vec,
        Err(_) => [].to_vec(),
    }
} 

pub async fn get_if_follows(user_id: i64, community_id: &String, db: &Pool<MySql>) -> Option<Follow> {
    let query_result = sqlx::query_as::<_, Follow>("SELECT usuario_id, comunidade_id, admin FROM inscricoes WHERE usuario_id=? AND comunidade_id=?")
    .bind(user_id)
    .bind(community_id)
    .fetch_optional(db)
    .await;

    match query_result {
        Ok(follow) => follow,
        Err(_) => None
    }
}

pub async fn inscrever(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>,
    Extension(referer): Extension<Referer>,
    jar: CookieJar,
    Path(id): Path<String>
) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {
    let follow = get_if_follows(user.id, &id, &state.db).await;

    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(5);

    match follow {
        Some(result) => {
            if result.admin {
                todo!("Check if is only admin");
            }else{
                let delete_result = sqlx::query("DELETE FROM inscricoes WHERE usuario_id=? AND comunidade_id=?")
                .bind(user.id)
                .bind(id)
                .execute(&state.db)
                .await;   
                
                match delete_result {
                    Ok(_) => {
                        let mut cookie_ob = Cookie::new("success_msg", "Você deixou de seguir essa comunidade.");
                        cookie_ob.set_path("/");
                        cookie_ob.set_expires(now);

                        Ok((
                            jar.add(cookie_ob),
                            Redirect::to(&referer.url)
                        ))
                    },
                    Err(_err) => {
                        let mut cookie_ob = Cookie::new("error_msg", "Erro ao deixar de seguir comunidade.");
                        cookie_ob.set_path("/");
                        cookie_ob.set_expires(now);

                        Err((
                            jar.add(cookie_ob),
                            Redirect::to(&referer.url)
                        ))
                    },
                }
            }
        },
        None => {
            let _ = sqlx::query("INSERT INTO inscricoes (usuario_id, comunidade_id) VALUES (?, ?)")
            .bind(user.id)
            .bind(id)
            .execute(&state.db)
            .await;

            let mut cookie_ob = Cookie::new("success_msg", "Comunidade seguida com sucesso.");
            cookie_ob.set_path("/");
            cookie_ob.set_expires(now);

            Ok((
                jar.add(cookie_ob),
                Redirect::to(&referer.url)
            ))
        },
    }
}

pub fn create_community_router() -> Router {
    Router::new()
        .route("/", post(create))
        .route("/:id/tag", post(create_tag))
        .route("/:id/seguir", get(inscrever))
        .route_layer(middleware::from_fn(
            |req, next| logged_in(req, next),
        ))
}