use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

use axum::extract::Request;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;
use lazy_regex::regex_captures;

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