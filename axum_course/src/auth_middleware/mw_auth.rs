use axum::extract::{FromRequestParts, Request};
use axum::http::request::Parts;
use axum::response::Response;
use axum::middleware::Next;
use axum::body::Body;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use crate::ctx::Ctx;
use async_trait::async_trait;

use crate::{Error, Result};
use crate::login_api::web::AUTH_TOKEN;

// custom middleware

pub async fn mw_require_auth(
    ctx:Result<Ctx>,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {

    ctx?;

    // todo, token validation

    Ok(next.run(req).await)
}


pub async fn mw_ctx_resolver(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {

    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
    .ok_or(Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token) {
        Ok((id, _exp, _sign)) => {
            Ok(Ctx::new(id))
        }
        Err(e) => Err(e)
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    // store ctx result in request extension
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

// region, ctx extractor

// extractor allows us to parse the request and we can use it in all the routes
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx { 
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
        .extensions
        .get::<Result<Ctx>>()
        .ok_or(Error::AuthFailCtxNotInRequestExtension)?
        .clone()
    }
}

// endregion ctx extractor

fn parse_token(token: String) -> Result<(i32, String, String)> {
    
    let (_whole, id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;

    let id: i32 = id.parse()
    .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((id, exp.to_string(), sign.to_string()))
}