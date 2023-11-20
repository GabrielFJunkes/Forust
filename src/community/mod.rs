pub mod structs;
pub mod api;
pub mod view;

use axum::{Extension, response::IntoResponse, extract::Path};
use axum_extra::extract::CookieJar;

use crate::{app_state::AppState, component::page::{build_page, is_logged_in_with_data}};

use self::{api::get_community_data, view::{content, content_empty}};

pub async fn community_page(Extension(state): Extension<AppState>, jar: CookieJar, Path(name): Path<String>, ) -> impl IntoResponse {
    let title = "f/".to_owned()+&name;
    let logged_in = is_logged_in_with_data(jar.get("session_jwt"));
    if let Some(community) = get_community_data(&state.db, &name).await {
        build_page(&title, content(&state, community, logged_in).await, jar).await
    }else {
        build_page(&title, content_empty(), jar).await
    }
}