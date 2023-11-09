use axum::{Extension, response::IntoResponse};
use axum_extra::extract::CookieJar;
use maud::{html, Markup};

use crate::{app_state::AppState, community::structs::TopCommunity, component::page::build_page, post::{structs::Post, api::get_posts_data, view::render_posts_preview}};

async fn render_posts(state: &AppState) -> Markup {
    let posts = get_posts_data(&state.db, None).await;
    render_posts_preview(posts)
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
                (render_posts(&state).await);
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