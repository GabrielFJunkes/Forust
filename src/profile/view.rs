use axum::{Extension, response::IntoResponse, extract::Path};
use axum_extra::extract::CookieJar;
use maud::{Markup, html, PreEscaped};

use crate::{app_state::AppState, component::{form::{Form, Input, FormElem, create_form}, page::{build_page, is_logged_in_with_data}}, auth::structs::UserJWT, community::{api::get_user_followed_communities, structs::FollowedCommunityData}, post::{structs::PostPreview, api::get_user_posts_data, view::render_posts_preview}};

use super::{api::get_user_by_name, structs::User};

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

const VALIDACOMUNIDADESCRIPT: &'static str= "
function validaComunidadeForm() {
    var nome = document.getElementById('nomeComunidade').value;
    var desc = document.getElementById('descComunidade').value;
    
    if (nome === '[Removido]') {
        alert('Nome inválido. O nome não pode ser \"[Removido]\".');
        return false;
    }
    if (nome.length <= 1) {
        alert('Nome inválido. O nome precisa ter pelo menos 2 caracteres.');
        return false;
    }

    if (desc === '[Removido]') {
        alert('Descrição inválida. A descrição não pode ser \"[Removido]\".');
        return false;
    }
    if (desc.length <= 1) {
        alert('Descrição inválida. A descrição precisa ter pelo menos 2 caracteres.');
        return false;
    }
    return true;
}
";

fn render_followed_communities(communities: Vec<FollowedCommunityData>
) -> Markup {
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

fn content_own(user: UserJWT, followed_communities: Vec<FollowedCommunityData>, posts: Vec<PostPreview>) -> Markup {
    let community_form: Form = Form {
        inputs: vec![
            Input {
                title: "Nome".to_string(),
                name: "nome".to_string(),
                id: "nomeComunidade".to_string(),
                form_elem: FormElem::Input,
                input_type: "text".to_string(),
                placeholder: "NomeDaComunidade".to_string()
            },
            Input {
                title: "Descrição".to_string(),
                name: "desc".to_string(),
                id: "descComunidade".to_string(),
                form_elem: FormElem::TextArea,
                input_type: "text".to_string(),
                placeholder: "Uma pequena descrição sobre a comunidade".to_string()
            },
        ],
        button_title: "Criar".to_string(),
        button_type: "submit".to_string(),
        action: "/api/comunidade".to_string(),
        method: "POST".to_string(),
        onsubmit: "return validaComunidadeForm()".to_string(),
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
                div class="flex items-center justify-between grow my-4" {
                    h1 class="text-xl font-bold text-gray-700 md:text-2xl " {"Postagens"}
                }
                (render_posts_preview(posts))
            }
            div class="w-4/12 lg:block" {
                h1 class="mb-4 text-xl font-bold text-gray-700" {"Comunidades inscritas"}
                div class="flex flex-col px-6 py-4 mx-auto bg-white rounded-lg shadow-md" {
                    (render_followed_communities(followed_communities))
                }
                h1 class="my-4 text-xl font-bold text-gray-700" {"Criar comunidade"}
                script {(PreEscaped(VALIDACOMUNIDADESCRIPT))}
                (create_form(community_form))
            }
        }
    )
}

fn content_user(user: User, followed_communities: Vec<FollowedCommunityData>, posts: Vec<PostPreview>) -> Markup {
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="w-4/5 lg:w-8/12" {
                div class="flex items-center justify-between grow mb-4" {
                    h1 class="text-xl font-bold text-gray-700 md:text-2xl " {(format!("Postagens de u/{}", user.nome))}
                }
                (render_posts_preview(posts))
            }
            div class="w-4/12 lg:block" {
                h1 class="mb-4 text-xl font-bold text-gray-700" {"Comunidades inscritas"}
                div class="flex flex-col px-6 py-4 mx-auto bg-white rounded-lg shadow-md" {
                    (render_followed_communities(followed_communities))
                }
            }
        }
    )
}

fn content_none() -> Markup {
    html!(
        div class="py-8 flex justify-center w-4/5 ml-10" {
            div class="w-full" {
                div class="flex items-center justify-between grow" {
                    h1 class="text-xl font-bold text-gray-700 md:text-2xl " {"Esse usuário não existe :("}
                }
            }
        }
    )
}

pub async fn profile_page(
    Extension(state): Extension<AppState>, 
    Path(username): Path<String>, 
    jar: CookieJar) -> impl IntoResponse {
    let title = format!("Forust - u/{username}");
    let user = get_user_by_name(&state.db, &username).await;
    if let Some(user) = user {
        let logged_in = is_logged_in_with_data(jar.get("session_jwt"));
        if let Some(logged_user) = logged_in {
            if logged_user.nome==username {
                let communities = get_user_followed_communities(&state.db, user.id).await;
                let posts = get_user_posts_data(&state.db, logged_user.id, Some(logged_user.id)).await;
                build_page(&title, content_own(logged_user, communities, posts), jar).await
            }else{
                let communities = get_user_followed_communities(&state.db, user.id).await;
                let posts = get_user_posts_data(&state.db, user.id, Some(logged_user.id)).await;
                build_page(&title, content_user(user, communities, posts), jar).await
            }
        }else{
            let communities = get_user_followed_communities(&state.db, user.id).await;
                let posts = get_user_posts_data(&state.db, user.id, None).await;
                build_page(&title, content_user(user, communities, posts), jar).await
        }
    }else{
        build_page(&title, content_none(), jar).await
    }
}