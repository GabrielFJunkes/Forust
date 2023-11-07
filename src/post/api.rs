use std::collections::HashMap;

use axum::{routing::post, Router, Extension, response::Redirect, Form};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use sqlx::{types::time::OffsetDateTime, Pool, MySql};
use crate::{app_state::AppState, community};

use super::structs::Post;


pub async fn create(
    Extension(state): Extension<AppState>, 
    jar: CookieJar,
    Form(body): Form<HashMap<String, String>>) -> Result<(CookieJar, Redirect), (CookieJar, Redirect)> {

    println!("{:?}", body);
    Ok((jar, Redirect::to("/")))

}

pub async fn get_post_data(db: &Pool<MySql>, community_id: i64) -> Vec<Post> {
    let result = sqlx::query_as::<_, Post>("SELECT id, titulo, body FROM posts WHERE comunidade_id=?")
    .bind(community_id)
    .fetch_all(db)
    .await;

    match result {
        Ok(vec) => vec,
        Err(_) => [].to_vec(),
    }
}

pub fn create_post_router() -> Router {
    Router::new()
        .route("/post", post(create))
}
