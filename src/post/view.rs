use axum::{response::{IntoResponse, Redirect}, Extension, extract::Path};
use axum_extra::extract::CookieJar;
use maud::{Markup, html};
use crate::{component::{page::{build_page, is_logged_in_with_data}, ranking::create_ranking, cookie::create_cookie}, app_state::AppState, comment::view::{create_comment_form, render_comments}, auth::structs::UserJWT, community::{api::{get_community_data, get_if_follows}, structs::{Community, Tag}}};

use super::{structs::{PostPreview, Post}, api::get_post_data};


pub fn render_posts_preview(posts: Vec<PostPreview>) -> Markup {
    html!(
        @if posts.is_empty(){
            div class="mt-6 px-5 py-6 bg-white rounded-lg shadow-md container flex justify-between" {
                h1 class="text-lg font-bold text-gray-700 md:text-lx" { "Não temos postagens ainda :(" }
            }
        }else{
            @for post in posts {
                div class="mt-6 px-5 py-6 bg-white rounded-lg shadow-md container flex justify-between" {
                    div class="lg:w-fit flex-col flex content-around justify-center mr-3" {
                        (create_ranking(post.ranking, post.id, false, false, post.liked))}
                    div class="lg:w-full" {
                        div class="flex items-center justify-between" {
                            span class="font-light text-gray-600" { 
                                a 
                                class="mr-2"
                                href=(format!("/f/{}", post.community_name)) {
                                    (format!("f/{}", post.community_name))
                                }
                                (post.created_at.date()) 
                            }
                            @if let Some(tag_name) = post.tag_name {
                                a href=(format!("/f/{}?tag={}", post.community_name,tag_name))
                                class="px-2 py-1 font-bold text-gray-100 bg-gray-600 rounded hover:bg-gray-500" { (tag_name) }
                            }
                        }
                        div class="mt-2" {
                            a href=(format!("/p/{}", post.id)) class="text-2xl font-bold text-gray-700 hover:underline"{
                                (post.titulo)
                            }
                            @let mut line = post.body.lines();
                            @if let Some(first_line) = line.next() {
                                p class="mt-2 text-gray-600" { (first_line) }
                            }
                            @if let Some(second_line) = line.next() {
                                p class="mt-2 text-gray-600" { (second_line) }
                            }
                        }
                    }
                }
            }
        }
    )
}

