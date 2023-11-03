use axum::Extension;
use maud::{html, Markup};
use sqlx::types::time::OffsetDateTime;
use crate::{component::build_page, app_state::AppState};

#[derive(sqlx::FromRow)]
struct Post { id: i64, titulo: String, body: String, user_id: i64, comunidade_id: i64, created_at: OffsetDateTime }

async fn render_post_preview(state: AppState) -> Markup {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
    .fetch_all(&state.db)
    .await;
    let posts = match posts {
        Ok(posts) => posts,
        Err(err) => {panic!("{:?}", err)},
    };
    
    html!(
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
                        span class="font-light text-gray-600" { "f/Comida - " (post.created_at) }
                        a href="#"
                        class="px-2 py-1 font-bold text-gray-100 bg-gray-600 rounded hover:bg-gray-500" { "Tag" }
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
    )
}

async fn content(state: AppState) -> Markup {
    html!(
        div class="py-8 flex-grow flex justify-center w-4/5 mx-auto space-x-8" {
            div class="w-full lg:w-8/12" {
                div class="flex items-center justify-between grow" {
                    h1 class="text-xl font-bold text-gray-700 md:text-2xl " {"Post"}
                    div {
                        select
                        class="w-full border-gray-300 rounded-md shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                        {
                            option { "Mais recente" };
                            option { "Última semana" };
                        }
                    }
                }
                (render_post_preview(state).await);
            }
            div class="hidden w-4/12 lg:block" {
                h1 class="mb-4 text-xl font-bold text-gray-700" {"Comunidades mais populares"}
                div class="flex flex-col px-6 py-4 mx-auto bg-white rounded-lg shadow-md" {
                    ul class="list-inside list-disc" {
                        li {
                            a href="#" class="mx-1 inline-flex flex font-bold text-gray-700 hover:text-gray-500" {
                                span class="underline decoration-blue-500" { "f/Comida" }
                                span class="ml-2 text-xs px-3 my-1 rounded-lg flex items-center bg-blue-600 text-white" { "582" }
                            }
                        }
                    }
                }
            }
        }
    )
}


pub async fn home_page(Extension(state): Extension<AppState>) -> Markup {
    let title = "Forust - Home";
    build_page(&title, content(state).await).await
}