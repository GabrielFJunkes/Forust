use std::time::Duration;

use axum_extra::extract::cookie::Cookie;
use sqlx::types::time::OffsetDateTime;

pub fn create_cookie(name: &'static str, msg: &'static str, referer: String) -> Cookie<'static> {
    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);
    let mut cookie = Cookie::new(name, msg);
    cookie.set_path(referer);
    cookie.set_expires(now);
    cookie
}