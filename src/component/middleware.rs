use axum::{http::Request, middleware::Next, response::Response, Error};

use super::structs::Referer;

pub async fn get_referer<B>(mut req: Request<B>, next: Next<B>) -> Response {
    let header = req.headers();
    let referer = match header.get("referer") {
        Some(url) => url.to_str().unwrap_or("/"),
        None => "/",
    };
    let referer = Referer{url: referer.to_string()};
    req.extensions_mut().insert(referer);
    next.run(req).await
}