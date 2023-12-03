use std::collections::HashMap;
use rand::Rng;

use axum::{response::IntoResponse, Extension, extract::Path};
use axum_extra::extract::CookieJar;
use maud::{Markup, html};
use crate::{component::{page::{build_page, is_logged_in, is_logged_in_with_data}, ranking::create_ranking}, app_state::AppState, comment::view::{create_comment_form, render_comments}};

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

pub fn content(post: Post, logged_in: bool) -> Markup {
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
                        }
                    }
                }
                div class="mt-6 px-5 py-6 bg-white rounded-lg shadow-md container flex justify-between" {
                    div class="w-full" {
                        span class="text-2xl font-bold text-gray-700 mb-1 w-full"{
                            "Comentários"
                        }
                        @if logged_in {
                            (create_comment_form(post.id, None))
                        }
                        (render_comments(post.comments, post.id, &post.answers, logged_in))
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
    let post: Option<Post>;
    let logged_in = is_logged_in_with_data(jar.get("session_jwt"));
    if let Some(user) = &logged_in {
        post = get_post_data(&state.db, id, Some(user.id)).await;
    }else{
        post = get_post_data(&state.db, id, None).await;
    }
    
    if let Some(post) = post {
        let title = format!("Forust - {}", post.titulo);
        build_page(&title, content(post, logged_in.is_some()), jar).await
    }else{
        let title = "Forust - Erro";
        build_page(&title, content_empty(), jar).await
    }
}