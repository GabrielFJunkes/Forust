use std::collections::HashMap;

use maud::{Markup, html};
use rand::Rng;

use crate::{post::structs::Comment, component::ranking::create_ranking};

const COLORS: &'static [&'static str] = &[
    "red", "amber", "orange", "yellow", "lime", "emerald", "teal", "cyan", "pink", "blue", "indigo",
    "violet", "purple", "rose"
];

pub fn create_comment_form(id: i64, post_id: Option<i64>) -> Markup {
    let mut url = String::new();
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


pub fn render_comments (comments: Vec<Comment>, post_id: i64, answers: &HashMap<i64, Comment>, logged_in: bool) -> Markup {
    html!(
        @for comment in comments{
            (render_comment(comment, post_id, answers, logged_in))
        }
    )
}

pub fn render_comment (comment: Comment, post_id: i64, answers: &HashMap<i64, Comment>, logged_in: bool) -> Markup {
    let random_index = rand::thread_rng().gen_range(0..15);
    let random_color = rand::thread_rng().gen_range(2..8);
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
            @if logged_in {
                (create_comment_form(comment.id, Some(post_id)))
            }
            div class=(format!("border-l-4 border-{}-{}00 pl-2", COLORS.get(random_index).unwrap_or(&"indigo"), random_color)){
                @for answer_id in comment.answers_id {
                    @if answers.contains_key(&answer_id) {
                        (render_comment(answers.get(&answer_id).unwrap().clone(), post_id, answers, logged_in))
                    }
                }
            }
        }
    )
}
