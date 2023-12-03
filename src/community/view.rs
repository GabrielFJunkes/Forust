use maud::{html, Markup};

use crate::{app_state::AppState, post::{api::get_posts_data, view::render_posts_preview}, community::api::get_if_follows, auth::structs::UserJWT};

use super::structs::{Tag, Community};

pub async fn render_posts(state: &AppState, id: i64, user_id: Option<i64>) -> Markup {
    let posts = get_posts_data(&state.db, Some(id), user_id).await;
    render_posts_preview(posts)
}

fn render_tags(tags: Vec<Tag>) -> Markup {
    html!(
        @for tag in tags {
            li {
                a href=(format!("?tag={}", tag.nome)) class="mx-1 inline-flex flex font-bold text-gray-700 hover:text-gray-600 hover:underline" {
                    "- "(tag.nome)
                }
            }
        }
    )
}

fn render_create_post(tags: &Vec<Tag>, community_id: i64) -> Markup {
    html!(
        form 
        action="/api/post"
        method="POST"
        class="bg-white flex flex-col shadow-md rounded px-8 pt-6 pb-8 mb-4" {
            input type="hidden" name="community_id" value=(community_id) {  }
            div class="mb-6" {
                label class="block text-gray-700 text-sm font-bold mb-2" for="titulo" { "Título" }
                input 
                id="titulo"
                name="titulo"
                type="text"
                placeholder="Título da sua postagem"
                class="shadow appearance-none border rounded w-full py-2 px-3 
                text-gray-700 leading-tight focus:outline-none focus:shadow-outline" {}
            }
            div class="mb-6" {
                label class="block text-gray-700 text-sm font-bold mb-2" for="conteudo" { "Conteúdo" }
                textarea 
                id="conteudo"
                name="body"
                placeholder="O conteúdo da sua postagem"
                class="shadow appearance-none border rounded w-full py-2 px-3 
                text-gray-700 leading-tight focus:outline-none focus:shadow-outline" {}
            }
            @if !tags.is_empty() {
                div class="mb-6 flex flex-nowrap" {
                    div class="flex mr-5" {
                        input 
                        name="tag_id"
                        id="nenhum"
                        checked
                        value="NULL"
                        type="radio" {}
                        label class="block text-gray-700 text-sm font-bold ml-1" for="nenhum" { "Nenhum" }
                    }
                    @for tag in tags {
                        div class="flex mr-5" {
                            input 
                            name="tag_id"
                            value=(tag.id)
                            id=(tag.nome)
                            type="radio" {}
                            label class="block text-gray-700 text-sm font-bold ml-1" for=(tag.nome) { (tag.nome) }
                        }
                    }
                }
            }@else{
                input 
                name="tag_id"
                id="nenhum"
                hidden
                checked
                value="NULL"
                type="radio" {}
            }
            div class="flex items-center justify-between" {
                button 
                class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded 
                focus:outline-none focus:shadow-outline mx-auto w-full" 
                type="submit" { "Postar" }
            }
        }
    )
}

pub async fn content(state: &AppState, community: Community, user: Option<UserJWT>) -> Markup {
    let follows = if let Some(user) = &user {
        get_if_follows(user.id, &(community.id.to_string()), &state.db).await
    }else {
        None
    };

        
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="w-4/5 lg:w-8/12" {
                div class="flex items-center justify-between grow" {
                    h1 class="mb-4 text-xl font-bold text-gray-700 md:text-2xl " {"Postagens"}
                    div class="mb-4" {
                        select
                        class="w-full border-gray-300 rounded-md shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                        {
                            option { "Mais recente" };
                            option { "Última semana" };
                        }
                    }
                }
                @if let Some(user) = &user {
                    (render_create_post(&community.tags, community.id))
                    (render_posts(state, community.id, Some(user.id)).await);
                }@else{
                    (render_posts(state, community.id, None).await);
                }
            }
            div class="w-4/12 lg:block" {
                div class="mb-4 inline-flex flex" {
                    h1 class="text-xl font-bold text-gray-700 md:text-2xl" {"f/" (community.nome)}
                    @if user.is_some() {
                        a href=(format!("/api/comunidade/{}/seguir", community.id)) 
                        class="ml-2 w-fit text-sm px-3 my-1 rounded-lg flex items-center bg-blue-600 text-white" { 
                            @if follows.is_some() {
                                "Deixar de seguir" 
                            }@else{
                                "Seguir"
                            }
                        }

                    }
                }
                div class="flex flex-col px-6 py-4 mx-auto bg-white rounded-lg shadow-md" {
                    (community.desc)
                }
                h1 class="mb-4 mt-10 text-xl font-bold text-gray-700" {"Categorias"}
                div class="flex flex-col px-6 py-4 mx-auto bg-white rounded-lg shadow-md" {
                    @if let Some(follows) = follows {
                        @if follows.admin {
                            form 
                            action=(format!("/api/comunidade/{}/tag", community.id))
                            method="POST"
                            class="flex w-full mb-3" {
                                input 
                                name="nome"
                                class="shadow w-3/4 appearance-none border rounded px-3 py-1 
                                text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                placeholder="Nome da Tag" {}
                                button 
                                class="bg-blue-500 w-1/4 hover:bg-blue-700 text-white font-bold rounded 
                                focus:outline-none focus:shadow-outline text-sm ml-2 px-2"
                                type="submit" {"Criar"}
                            }
                        }
                    }
                    ul {
                        (render_tags(community.tags))
                    }
                }
            }
        }
    )
}

pub fn content_empty() -> Markup {
    html!(
        div class="py-8 flex justify-center w-4/5 ml-10" {
            div class="w-full" {
                div class="flex items-center justify-between grow" {
                    h1 class="text-xl font-bold text-gray-700 md:text-2xl " {"Essa comunidade não existe :("}
                }
            }
        }
    )
}