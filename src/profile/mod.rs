use axum::{Extension, response::IntoResponse};
use axum_extra::extract::CookieJar;
use maud::{Markup, html};

use crate::{app_state::AppState, component::{form::{Form, Input, FormElem, create_form}, page::build_page}};

async fn content(_state: AppState) -> Markup {
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
        rest: None,
    };
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="w-4/5 lg:w-8/12" {
                div class="flex items-center justify-between grow" {
                    h1 class="text-xl font-bold text-gray-700 md:text-2xl " {"Perfil"}
                }
            }
            div class="w-4/12 lg:block" {
                h1 class="mb-4 text-xl font-bold text-gray-700" {"Comunidades inscritas"}
                div class="flex flex-col px-6 py-4 mx-auto bg-white rounded-lg shadow-md" {
                    ul class="list-inside list-disc" {
                        li {
                            a href="#" class="mx-1 inline-flex flex font-bold text-gray-700 hover:text-gray-500" {
                                span class="underline decoration-blue-500" { "f/Comida" }
                                span class="ml-2 text-xs px-3 my-1 rounded-lg flex items-center bg-blue-600 text-white" { "admin" }
                            }
                        }
                    }
                }
                h1 class="my-4 text-xl font-bold text-gray-700" {"Criar comunidade"}
                (create_form(community_form))
            }
        }
    )
}

pub async fn profile_page(Extension(state): Extension<AppState>, jar: CookieJar) -> impl IntoResponse {
    let title = "Forust - Home";
    build_page(&title, content(state).await, jar).await
}