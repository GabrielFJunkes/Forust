use sqlx::{Pool, MySql};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<MySql>
}