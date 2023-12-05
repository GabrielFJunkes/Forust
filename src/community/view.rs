use maud::{html, Markup, PreEscaped};

use crate::{app_state::AppState, post::{api::get_posts_data, view::render_posts_preview, structs::PostPreview}, community::api::get_if_follows, auth::structs::UserJWT, component::cookie::create_cookie};

use super::{structs::{Tag, Community, User, CommunityParams}, api::{get_community_data, get_tag_data, get_community_users}};

use axum::{Extension, response::{IntoResponse, Redirect}, extract::{Path, Query}};
use axum_extra::extract::CookieJar;

use crate::component::page::{build_page, is_logged_in_with_data};

const VALIDAPOSTSCRIPT: &'static str= "
function validaPostForm() {
    var titulo = document.getElementById('titulo').value;
    var body = document.getElementById('conteudo').value;
    
    if (titulo === '[Removido]') {
        alert('Título inválido. O título não pode ser \"[Removido]\".');
        return false;
    }

    if (titulo.length <= 2) {
        alert('Título deve conter pelo menos 3 caracteres.');
        return false;
    }

    if (body.length <= 2) {
        alert('Corpo da postagem deve conter pelo menos 3 caracteres.');
        return false;
    }

    return true;
}
";

pub async fn render_posts(posts: Vec<PostPreview>) -> Markup {
    render_posts_preview(posts)
}

