use axum::{response::IntoResponse, Extension, extract::Path};
use axum_extra::extract::CookieJar;
use maud::{Markup, html};
use crate::{component::page::build_page, app_state::AppState};

use super::{structs::{PostPreview, Post, Comment}, api::get_post_data};

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
                        div {
                            svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-6 h-6" {
                                path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M4.5 15.75l7.5-7.5 7.5 7.5" {}
                            }
                        }
                        div class="flex justify-center my-3" { "28" }
                        div {
                            svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-6 h-6" {
                                path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M19.5 8.25l-7.5 7.5-7.5-7.5" {}
                            }
                        }
                    }
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

fn create_comment_form(id: i64, post_id: Option<i64>) -> Markup {
    let mut url = String::new();
    let class_str ;
    if let Some(post_id) = post_id {
        url=format!("/api/comentario/{}/responder/{}", post_id, id);
        let temp = format!("w-full my-3 hidden peer-checked/comentario{}:block", id);
        class_str = temp;
    }else{
        url=format!("/api/comentario/{}", id);
        class_str = String::from("w-full my-3");
    }
    html!(
        form 
            action=(url)
            method="POST"
            class=(class_str) {
                textarea 
                name="body"
                class="shadow w-full appearance-none border rounded px-3 py-1 
                text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                placeholder="" {}
                button 
                class="bg-blue-500 w-full hover:bg-blue-700 text-white font-bold rounded 
                focus:outline-none focus:shadow-outline text-sm px-2 py-1 mt-2"
                type="submit" {"Comentar"}
            }
    )
}

fn render_comment (comments: Vec<Comment>, post_id: i64) -> Markup {
    html!(
        @for comment in comments{
            div class="ml-2 mb-4 w-full" {
                input id=(format!("comentario{}", comment.id)) type="checkbox" 
                class=(format!("hidden peer/comentario{}", comment.id)) {}
                div class="flex" {
                    a 
                    href=(format!("/u/{}",comment.user_name)) 
                    class="font-light text-gray-950 text-bold hover:text-gray-600 hover:underline" { 
                        (format!("u/{}",comment.user_name)) 
                    }
                    span class="font-light ml-3 text-gray-600" { 
                        (comment.created_at.date()) 
                    }
                }
                div {
                    @for line in comment.body.lines() {
                        p class="mt-1 text-gray-600" { (line) }
                    }
                }
                div class="flex" {
                    svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="w-5 h-5" {
                        path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M4.5 15.75l7.5-7.5 7.5 7.5" {}
                    }
                    p class="text-sm mx-2" {"28"}
                    svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="w-5 h-5" {
                        path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M19.5 8.25l-7.5 7.5-7.5-7.5" {}
                    }
                    label for=(format!("comentario{}", comment.id)) 
                    class="text-gray-700 hover:text-gray-500 font-bold rounded 
                    focus:outline-none focus:shadow-outline hover:cursor-pointer ml-2"
                    { 
                        svg 
                        xmlns="http://www.w3.org/2000/svg" 
                        fill="none" 
                        viewBox="0 0 24 24" 
                        stroke-width="1.5" 
                        stroke="currentColor" 
                        class="w-5 h-5 mr-2" {
                            path 
                            stroke-linecap="round"
                            stroke-linejoin="round" 
                            d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 
                            2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 
                            21l2.755-4.133a1.14 1.14 0 01.865-.501 48.172 48.172 0 
                            003.423-.379c1.584-.233 2.707-1.626 
                            2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 
                            0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 
                            6.741v6.018z" {}
                        }
                    }
                }
                (create_comment_form(comment.id, Some(post_id)))
            }
        }
    )
}

pub fn content(post: Post) -> Markup {
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="w-4/5 lg:w-8/12" {
                div class="mt-6 px-5 py-6 bg-white rounded-lg shadow-md container flex justify-between" {
                    div class="w-full" {
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
                            span class="text-2xl font-bold text-gray-700 mb-1"{
                                (post.titulo)
                            }
                            @for line in post.body.lines() {
                                p class="mt-1 text-gray-600" { (line) }
                            }
                        }
                    }
                }
                div class="mt-6 px-5 py-6 bg-white rounded-lg shadow-md container flex justify-between" {
                    div class="w-full" {
                        span class="text-2xl font-bold text-gray-700 mb-1 w-full"{
                            "Comentários"
                        }
                        (create_comment_form(post.id, None))
                        (render_comment(post.comments, post.id))
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

pub async fn post_page(
    Extension(state): Extension<AppState>,
    jar: CookieJar,
    Path(id): Path<String>) -> impl IntoResponse {
    let post = get_post_data(&state.db, id).await;
    if let Some(post) = post {
        let title = format!("Forust - {}", post.titulo);
        build_page(&title, content(post), jar).await
    }else{
        let title = "Forust - Erro";
        build_page(&title, content_empty(), jar).await
    }
}