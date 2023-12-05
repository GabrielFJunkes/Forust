use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use maud::{html, Markup, PreEscaped};
use crate::component::{form::{Form, Input, create_form, FormElem}, page::build_page};

fn login() -> Markup {
    let form = Form {
        inputs: vec![
            Input {
                title: "Email".to_string(),
                name: "email".to_string(),
                id: "email".to_string(),
                form_elem: FormElem::Input,
                input_type: "email".to_string(),
                placeholder: "usuario@email.com".to_string()
            },
            Input {
                title: "Senha".to_string(),
                name: "senha".to_string(),
                id: "senha".to_string(),
                form_elem: FormElem::Input,
                input_type: "password".to_string(),
                placeholder: "************".to_string()
            },
        ],
        button_title: "Logar".to_string(),
        button_type: "submit".to_string(),
        action: "/api/auth/login".to_string(),
        method: "POST".to_string(),
        onsubmit: "".to_string(),
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

const VALIDAREGISTERSCRIPT: &'static str= "
function validaRegisterForm() {
    var nome = document.getElementById('nome').value;
    var senha = document.getElementById('senha').value;
    
    if (nome === '[Removido]') {
        alert('Nome inválido. O nome não pode ser \"[Removido]\".');
        return false;
    }

    if (senha.length <= 1) {
        alert('Senha inválida. A senha precisa ter pelo menos 2 caracteres.');
        return false;
    }
    return true;
}
";

fn register() -> Markup {
    let form = Form {
        inputs: vec![
            Input {
                title: "Nome".to_string(),
                name: "nome".to_string(),
                id: "nome".to_string(),
                form_elem: FormElem::Input,
                input_type: "text".to_string(),
                placeholder: "Nome".to_string()
            },
            Input {
                title: "Email".to_string(),
                name: "email".to_string(),
                id: "email".to_string(),
                form_elem: FormElem::Input,
                input_type: "email".to_string(),
                placeholder: "usuario@email.com".to_string()
            },
            Input {
                title: "Senha".to_string(),
                name: "senha".to_string(),
                id: "senha".to_string(),
                form_elem: FormElem::Input,
                input_type: "password".to_string(),
                placeholder: "************".to_string()
            },
        ],
        button_title: "Registrar".to_string(),
        button_type: "submit".to_string(),
        action: "/api/auth/register".to_string(),
        method: "POST".to_string(),
        onsubmit: "return validaRegisterForm()".to_owned(),
        rest: None,
    };
    html!(
        div class="flex-grow flex flex-col items-center justify-center" {
            div {
                script {(PreEscaped(VALIDAREGISTERSCRIPT))}
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