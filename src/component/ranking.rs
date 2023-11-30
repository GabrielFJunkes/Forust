use maud::{Markup, html};



pub fn create_ranking(count: i64, id: i64, is_comment: bool, horizontal: bool) -> Markup {
    let mut pclass = "flex justify-center my-3";
    let mut route = "post";
    if is_comment {
        route = "comentario"
    }
    if horizontal {
        pclass = "text-sm mx-2";
    }
    html!(
        a href=(format!("/api/{}/{}/avaliar/like", route, id)){
            svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-5 h-5 hover:text-sky-400" {
                path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M4.5 15.75l7.5-7.5 7.5 7.5" {}
            }
        }
        p class=(pclass) {(count)}
        a href=(format!("/api/{}/{}/avaliar/dislike", route, id)){
            svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-5 h-5 hover:text-red-600" {
                path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M19.5 8.25l-7.5 7.5-7.5-7.5" {}
            }
        }
    )
}