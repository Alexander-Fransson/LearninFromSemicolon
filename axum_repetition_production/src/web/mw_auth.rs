use std::result;

use crate::ctx::extractor::Ctx;
use crate::model::ticket::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, Request, State};
use axum::body::Body;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::{Cookie, Cookies};
use lazy_regex::regex_captures;
use async_trait::async_trait;

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
    .ok_or(Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token) {
        Ok((user_id, _exp, _sign)) => {
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e)
    };

    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

pub async fn mw_require_auth_3(
    ctx: Result<Ctx>,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {

    ctx?;

    Ok(next.run(req).await)
}

// ctx extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    // for server 7, the one bellow works for the other servers
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        
        parts
        .extensions
        .get::<Result<Ctx>>()
        .ok_or(Error::AuthFailCtxNotInRequestExtension)?
        .clone()
    }

    // async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
    //     let cookies = parts.extract::<Cookies>().await.unwrap();

    //     let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    //     let (user_id, _exp, _sign) = auth_token
    //     .ok_or(Error::AuthFailNoAuthTokenCookie)
    //     .and_then(parse_token)?;
            
    //     Ok(Ctx::new(user_id))
    // }
}

pub async fn mw_require_auth_2(
    cookies: Cookies,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let (_user_id, _exp, _sign) = auth_token
    .ok_or(Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token)?;
        
    Ok(next.run(req).await)
}


pub async fn mw_require_auth(
    cookies: Cookies,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;
    
    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(i32, String, String)> {
    let (_whole, id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: i32 = id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}