use std::collections::HashMap;

use axum::{Extension, extract::Path, response::{IntoResponse, Redirect}};
use axum_extra::extract::CookieJar;
use maud::{Markup, html};
use rand::Rng;

use crate::{post::structs::Comment, component::{ranking::create_ranking, page::build_page, structs::Referer, cookie::create_cookie}, auth::structs::UserJWT, app_state::AppState};

use super::{api::get_comment_data, structs::CommentEdit};

const COLORS: &'static [&'static str] = &[
    "red", "amber", "orange", "yellow", "lime", "emerald", "teal", "cyan", "pink", "blue", "indigo",
    "violet", "purple", "rose"
];

pub const VALIDACOMMENTSCRIPT: &'static str= "
function validaCommentForm(id) {
    var form = document.forms['form-comment-'+id];

    // Retrieve the values from the specific form
    var body = form['body'].value;
    
    if (body === '[Removido]') {
        alert('Comentário inválido. O comentário não pode ser \"[Removido]\".');
        return false;
    }

    if (body.length <= 2) {
        alert('Comentário deve conter pelo menos 3 caracteres.');
        return false;
    }

    return true;
}
";

pub fn create_comment_form(id: i64, post_id: Option<i64>) -> Markup {
    let url: String;
    let class_str ;
    if let Some(post_id) = post_id {
        url=format!("/api/comentario/{}/responder/{}", post_id, id);
        let temp = format!("w-full my-3 peer-checked/comentario{}:hidden", id);
        class_str = temp;
    }else{
        url=format!("/api/comentario/{}", id);
        class_str = String::from("w-full my-3");
    }
    html!(
        form 
            onsubmit=(format!("return validaCommentForm({})", id))
            action=(url)
            name=(format!("form-comment-{}", id))
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


pub fn render_comments (comments: Vec<Comment>, post_id: i64, answers: &HashMap<i64, Comment>, logged_in: Option<UserJWT>, admin: bool) -> Markup {
    html!(
        @for comment in comments{
            (render_comment(comment, post_id, answers, &logged_in, admin))
        }
    )
}

pub fn render_comment (comment: Comment, post_id: i64, answers: &HashMap<i64, Comment>, logged_in: &Option<UserJWT>, admin: bool) -> Markup {
    let random_index = rand::thread_rng().gen_range(0..15);
    let random_color = rand::thread_rng().gen_range(2..8);
    let removed = comment.body == "[Removido]";
    html!(
        div class="ml-2 mb-4" {
            input id=(format!("comentario{}", comment.id)) type="checkbox" checked
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
                    p class="my-1 text-gray-600" { (line) }
                }
            }
            div class="flex" {
                (create_ranking(comment.ranking, comment.id, true, true, comment.liked))
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
                    class="w-5 h-5" {
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
                @if let Some(user) = logged_in {
                    @if user.nome == comment.user_name && !removed {
                        a class="ml-2 text-gray-700 hover:text-blue-500 font-bold rounded 
                        focus:outline-none focus:shadow-outline hover:cursor-pointer right-1"
                        href=(format!("/c/{}/editar", comment.id)) {
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
                    @if (user.nome == comment.user_name || admin) && !removed {
                        a class="ml-2 text-gray-700 hover:text-red-500 font-bold rounded 
                        focus:outline-none focus:shadow-outline hover:cursor-pointer"
                        href=(format!("/api/comentario/{}/excluir", comment.id)) {
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
            @if logged_in.is_some() {
                (create_comment_form(comment.id, Some(post_id)))
            }
            div class=(format!("border-l-4 border-{}-{}00 pl-2", COLORS.get(random_index).unwrap_or(&"indigo"), random_color)){
                @for answer_id in comment.answers_id {
                    @if answers.contains_key(&answer_id) {
                        (render_comment(answers.get(&answer_id).unwrap().clone(), post_id, answers, logged_in, admin))
                    }
                }
            }
        }
    )
}

fn render_edit_comment(comment: CommentEdit) -> Markup {
    html!(
        form 
        action=(format!("/api/post/{}/comentario/{}", comment.post_id, comment.id))
        method="POST"
        class="bg-white flex flex-col shadow-md rounded px-8 pt-6 pb-8 mb-4" {
            div class="mb-6" {
                label class="block text-gray-700 text-sm font-bold mb-2" for="body" { "Comentário" }
                textarea 
                id="body"
                name="body"
                placeholder="Comentário"
                class="shadow appearance-none border rounded w-full py-2 px-3 
                text-gray-700 leading-tight focus:outline-none focus:shadow-outline" {(comment.body)}
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

fn content_edit(comment: CommentEdit) -> Markup {
    html!(
        div class="py-8 flex justify-center w-4/5 mx-auto space-x-8" {
            div class="mt-6 px-5 pb-2 pt-5 bg-white rounded-lg container flex justify-between" {
                div class="w-full" {
                    (render_edit_comment(comment))
                }
            }
        }
    )
}

pub async fn edit_comment_page(
    Extension(state): Extension<AppState>, 
    Extension(user): Extension<UserJWT>, 
    Extension(referer): Extension<Referer>, 
    jar: CookieJar,
    Path(id): Path<String>, ) -> impl IntoResponse {
    let title = "Forust - Editar";
    if let Some(comment) = get_comment_data(&state.db, id).await {
        if user.id == comment.usuario_id {
            Ok(build_page(&title, content_edit(comment), jar).await)
        }else{
            let jar = jar.add(create_cookie("error_msg", "Você não tem permissão para editar esse comentário.", referer.url.clone()));
            Err(
                (jar,
                Redirect::to(&referer.url))
            ) 
        }
    }else {
        let jar = jar.add(create_cookie("error_msg", "Erro ao editar comentário.", referer.url.clone()));
        Err(
            (jar,
            Redirect::to(&referer.url))
        ) 
    }
}

