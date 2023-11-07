use std::time::Duration;

use axum::{routing::post, Router, Extension, response::Redirect, Form};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jsonwebtoken::{encode, Header, EncodingKey};
use sqlx::types::time::OffsetDateTime;
use crate::app_state::AppState;

use super::structs_api::*;

pub async fn register(
    Extension(state): Extension<AppState>, 
    jar: CookieJar,
    Form(body): Form<RegisterBody>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let password = bcrypt::hash(body.senha, 5);

    if let Ok(password) = password {
        let query_result = sqlx::query("INSERT INTO usuarios (nome, email, senha) VALUES (?, ?, ?)")
            .bind(body.nome)
            .bind(body.email)
            .bind(password)
            .execute(&state.db)
            .await;

        let mut now = OffsetDateTime::now_utc();
        now += Duration::from_secs(1);

        match query_result {
            Ok(_) => {
                let mut cookie_ob = Cookie::new("success_msg", "Usuário cadastrado com sucesso.");
                cookie_ob.set_path("/");
                cookie_ob.set_expires(now);
                let cookie = jar.add(cookie_ob);
                Ok((cookie, Redirect::to("/login")))
            },
            Err(error) => {
                
                let cookie = match error {
                    sqlx::Error::Database(error) => {
                        if error.is_unique_violation() {
                            let mut cookie_ob = Cookie::new("error_msg", "Email já está em uso.");
                            cookie_ob.set_path("/");
                            cookie_ob.set_expires(now);

                            jar.add(cookie_ob)
                        }else{
                            let mut cookie_ob = Cookie::new("error_msg", "Erro ao criar usuário.");
                            cookie_ob.set_path("/");
                            cookie_ob.set_expires(now);
                            jar.add(cookie_ob)
                        }
                    },
                    _ => {
                        let mut cookie_ob = Cookie::new("error_msg", "Erro ao criar usuário.");
                        cookie_ob.set_path("/");
                        cookie_ob.set_expires(now);
                        jar.add(cookie_ob)
                    },
                };
                Err((cookie, Redirect::to("/register")))
            },
        }

    }else{
        let cookie = jar.add(Cookie::new("error_msg", "Erro ao criar hash."));
        Err((cookie, Redirect::to("/register")))
    }

}

pub async fn login(Extension(state): Extension<AppState>, jar: CookieJar, Form(body): Form<LoginBody>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);

    let query_result = sqlx::query_as::<_, UserLogin>("SELECT id, nome, email, senha FROM usuarios WHERE email = ?")
    .bind(body.email)
    .fetch_one(&state.db)
    .await;

    match query_result {
        Ok(result) => {
            match bcrypt::verify(body.senha, &result.senha) {
                Ok(equal) => {
                    if equal {
                        let expiration = now+Duration::from_secs(60*60*24);
                        let expiration = expiration.unix_timestamp();

                        let token = encode(
                            &Header::default(), 
                            &UserJWT {
                                exp: expiration,
                                id: result.id,
                                nome: result.nome,
                                email: result.email
                            }, 
                            &EncodingKey::from_secret("secret".as_ref())
                        );

                        match token {
                            Ok(token) => {
                                let mut cookie_ob = Cookie::new("success_msg", "Login realizado com sucesso.");
                                cookie_ob.set_path("/");
                                cookie_ob.set_expires(now);
                                let jar = jar.add(cookie_ob);
                                cookie_ob = Cookie::new("session_jwt", token);
                                cookie_ob.set_path("/");
                                let jar = jar.add(cookie_ob);
                                
                                Ok((jar, Redirect::to("/")))
                            },
                            Err(_) => {
                                let mut cookie_ob = Cookie::new("error_msg", "Erro interno no servidor.");
                                cookie_ob.set_path("/");
                                cookie_ob.set_expires(now);
                                Err((jar.add(cookie_ob), Redirect::to("/login")))
                            },
                        }

                    }else{
                        let mut cookie_ob = Cookie::new("error_msg", "Email ou senha incorreto.");
                        cookie_ob.set_path("/");
                        cookie_ob.set_expires(now);
                        Err((jar.add(cookie_ob), Redirect::to("/login")))
                    }
                },
                Err(err) => {
                    println!("1 {:?}", err);
                    let mut cookie_ob = Cookie::new("error_msg", "Erro interno no servidor.");
                    cookie_ob.set_path("/");
                    cookie_ob.set_expires(now);
                    Err((jar.add(cookie_ob), Redirect::to("/login")))
                },
            }
        }
        Err(err) => {
            match err {
                // Se não achou nenhum usuário não é um erro em si.
                sqlx::Error::RowNotFound => {
                    let mut cookie_ob = Cookie::new("error_msg", "Email ou senha incorreto.");
                    cookie_ob.set_path("/");
                    cookie_ob.set_expires(now);
                    Err((jar.add(cookie_ob), Redirect::to("/login")))
                }
                _ => {
                    let mut cookie_ob = Cookie::new("error_msg", "Erro interno no servidor.");
                    cookie_ob.set_path("/");
                    cookie_ob.set_expires(now);
                    Err((jar.add(cookie_ob), Redirect::to("/login")))
                }
            }
            
        }
    }
}


pub fn create_auth_router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