fn render_tags(tags: Vec<Tag>, is_admin: bool, name: &str) -> Markup {
    html!(
        @for tag in tags {
            li class="flex" {
                @if tag.status {
                    a 
                    href=(format!("?tag={}", tag.nome)) 
                    class="mx-1 inline-flex flex font-bold text-gray-700 hover:text-gray-600 hover:underline" {
                        "- "(tag.nome)
                    }
                }@else{
                    a 
                    href=(format!("?tag={}", tag.nome)) 
                    class="mx-1 line-through inline-flex flex font-bold text-gray-700 hover:text-gray-600 hover:underline" {
                        "- "(tag.nome)
                    }
                }
                
                @if is_admin {
                    @if tag.status {
                        a
                        title="Editar tag"  
                        class="ml-2 grid content-center text-gray-700 hover:text-blue-500 font-bold rounded 
                        focus:outline-none focus:shadow-outline hover:cursor-pointer"
                        href=(format!("/f/{name}/tag/{}/editar", tag.id)) {
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
                        a
                        title="Desativar tag" 
                        class="ml-2 grid content-center text-gray-700 hover:text-red-500 font-bold rounded 
                            focus:outline-none focus:shadow-outline hover:cursor-pointer"
                            href=(format!("/api/comunidade/{name}/tag/{}/excluir", tag.id)) {
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
                    }@else{
                        a 
                        title="Ativar tag"
                        class="ml-2 grid content-center text-gray-700 hover:text-blue-500 font-bold rounded 
                        focus:outline-none focus:shadow-outline hover:cursor-pointer"
                        href=(format!("/api/comunidade/{name}/tag/{}/ativar", tag.id)) {
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
                                d="M12 9v6m3-3H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" {}
                            }
                        }
                    }
                }
            }
        }
    )
}

fn render_create_post(tags: &Vec<Tag>, community_id: i64) -> Markup {
    let active_tags = tags.iter().filter(|&tag| tag.status).count();
    html!(
        script {(PreEscaped(VALIDAPOSTSCRIPT))}
        form 
        onsubmit="return validaPostForm()"
        action="/api/post"
        method="POST"
        class="mt-6 bg-white flex flex-col shadow-md rounded px-8 pt-6 pb-8 mb-4" {
            input type="hidden" name="community_id" value=(community_id) {  }
            div class="mb-6" {
                label class="block text-gray-700 text-sm font-bold mb-2" for="titulo" { "Título" }
                input 
                required
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
                required
                id="conteudo"
                name="body"
                placeholder="O conteúdo da sua postagem"
                class="shadow appearance-none border rounded w-full py-2 px-3 
                text-gray-700 leading-tight focus:outline-none focus:shadow-outline" {}
            }
            @if !tags.is_empty() && active_tags>0 {
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
                        @if tag.status {
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

pub async fn content(state: &AppState, community: Community, user: Option<UserJWT>, posts: Vec<PostPreview>) -> Markup {
    let follows = if let Some(user) = &user {
        get_if_follows(user.id, &(community.id.to_string()), &state.db).await
    }else {
        None
    };

        
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="w-4/5 lg:w-8/12" {
                div class="flex items-center justify-between grow" {
                    h1 class=" text-xl font-bold text-gray-700 md:text-2xl " {"Postagens"}
                    div class="" {
                        div class="relative w-fit" {
                            input id="dropdownCheckbox" type="checkbox" class="hidden peer" {}
                    
                            label 
                            for="dropdownCheckbox" 
                            class="w-full px-5 border-gray-300 rounded-md shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50" {
                                "Filtros"
                            }
                    
                            div class="hidden absolute z-10 w-full bg-white border border-gray-300 rounded-md mt-1 peer-checked:block" {
                                a href="?" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100" { "Mais votados" }
                                a href="?filter=recente" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100" { "Mais recente" }
                                a href="?filter=semana" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100" { "Última semana" }
                            }
                        }
                    }
                }
                @if user.is_some() {
                    (render_create_post(&community.tags, community.id))
                    (render_posts(posts).await);
                }@else{
                    (render_posts(posts).await);
                }
            }
            div class="w-4/12 lg:block" {
                div class="flex justify-between" {
                    div class="inline-flex flex" {
                        a 
                        href=(format!("/f/{}", community.nome))
                        class="text-xl font-bold text-gray-700 md:text-2xl" {"f/" (community.nome)}
                        @if user.is_some() {
                            a href=(format!("/api/comunidade/{}/seguir", community.id)) 
                            class="ml-2 w-fit text-sm px-2 rounded-lg flex items-center bg-blue-600 text-white" { 
                                @if follows.is_some() {
                                    "Deixar de seguir" 
                                }@else{
                                    "Seguir"
                                }
                            }
                        }
                    }
                    div class="flex"{
                        @if user.is_some() {
                            @if let Some(follow) = &follows {
                                @if follow.admin {
                                    a class="ml-2 grid content-center text-gray-700 hover:text-blue-500 font-bold rounded 
                                    focus:outline-none focus:shadow-outline hover:cursor-pointer right-1"
                                    href=(format!("/f/{}/editar", community.nome)) {
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
                            }
                        }
                    }
                }
                div class="mt-6 flex flex-col px-6 py-4 mx-auto bg-white rounded-lg shadow-md" {
                    (community.desc)
                }
                h1 class="mb-4 mt-10 text-xl font-bold text-gray-700" {"Categorias"}
                div class="flex flex-col px-6 py-4 mx-auto bg-white rounded-lg shadow-md" {
                    @if let Some(follows) = &follows {
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
                        @if let Some(follows) = &follows {
                            (render_tags(community.tags, follows.admin, &community.nome))
                        }@else{
                            (render_tags(community.tags, false, &community.nome))
                        }
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

pub async fn community_page(
    Extension(state): Extension<AppState>, 
    jar: CookieJar, 
    Path(name): Path<String>, Query(params): Query<CommunityParams>) -> impl IntoResponse {
    let title = "f/".to_owned()+&name;
    if let Some(community) = get_community_data(&state.db, &name).await {
        let logged_in = is_logged_in_with_data(jar.get("session_jwt"));
        let posts: Vec<PostPreview>;
        if let Some(user) = &logged_in {
            posts = get_posts_data(&state.db, Some(community.id), Some(user.id), params).await;
        }else{
            posts = get_posts_data(&state.db, Some(community.id), None, params).await;
        }
        build_page(&title, content(&state, community, logged_in, posts).await, jar).await
    }else {
        build_page(&title, content_empty(), jar).await
    }
}

fn render_edit_tag(community: Community, tag: Tag) -> Markup {
    html!(
        form 
        action=(format!("/api/comunidade/{}/tag/{}", community.id, tag.id))
        method="POST"
        class="bg-white flex flex-col shadow-md rounded px-8 pt-6 pb-8 mb-4" {
            input value=(community.nome) hidden name="nome_comunidade" {}
            div class="mb-6" {
                label class="block text-gray-700 text-sm font-bold mb-2" for="nome" { "Tag" }
                input 
                id="nome"
                name="nome"
                value=(tag.nome)
                placeholder="Nome da tag"
                class="shadow appearance-none border rounded w-full py-2 px-3 
                text-gray-700 leading-tight focus:outline-none focus:shadow-outline" {}
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

fn edit_tag_content(community: Community, tag: Tag) -> Markup {
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="mt-6 px-5 pb-2 pt-5 bg-white rounded-lg container flex justify-between" {
                div class="w-full" {
                    (render_edit_tag(community, tag))
                }
            }
        }
    )
}

pub async fn edit_tag_page(Extension(state): Extension<AppState>, 
                        Extension(user): Extension<UserJWT>, 
                        jar: CookieJar, 
                        Path((name, id)): Path<(String, String)>, ) -> impl IntoResponse {
    let title = "f/".to_owned()+&name;
    if let Some(community) = get_community_data(&state.db, &name).await {
        if let Some(follows) = get_if_follows(user.id, &(community.id.to_string()), &state.db).await {
            if follows.admin {
                if let Some(tag) = get_tag_data(&state.db, &id).await{
                    Ok(build_page(&title, edit_tag_content(community, tag), jar).await)
                }else{
                    let path = format!("c/{name}");
                    let jar = jar.add(create_cookie("error_msg", "Erro ao editar tag.", path.clone()));
                    Err(
                        (jar,
                        Redirect::to(&path))
                    ) 
                }
            }else{
                let path = format!("c/{name}");
                let jar = jar.add(create_cookie("error_msg", "Você não tem permissão para editar tags dessa comunidade.", path.clone()));
                Err(
                    (jar,
                    Redirect::to(&path))
                ) 
            }
        }else{
            let path = format!("c/{name}");
            let jar = jar.add(create_cookie("error_msg", "Erro ao editar tag.", path.clone()));
            Err(
                (jar,
                Redirect::to(&path))
            ) 
        }
    }else {
        let path = format!("c/{name}");
        let jar = jar.add(create_cookie("error_msg", "Erro ao editar tag.", path.clone()));
        Err(
            (jar,
            Redirect::to(&path))
        ) 
    }
}

fn render_edit_community(community: &Community) -> Markup {
    html!(
        form 
        action=(format!("/api/comunidade/{}", community.nome))
        method="POST"
        class="bg-white flex flex-col shadow-md rounded px-8 pt-6 pb-8 mb-4" {
            div class="mb-6" {
                label class="block text-gray-700 text-sm font-bold mb-2" for="desc" { "Descrição" }
                textarea 
                id="desc"
                name="desc"
                placeholder="Descrição da comunidade"
                class="shadow appearance-none border rounded w-full py-2 px-3 
                text-gray-700 leading-tight focus:outline-none focus:shadow-outline" {(community.desc)}
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

fn render_users(community: &Community, users: Vec<User>) -> Markup {
    html!(
        ul class="list-disc shadow-md rounded px-8 pt-6 pb-8 mb-4" {
            @for user in users {
                li class="flex mb-2"{
                    @if user.admin {
                        span class="underline decoration-sky-500/30" {
                            (user.nome) 
                        }
                        a 
                        href=(format!("/api/comunidade/{}/admin/{}/rem", community.id, user.id))
                        title="Tirar admin" 
                        class="ml-2 grid content-center text-gray-700 hover:text-red-500 font-bold rounded 
                        focus:outline-none focus:shadow-outline hover:cursor-pointer"{
                            svg 
                            xmlns="http://www.w3.org/2000/svg" 
                            fill="none" 
                            viewBox="0 0 24 24" 
                            stroke-width="1.5" 
                            stroke="currentColor" 
                            class="w-5 h-5" {
                                path stroke-linecap="round" stroke-linejoin="round" d="M22 10.5h-6m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.374 21c-2.331 0-4.512-.645-6.374-1.766z" {}
                            }
                        }
                    }@else{
                        span {
                            (user.nome) 
                        }
                        a
                        href=(format!("/api/comunidade/{}/admin/{}/add", community.id, user.id))
                        title="Adicionar admin"
                        class="ml-2 grid content-center text-gray-700 hover:text-green-500 font-bold rounded 
                        focus:outline-none focus:shadow-outline hover:cursor-pointer" {
                            svg 
                            xmlns="http://www.w3.org/2000/svg" 
                            fill="none" 
                            viewBox="0 0 24 24" 
                            stroke-width="1.5" 
                            stroke="currentColor" 
                            class="w-5 h-5" {
                                path stroke-linecap="round" stroke-linejoin="round" d="M19 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.374 21c-2.331 0-4.512-.645-6.374-1.766z" {}
                            }
                        }
                    }
                }
            }
        }
    )
}

fn edit_content(community: &Community, users: Vec<User>) -> Markup {
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="w-8/12" {
                div class="flex items-center justify-between grow" {
                    h1 class="mb-4 text-xl font-bold text-gray-700 md:text-2xl " {"Usuários"}
                }
                (render_users(community, users))
            }
            div class="w-4/12 lg:block" {
                div class="flex items-center justify-between grow" {
                    h1 class="mb-4 text-xl font-bold text-gray-700 md:text-2xl " {(community.nome)}
                }
                (render_edit_community(community))
            }
        }
    )
}

pub async fn edit_community_page(Extension(state): Extension<AppState>, 
                        Extension(user): Extension<UserJWT>, 
                        jar: CookieJar, 
                        Path(name): Path<String>, ) -> impl IntoResponse {
    let title = "f/".to_owned()+&name;
    if let Some(community) = get_community_data(&state.db, &name).await {
        if let Some(follows) = get_if_follows(user.id, &(&community.id.to_string()), &state.db).await {
            if follows.admin {
                Ok(build_page(&title, edit_content(&community, get_community_users(&state.db, community.id).await), jar).await)
            }else{
                let path = format!("c/{name}");
                let jar = jar.add(create_cookie("error_msg", "Você não tem permissão para editar essa comunidade.", path.clone()));
                Err(
                    (jar,
                    Redirect::to(&path))
                ) 
            }
        }else{
            let path = format!("c/{name}");
            let jar = jar.add(create_cookie("error_msg", "Erro ao editar comunidade.", path.clone()));
            Err(
                (jar,
                Redirect::to(&path))
            ) 
        }
    }else {
        let path = format!("c/{name}");
        let jar = jar.add(create_cookie("error_msg", "Erro ao editar comunidade.", path.clone()));
        Err(
            (jar,
            Redirect::to(&path))
        ) 
    }
}