use axum::extract::Request;
use axum::response::Response;
use axum::middleware::Next;
use axum::body::Body;
use tower_cookies::Cookies;


use crate::{Error, Result};
use crate::login_api::web::AUTH_TOKEN;

// custom middleware

pub async fn mw_require_auth(
    cookies:Cookies,
    req: Request<Body>, 
    next: Next
) -> Result<Response> {
    // print cookies argument
    println!("cookies: {:#?}", cookies);

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    
    println!("auth_token: {:#?}", auth_token);

    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    // Todo real auth token, parsing and validation

    Ok(next.run(req).await)
}