use axum::{Extension, response::IntoResponse};
use axum_extra::extract::CookieJar;
use maud::{html, Markup};
use sqlx::types::time::OffsetDateTime;
use crate::{app_state::AppState, community::{structs::{Community, TopCommunity}, self}, component::page::build_page};

#[derive(sqlx::FromRow)]
struct Post { id: i64, titulo: String, body: String, user_id: i64, comunidade_id: i64, created_at: OffsetDateTime }

async fn render_post_preview(state: &AppState) -> Markup {
    let result = sqlx::query_as::<_, Post>("SELECT * FROM posts")
    .fetch_all(&state.db)
    .await;
    
    html!(
        @if let Ok(posts) = result{
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
            }
        } @else {
            h1 { "Ops! Aconteceu um erro no servidor." }
        }
    )
}

async fn get_top_communities(state: &AppState) -> Option<Vec<TopCommunity>> {
    let communities_query = sqlx::query_as::<_, TopCommunity>(
        r#"
        SELECT c.id, c.nome, COUNT(i.comunidade_id) AS count
        FROM comunidades AS c
        LEFT JOIN inscricoes AS i ON c.id = i.comunidade_id
        GROUP BY c.id, c.nome
        ORDER BY count DESC
        LIMIT 10
        "#,
    )
    .fetch_all(&state.db)
    .await;

    match communities_query {
        Ok(communities) => {
            if communities.is_empty(){
                None
            }else{
                Some(communities)
            }
        },
        Err(_) => {
            None
        },
    }
}

async fn render_top_communities(communities: Option<Vec<TopCommunity>>) -> Markup {
    html!(
        @if let Some(communities) = communities {
            @for community in communities {
                li {
                    a href={(format!("/f/{}", community.nome))} class="mx-1 inline-flex flex font-bold text-gray-700 hover:text-gray-500" {
                        span class="underline decoration-blue-500" { (format!("f/{}", community.nome)) }
                        span class="ml-2 text-xs px-3 my-1 rounded-lg flex items-center bg-blue-600 text-white" { (community.count) }
                    }
                }
            }
        } @else {
            li {
                span class="mx-1 inline-flex flex font-bold text-gray-700 underline decoration-blue-500" { "Não temos comunidades ainda :(" }
            }
        }
    )
}

async fn content(state: AppState) -> Markup {
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="w-4/5 lg:w-8/12" {
                div class="flex items-center justify-between grow" {
                    h1 class="text-xl font-bold text-gray-700 md:text-2xl " {"Postagens"}
                    div {
                        select
                        class="w-full border-gray-300 rounded-md shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
                        {
                            option { "Mais recente" };
                            option { "Última semana" };
                        }
                    }
                }
                (render_post_preview(&state).await);
            }
            div class="hidden w-4/12 lg:block" {
                h1 class="text-xl font-bold text-gray-700 md:text-2xl" {"Comunidades mais populares"}
                div class="flex flex-col px-6 py-4 mx-auto bg-white rounded-lg shadow-md" {
                    ul class="list-inside list-disc" {
                        (render_top_communities(get_top_communities(&state).await).await)
                    }
                }
            }
        }
    )
}


pub async fn home_page(Extension(state): Extension<AppState>, jar: CookieJar) -> impl IntoResponse {
    let title = "Forust - Home";
    build_page(&title, content(state).await, jar).await
}