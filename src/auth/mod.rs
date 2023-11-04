pub mod auth_api;

use axum::response::IntoResponse;
use axum_extra::extract::{CookieJar, cookie::Cookie};
use maud::{html, Markup, PreEscaped};
use crate::component::{build_page, form::{Form, Input, create_form}};

fn login() -> Markup {
    let form = Form {
        inputs: vec![
            Input {
                name: "Email".to_string(),
                id: "email".to_string(),
                input_type: "email".to_string(),
                placeholder: "usuario@email.com".to_string()
            },
            Input {
                name: "Senha".to_string(),
                id: "senha".to_string(),
                input_type: "password".to_string(),
                placeholder: "************".to_string()
            },
        ],
        button_title: "Logar".to_string(),
        button_type: "submit".to_string(),
        action: "/api/login".to_string(),
        method: "POST".to_string(),
        rest: Some(html!(
            a class="inline-block mx-5 align-baseline font-bold text-sm text-blue-500 hover:text-blue-800" href="register"
                {"Não tem cadastro?"}
        )),
    };
    html!(
        div class="flex-grow flex flex-col items-center justify-center" {
            div {
                (create_form(form));
                p class="text-center text-gray-500 text-xs max-w-md" {
                    {(PreEscaped("&copy;")) "2023 Forust. All rights reserved."}
                }
            }
        }
    )
}

fn register() -> Markup {
    let form = Form {
        inputs: vec![
            Input {
                name: "Nome".to_string(),
                id: "nome".to_string(),
                input_type: "text".to_string(),
                placeholder: "Nome".to_string()
            },
            Input {
                name: "Email".to_string(),
                id: "email".to_string(),
                input_type: "email".to_string(),
                placeholder: "usuario@email.com".to_string()
            },
            Input {
                name: "Senha".to_string(),
                id: "senha".to_string(),
                input_type: "password".to_string(),
                placeholder: "************".to_string()
            },
        ],
        button_title: "Registrar".to_string(),
        button_type: "submit".to_string(),
        action: "/api/register".to_string(),
        method: "POST".to_string(),
        rest: None,
    };
    html!(
        div class="flex-grow flex flex-col items-center justify-center" {
            div {
                (create_form(form));
                p class="text-center text-gray-500 text-xs max-w-md" {
                    {(PreEscaped("&copy;")) "2023 Forust. All rights reserved."}
                }
            }
        }
    )
}

pub async fn login_page(jar: CookieJar) -> impl IntoResponse {
    let title = "Forust - Login";
    build_page(&title, login(), jar).await
}

pub async fn regiter_page(jar: CookieJar) -> impl IntoResponse {
    let title = "Forust - Cadastro";
    build_page(&title, register(), jar).await
}