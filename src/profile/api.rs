use std::time::Duration;

use axum::{routing::{post, get}, Router, Extension, response::Redirect, Form, middleware, extract::Path};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jsonwebtoken::{encode, Header, EncodingKey};
use sqlx::{query::Query, MySql, mysql::{MySqlArguments, MySqlQueryResult}, types::time::OffsetDateTime};
use crate::{app_state::AppState, auth::{middleware::logged_in, structs::UserJWT}, component::{structs::Referer, middleware::get_referer, cookie::create_cookie}, post::structs::Comment};

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
        Err(_) => {
            let cookie = jar.add(create_cookie("error_msg", "Falha ao editar perfil.", url));
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