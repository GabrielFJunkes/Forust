use std::time::Duration;

use axum::{routing::post, Router, Extension, response::Redirect, Form, middleware};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jsonwebtoken::{encode, Header, EncodingKey};
use sqlx::{mysql::MySqlQueryResult, types::time::OffsetDateTime};
use crate::{app_state::AppState, auth::{middleware::logged_in, structs::{UserJWT, UserName}}, component::{structs::Referer, cookie::create_cookie}};

use super::structs::ProfileBody;



pub async fn create(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Form(body): Form<ProfileBody>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {
    
    let url = referer.url;
    let referer = url.clone();
    let referer = &referer;

    let query_username = sqlx::query_as::<_, UserName>("SELECT nome FROM usuarios WHERE nome = ? AND nome != '[Removido]' AND id!=?")
    .bind(&body.nome)
    .bind(&user.id)
    .fetch_optional(&state.db)
    .await;

    if let Ok(username) = query_username {
        if username.is_some() {
            let jar = jar.add(create_cookie("error_msg", "Nome de usuário já existe.", url));
            return Err((jar, Redirect::to(&referer)))
        }
    };

    let query_result: Result<MySqlQueryResult, sqlx::Error>;
    if body.senha.len()>0 {
        let password = bcrypt::hash(body.senha, 5).unwrap();
        query_result = sqlx::query(
            "UPDATE usuarios SET nome = ?, email = ?, senha = ? WHERE id = ?")
            .bind(&body.nome)
            .bind(&body.email)
            .bind(password)
            .bind(&user.id)
            .execute(&state.db)
            .await;
    }else{
        query_result = sqlx::query(
            "UPDATE usuarios SET nome = ?, email = ? WHERE id = ?")
            .bind(&body.nome)
            .bind(&body.email)
            .bind(&user.id)
            .execute(&state.db)
            .await;
    }

    match query_result {
        Ok(_) => {
            let expiration = OffsetDateTime::now_utc()+Duration::from_secs(60*60*24);
            let expiration = expiration.unix_timestamp();

            let token = encode(
                &Header::default(), 
                &UserJWT {
                    exp: expiration,
                    id: user.id,
                    nome: body.nome,
                    email: body.email
                }, 
                &EncodingKey::from_secret("secret".as_ref())
            ).unwrap();
            let mut cookie_ob = Cookie::new("session_jwt", token);
            cookie_ob.set_path("/");
            let jar = jar.add(cookie_ob);
            let cookie = jar.add(create_cookie("success_msg", "Perfil editado com sucesso.", url));
            Ok((cookie, Redirect::to(referer)))
        },
        Err(err) => {
            let cookie = match err {
                sqlx::Error::Database(error) => {
                    if error.is_unique_violation() {
                        jar.add(create_cookie("error_msg", "Email já está em uso.", url))
                    }else{
                        jar.add(create_cookie("error_msg", "Falha ao editar perfil.", url))
                    }
                },
                _ => {
                    jar.add(create_cookie("error_msg", "Falha ao editar perfil.", url))
                },
            };
            Err((cookie, Redirect::to(referer)))
        }
    }

}

pub fn create_profile_router() -> Router {
    Router::new()
        .route("/", post(create))
        .route_layer(middleware::from_fn(
            |req, next| logged_in(req, next),
        ))
}