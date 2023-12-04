use axum::{Extension, response::IntoResponse};
use axum_extra::extract::CookieJar;
use maud::{Markup, html, PreEscaped};

use crate::{app_state::AppState, component::{form::{Form, Input, FormElem, create_form}, page::build_page}, auth::structs::UserJWT, community::api::get_user_followed_communities};

const VALIDASCRIPT: &'static str= "
function validaPerfilForm() {
    var nome = document.getElementById('nome').value;
    var senha = document.getElementById('senha').value;
    var confirmaSenha = document.getElementById('senhaConfirma').value;
    
    if (nome === '[Removido]') {
        alert('Nome inválido. O nome não pode ser \"[Removido]\".');
        return false;
    }
    if (nome.length <= 1) {
        alert('Nome inválido. O nome precisa ter pelo menos 2 caracteres.');
        return false;
    }

    if (senha !== confirmaSenha) {
        alert('As senhas não coincidem. Por favor, confirme a senha novamente.');
        return false;
    }
    return true;
}
";

async fn render_followed_communities(state: AppState,user_id: i64) -> Markup {
    let communities = get_user_followed_communities(&state.db, user_id).await;
    html!(
        ul class="list-inside list-disc" {
            @if communities.is_empty() {
                li {
                    span class="mx-1 inline-flex flex font-bold text-gray-700 hover:text-gray-500" {
                        "Nenhuma comunidade inscrita"
                    }
                }
            }@else{
                @for community in communities {
                    li {
                        a href=(format!("/f/{}", community.nome)) class="mx-1 inline-flex flex font-bold text-gray-700 hover:text-gray-500" {
                            span class="underline decoration-blue-500" { (format!("f/{}", community.nome)) }
                            @if community.admin {
                                span class="ml-2 text-xs px-3 my-1 rounded-lg flex items-center bg-blue-600 text-white" { "admin" }
                            }
                        }
                    }
                }
            }
        }
    )
}

async fn content(state: AppState, user: UserJWT) -> Markup {
    let community_form: Form = Form {
        inputs: vec![
            Input {
                name: "Nome".to_string(),
                id: "nome".to_string(),
                form_elem: FormElem::Input,
                input_type: "text".to_string(),
                placeholder: "NomeDaComunidade".to_string()
            },
            Input {
                name: "Descrição".to_string(),
                id: "desc".to_string(),
                form_elem: FormElem::TextArea,
                input_type: "text".to_string(),
                placeholder: "Uma pequena descrição sobre a comunidade".to_string()
            },
        ],
        button_title: "Criar".to_string(),
        button_type: "submit".to_string(),
        action: "/api/comunidade".to_string(),
        method: "POST".to_string(),
        onsubmit: "".to_string(),
        rest: None,
    };
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="w-4/5 lg:w-8/12" {
                div class="flex items-center justify-between grow mb-4" {
                    h1 class="text-xl font-bold text-gray-700 md:text-2xl " {"Perfil"}
                }
                div class="px-5 py-6 bg-white rounded-lg shadow-md container flex justify-between w-full" {
                    script {(PreEscaped(VALIDASCRIPT))}
                    form method="POST" onsubmit="return validaPerfilForm()" action="/api/perfil" class="w-full" {
                        div class="flex" {
                            div class="w-2/5" {
                                label for="nome" {
                                    "Nome"
                                }
                                input 
                                required
                                name="nome"
                                id="nome"
                                value=(user.nome)
                                type="text"
                                placeholder="Seu Nome"
                                class="shadow appearance-none border rounded w-full 
                                py-2 px-3 text-gray-700 leading-tight focus:outline-none 
                                focus:shadow-outline" {}
                            }
                            div class="w-3/5 ml-5" {
                                label for="email" {
                                    "Email"
                                }
                                input 
                                required
                                name="email"
                                id="email"
                                value=(user.email)
                                type="email"
                                class="shadow appearance-none border rounded w-full 
                                py-2 px-3 text-gray-700 leading-tight focus:outline-none 
                                focus:shadow-outline" {}
                            }
                        }
                        div class="flex mt-3" {
                            div class="w-1/2 mr-3" {
                                label for="senha" {
                                    "Senha"
                                }
                                input 
                                name="senha"
                                id="senha"
                                type="password"
                                placeholder="*********"
                                class="shadow appearance-none border rounded w-full 
                                py-2 px-3 text-gray-700 leading-tight focus:outline-none 
                                focus:shadow-outline" {}
                            }
                            div class="w-1/2 ml-3" {
                                label for="senhaConfirma" {
                                    "Confirmar senha"
                                }
                                input 
                                name="senhaConfirma"
                                id="senhaConfirma"
                                type="password"
                                placeholder="*********"
                                class="shadow appearance-none border rounded w-full 
                                py-2 px-3 text-gray-700 leading-tight focus:outline-none 
                                focus:shadow-outline" {}
                            }
                        }
                        div class="flex justify-between" {
                            button type="submit"
                            class="mt-3 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded 
                            focus:outline-none focus:shadow-outline" {"Editar"}

                            a 
                            href=(format!("/api/perfil/{}", user.id))
                            class="mt-3 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-6 rounded 
                            focus:outline-none focus:shadow-outline"
                            {"Excluir conta"}
                        }
                    }
                }
            }
            div class="w-4/12 lg:block" {
                h1 class="mb-4 text-xl font-bold text-gray-700" {"Comunidades inscritas"}
                div class="flex flex-col px-6 py-4 mx-auto bg-white rounded-lg shadow-md" {
                    (render_followed_communities(state, user.id).await)
                }
                h1 class="my-4 text-xl font-bold text-gray-700" {"Criar comunidade"}
                (create_form(community_form))
            }
        }
    )
}

pub async fn profile_page(Extension(state): Extension<AppState>, Extension(user): Extension<UserJWT>, jar: CookieJar) -> impl IntoResponse {
    let title = "Forust - Perfil";
    build_page(&title, content(state, user).await, jar).await
}