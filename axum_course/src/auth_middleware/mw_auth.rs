use axum::extract::Request;
use axum::response::Response;
use axum::middleware::Next;
use axum::body::Body;
use lazy_regex::regex_captures;
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

    //token parsing
    let (_id, _exp, _sign) = auth_token
    .ok_or(Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token)?;

    // todo, token validation

    Ok(next.run(req).await)
}

// parse token to return (id, expiration, signature)

fn parse_token(token: String) -> Result<(i32, String, String)> {
    
    let (_whole, id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#,
        &token
    ).ok_or(Error::AuthFailTokenWrongFormat)?;

    let id: i32 = id.parse()
    .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((id, exp.to_string(), sign.to_string()))
}