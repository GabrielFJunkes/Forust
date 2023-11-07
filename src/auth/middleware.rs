
// TODO is logged in

use std::time::Duration;

use axum::{response::{IntoResponse, Redirect, Response}, http::{HeaderMap, StatusCode, Request}, middleware::Next};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jsonwebtoken::{decode, EncodingKey, Validation, DecodingKey, Algorithm};
use sqlx::types::time::OffsetDateTime;

use super::structs_api::UserJWT;

pub async fn logged_in<B>(req: Request<B>, next: Next<B>) -> Result<Response, (CookieJar, Redirect)> {
    let header = req.headers();
    let jar = CookieJar::from_headers(header);
    
    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);
    
    let referer = match header.get("referer") {
        Some(url) => url.to_str().unwrap_or("/"),
        None => "/",
    };
    let session = jar.get("session_jwt");
    
    if let Some(token) = session {
        if token.value().is_empty() {
            let mut cookie_ob = Cookie::new("error_msg", "Você precisa estar logado.");
            cookie_ob.set_path("/");
            cookie_ob.set_expires(now);
            let jar = jar.add(cookie_ob);
            
            Err(
                (jar,
                Redirect::to(referer))
            )
        }else{
            let token = match token.value().split("=").next() {
                Some(str) => str,
                None => "",
            };

            match decode::<UserJWT>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256)) {
                Ok(data) => {
                    print!("{:?}", data);
                    Ok(next.run(req).await)
                }
                Err(err) => {
                    println!("{err}");
                    let mut cookie_ob = Cookie::new("error_msg", "Você precisa estar logado.");
                    cookie_ob.set_path("/");
                    cookie_ob.set_expires(now);
                    let jar = jar.add(cookie_ob);
                    
                    Err(
                        (jar,
                        Redirect::to(referer))
                    )
                },
            }
        }
        
    }else{
        Ok(next.run(req).await)
    }
}