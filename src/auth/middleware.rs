
// TODO is logged in

use std::time::Duration;

use axum::{response::{Redirect, Response, IntoResponse}, http::Request, middleware::Next};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use jsonwebtoken::{decode, Validation, DecodingKey, Algorithm};
use sqlx::types::time::OffsetDateTime;

use crate::component::structs::Referer;

use super::structs::UserJWT;

pub async fn logged_in<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, (CookieJar, Redirect)> {
    let header = req.headers();
    let jar = CookieJar::from_headers(header);
    
    let mut now = OffsetDateTime::now_utc();
    now += Duration::from_secs(1);
    let mut referer = "/";
    if let Some(referer_ob) = req.extensions().get::<Referer>() {
        referer = referer_ob.url.as_str();
    }
    let session = jar.get("session_jwt");
    
    if let Some(token) = session {
        if token.value().is_empty() {
            let mut cookie_ob = Cookie::new("error_msg", "Você precisa estar logado.");
            cookie_ob.set_path("/");
            cookie_ob.set_expires(now);
            let jar = jar.add(cookie_ob);
            
            Err(
                (jar,
                Redirect::to(&referer))
            )
        }else{
            let token = match token.value().split("=").next() {
                Some(str) => str,
                None => "",
            };
            match decode::<UserJWT>(token, &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS256)) {
                Ok(data) => {
                    req.extensions_mut().insert(data.claims);
                    Ok(next.run(req).await)
                }
                Err(_err) => {
                    let mut cookie_ob = Cookie::new("error_msg", "Você precisa estar logado.");
                    cookie_ob.set_path("/");
                    cookie_ob.set_expires(now);
                    let jar = jar.add(cookie_ob);
                    
                    Err(
                        (jar,
                        Redirect::to(&referer))
                    )
                },
            }
        }
        
    }else{
        Ok(next.run(req).await)
    }
}
