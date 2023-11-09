use maud::{Markup, html};
use super::structs::Post;

pub fn render_posts_preview(posts: Vec<Post>) -> Markup {
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
                            span class="font-light text-gray-600" { (format!("f/{} - {}", post.community_name, post.created_at)) }
                            @if let Some(tag_name) = post.tag_name {
                                a href=(format!("/f/{}?tag={}", post.community_name,tag_name))
                                class="px-2 py-1 font-bold text-gray-100 bg-gray-600 rounded hover:bg-gray-500" { (tag_name) }
                            }
                        }
                        div class="mt-2" {
                            a href="#" class="text-2xl font-bold text-gray-700 hover:underline"{
                                (post.titulo)
                            }
                            p class="mt-2 text-gray-600" {
                                (post.body)
                            }
                        }
                    }
                }
            }
        }
    )
}
