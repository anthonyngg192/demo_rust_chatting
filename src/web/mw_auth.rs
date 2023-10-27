use crate::ctx::Ctx;
use crate::{Error, Result};
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn mw_require_auth<B>(
    _ctx: Result<Ctx>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth: Option<&axum::http::HeaderValue> = req.headers().get("Authorization");
    match auth {
        Some(_auth) => Ok(next.run(req).await),
        None => {
            println!("error");
            Err(Error::Unauthorized)
        }
    }
}

pub async fn mv_ctx_resolve<B>(req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");
    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::Unauthorized)?
            .clone()
    }
}
