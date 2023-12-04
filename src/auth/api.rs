use std::time::Duration;

use axum::{routing::{post, get}, Router, Extension, response::Redirect, Form};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jsonwebtoken::{encode, Header, EncodingKey};
use sqlx::types::time::OffsetDateTime;
use crate::{app_state::AppState, component::{structs::Referer, cookie::create_cookie}};

use super::structs::*;

pub async fn register<'a>(
    Extension(state): Extension<AppState>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Form(body): Form<RegisterBody>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {
    
    let url = referer.url;

    let password = bcrypt::hash(body.senha, 5);

    if let Ok(password) = password {

        let query_username = sqlx::query_as::<_, UserName>("SELECT nome FROM usuarios WHERE nome = ? AND nome != '[Removido]'")
        .bind(&body.nome)
        .fetch_optional(&state.db)
        .await;

        if let Ok(username) = query_username {
            if username.is_some() {
                let jar = jar.add(create_cookie("error_msg", "Nome de usuário já existe.", String::from("/register")));
                return Err((jar, Redirect::to("/register")))
            }
        };

        let query_result = sqlx::query("INSERT INTO usuarios (nome, email, senha) VALUES (?, ?, ?)")
            .bind(body.nome)
            .bind(body.email)
            .bind(password)
            .execute(&state.db)
            .await;

        match query_result {
            Ok(_) => {
                let cookie = jar.add(create_cookie("success_msg", "Usuário cadastrado com sucesso.", String::from("/login")));
                Ok((cookie, Redirect::to("/login")))
            },
            Err(error) => {
                
                let cookie = match error {
                    sqlx::Error::Database(error) => {
                        if error.is_unique_violation() {
                            jar.add(create_cookie("error_msg", "Email já está em uso.", url))
                        }else{
                            jar.add(create_cookie("error_msg", "Erro ao criar usuário.", url))
                        }
                    },
                    _ => {
                        jar.add(create_cookie("error_msg", "Erro ao criar usuário.", url))
                    },
                };
                Err((cookie, Redirect::to("/register")))
            },
        }

    }else{
        let cookie = jar.add(create_cookie("error_msg", "Erro ao criar Hash.", url));
        Err((cookie, Redirect::to("/register")))
    }

}

pub async fn login(Extension(state): Extension<AppState>, jar: CookieJar, Form(body): Form<LoginBody>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    let query_result = sqlx::query_as::<_, UserLogin>("SELECT id, nome, email, senha FROM usuarios WHERE email = ?")
    .bind(body.email)
    .fetch_one(&state.db)
    .await;

    match query_result {
        Ok(result) => {
            match bcrypt::verify(body.senha, &result.senha) {
                Ok(equal) => {
                    if equal {
                        let expiration = OffsetDateTime::now_utc()+Duration::from_secs(60*60*24);
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
                                let jar = jar.add(create_cookie("success_msg", "Login realizado com sucesso.", String::from("/")));
                                let mut cookie_ob = Cookie::new("session_jwt", token);
                                cookie_ob.set_path("/");
                                let jar = jar.add(cookie_ob);
                                
                                Ok((jar, Redirect::to("/")))
                            },
                            Err(_) => {
                                let cookie_ob = create_cookie("error_msg", "Erro interno no servidor.", String::from("/login"));
                                Err((jar.add(cookie_ob), Redirect::to("/login")))
                            },
                        }

                    }else{
                        let cookie_ob = create_cookie("error_msg", "Email ou senha incorreto.", String::from("/login"));
                        Err((jar.add(cookie_ob), Redirect::to("/login")))
                    }
                },
                Err(_) => {
                    let cookie_ob = create_cookie("error_msg", "Erro interno no servidor.", String::from("/login"));
                    Err((jar.add(cookie_ob), Redirect::to("/login")))
                },
            }
        }
        Err(err) => {
            match err {
                // Se não achou nenhum usuário não é um erro em si.
                sqlx::Error::RowNotFound => {
                    let cookie_ob = create_cookie("error_msg", "Email ou senha incorreto.", String::from("/login"));
                    Err((jar.add(cookie_ob), Redirect::to("/login")))
                }
                _ => {
                    let cookie_ob = create_cookie("error_msg", "Erro interno no servidor.", String::from("/login"));
                    Err((jar.add(cookie_ob), Redirect::to("/login")))
                }
            }
            
        }
    }
}

async fn logout(jar: CookieJar, Extension(referer): Extension<Referer>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {
    let mut cookie = Cookie::named("session_jwt");
    cookie.set_path("/");
    Ok((jar.remove(cookie),Redirect::to(&referer.url)))
}

pub fn create_auth_router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout))
}
