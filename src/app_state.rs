use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use sqlx::{Pool, MySql};

#[derive(Clone)]
pub struct AppState {
    pub key: Key,
    pub db: Pool<MySql>
}

// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}