pub fn content(post: Post, logged_in: Option<UserJWT>, admin: bool) -> Markup {
    let removed = post.titulo == "[Removido]";
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="w-4/5 lg:w-8/12" {
                div class="mt-6 px-5 pb-2 pt-5 bg-white rounded-lg shadow-md container flex justify-between" {
                    div class="w-full" {
                        div class="flex items-center justify-between" {
                            div class="font-light text-gray-600"{
                                a 
                                class="text-gray-950 text-lg text-bold hover:text-gray-600 hover:underline"
                                href=(format!("/f/{}", post.community_name)) {
                                    (format!("f/{}", post.community_name))
                                }
                                span class="mx-2 text-sm"{
                                    "- Postado por:"
                                }
                                a 
                                href=(format!("/u/{}",post.user_name)) 
                                class="font-light text-sm text-gray-950 text-bold hover:text-gray-600 hover:underline" { 
                                    (format!("u/{}",post.user_name)) 
                                }
                                span class="mx-2 text-sm"{
                                    "-"
                                }
                                span class="text-sm"{
                                    (post.created_at.date()) 
                                }
                            }
                            @if let Some(tag_name) = post.tag_name {
                                a href=(format!("/f/{}?tag={}", post.community_name,tag_name))
                                class="px-2 py-1 font-bold text-gray-100 bg-gray-600 rounded hover:bg-gray-500" { (tag_name) }
                            }
                        }
                        
                        
                        div class="my-2" {
                            span class="text-2xl font-bold text-gray-700 mb-1"{
                                (post.titulo)
                            }
                            @for line in post.body.lines() {
                                p class="mt-1 text-gray-600" { (line) }
                            }
                        }
                        div class="flex"{
                            (create_ranking(post.ranking, post.id, false, true, post.liked))
                            @if let Some(user) = &logged_in {
                                @if user.nome == post.user_name && !removed {
                                    a class="ml-2 text-gray-700 hover:text-blue-500 font-bold rounded 
                                    focus:outline-none focus:shadow-outline hover:cursor-pointer right-1"
                                    href=(format!("/p/{}/editar", post.id)) {
                                        svg 
                                        xmlns="http://www.w3.org/2000/svg" 
                                        fill="none" 
                                        viewBox="0 0 24 24" 
                                        stroke-width="1.5" 
                                        stroke="currentColor" 
                                        class="w-5 h-5"{
                                            path 
                                            stroke-linecap="round" 
                                            stroke-linejoin="round" 
                                            d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 
                                            1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 
                                            0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10" 
                                            {}
                                        }
                                    }
                                }
                                @if (user.nome == post.user_name || admin) && !removed {
                                    a class="ml-2 text-gray-700 hover:text-red-500 font-bold rounded 
                                    focus:outline-none focus:shadow-outline hover:cursor-pointer"
                                    href=(format!("/api/post/{}/excluir", post.id)) {
                                        svg 
                                        xmlns="http://www.w3.org/2000/svg" 
                                        fill="none" 
                                        viewBox="0 0 24 24" 
                                        stroke-width="1.5" 
                                        stroke="currentColor" 
                                        class="w-5 h-5"{
                                            path 
                                            stroke-linecap="round" 
                                            stroke-linejoin="round" 
                                            d="M20.25 7.5l-.625 10.632a2.25 2.25 0 01-2.247 
                                            2.118H6.622a2.25 2.25 0 01-2.247-2.118L3.75 
                                            7.5m6 4.125l2.25 2.25m0 0l2.25 2.25M12 
                                            13.875l2.25-2.25M12 13.875l-2.25 2.25M3.375 
                                            7.5h17.25c.621 0 1.125-.504
                                             1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125H3.375c-.621 
                                             0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 
                                             1.125z" {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div class="mt-6 px-5 py-6 bg-white rounded-lg shadow-md container flex justify-between" {
                    div class="w-full" {
                        span class="text-2xl font-bold text-gray-700 mb-1 w-full"{
                            "Comentários"
                        }
                        @if logged_in.is_some() {
                            (create_comment_form(post.id, None))
                        }
                        (render_comments(post.comments, post.id, &post.answers, logged_in, admin))
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
                    h1 class="text-xl font-bold text-gray-700 md:text-2xl " {"Essa postagem não existe :("}
                }
            }
        }
    )
}

fn render_edit_post(post: Post, tags: Vec<Tag>) -> Markup {
    html!(
        form 
        action=(format!("/api/post/{}", post.id))
        method="POST"
        class="bg-white flex flex-col shadow-md rounded px-8 pt-6 pb-8 mb-4" {
            div class="mb-6" {
                label class="block text-gray-700 text-sm font-bold mb-2" for="titulo" { "Título" }
                input 
                required
                id="titulo"
                value=(post.titulo)
                name="titulo"
                type="text"
                placeholder="Título da sua postagem"
                class="shadow appearance-none border rounded w-full py-2 px-3 
                text-gray-700 leading-tight focus:outline-none focus:shadow-outline" {}
            }
            div class="mb-6" {
                label class="block text-gray-700 text-sm font-bold mb-2" for="conteudo" { "Conteúdo" }
                textarea 
                required
                id="conteudo"
                name="body"
                placeholder="O conteúdo da sua postagem"
                class="shadow appearance-none border rounded w-full py-2 px-3 
                text-gray-700 leading-tight focus:outline-none focus:shadow-outline" {
                    (post.body)
                }
            }
            @if !tags.is_empty() {
                div class="mb-6 flex flex-nowrap" {
                    div class="flex mr-5" {
                    @if post.tag_name.is_none() {
                        input 
                        name="tag_id"
                        id="nenhum"
                        checked
                        value="NULL"
                        type="radio" {}
                        label class="block text-gray-700 text-sm font-bold ml-1" for="nenhum" { "Nenhum" }
                    }@else {
                        input 
                        name="tag_id"
                        id="nenhum"
                        value="NULL"
                        type="radio" {}
                        label class="block text-gray-700 text-sm font-bold ml-1" for="nenhum" { "Nenhum" }
                    }
                    }
                    @for tag in tags {
                        div class="flex mr-5" {
                            @match &post.tag_name {
                                Some(tag_name) => {
                                    @if tag_name==&tag.nome {
                                        input 
                                        name="tag_id"
                                        value=(tag.id)
                                        checked
                                        id=(tag.nome)
                                        type="radio" {}
                                        label class="block text-gray-700 text-sm font-bold ml-1" for=(tag.nome) { (tag.nome) }
                                    }@else{
                                        input 
                                        name="tag_id"
                                        value=(tag.id)
                                        id=(tag.nome)
                                        type="radio" {}
                                        label class="block text-gray-700 text-sm font-bold ml-1" for=(tag.nome) { (tag.nome) }
                                    }
                                },
                                None => {
                                    input 
                                    name="tag_id"
                                    value=(tag.id)
                                    id=(tag.nome)
                                    type="radio" {}
                                    label class="block text-gray-700 text-sm font-bold ml-1" for=(tag.nome) { (tag.nome) }
                                }
                            };
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
                type="submit" { "Editar" }
            }
        }
    )
}


pub fn content_edit(post: Post, community: Community) -> Markup {
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="mt-6 px-5 pb-2 pt-5 bg-white rounded-lg container flex justify-between" {
                div class="w-full" {
                    (render_edit_post(post, community.tags))
                }
            }
        }
    )
}

pub async fn post_page(
    Extension(state): Extension<AppState>,
    jar: CookieJar,
    Path(id): Path<String>) -> impl IntoResponse {
    let post: Option<Post>;
    let logged_in = is_logged_in_with_data(jar.get("session_jwt"));
    let mut admin= false;
    if let Some(user) = &logged_in {
        post = get_post_data(&state.db, id, Some(user.id)).await;
        if let Some(post) = &post {
            if let Some(follows) = get_if_follows(user.id, &(&post.community_id.to_string()), &state.db).await {
                admin = follows.admin
            }
        }
    }else{
        post = get_post_data(&state.db, id, None).await;
    }
    
    if let Some(post) = post {
        let title = format!("Forust - {}", post.titulo);
        build_page(&title, content(post, logged_in, admin), jar).await
    }else{
        let title = "Forust - Erro";
        build_page(&title, content_empty(), jar).await
    }
}

pub async fn edit_post_page(
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserJWT>,
    jar: CookieJar,
    Path(id): Path<String>) -> impl IntoResponse {
    let post = get_post_data(&state.db, id, Some(user.id)).await;
    
    if let Some(post) = post {
        if post.user_name == user.nome {
            let title = "Forust - Editar";
            let community = get_community_data(&state.db, &post.community_name).await;
            if let Some(community) = community {
                Ok(
                build_page(&title, content_edit(post, community), jar).await
                )
            }else{
                let url = format!("/p/{}", post.id);
                let jar = jar.add(create_cookie("error_msg", "Erro ao editar postagem.", url.clone()));
                Err(
                    (jar,
                    Redirect::to(&url))
                ) 
            }
        }else{
            let url = format!("/p/{}", post.id);
            let jar = jar.add(create_cookie("error_msg", "Você não tem permissão para editar essa postagem.", url.clone()));
            Err(
                (jar,
                Redirect::to(&url))
            )  
        }
    }else{
        let title = "Forust - Erro";
        Ok(build_page(&title, content_empty(), jar).await)
    }
}