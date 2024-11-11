use axum::{http::HeaderMap, Json};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::model::{Claims, LoginInfo, LoginResponse};
use axum::http::StatusCode;

pub async fn login_handler(Json(login_info): Json<LoginInfo>)
-> Result<Json<LoginResponse>, StatusCode> {
    let username = &login_info.username;
    let password = &login_info.password;

    let is_valid = is_valid_user(username, password).await;

    if is_valid {
        let claims = Claims {
            sub: username.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize, // 1 hour
        };

        let token = match encode(
            &Header::default(), 
            &claims,
            &EncodingKey::from_secret("secret".as_ref())
        ) {
            Ok(token) => token,
            Err(err) => {
                println!("Error generating token: {}", err);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            },
        };

        Ok(Json(LoginResponse{token}))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn is_valid_user(username: &str, password: &str) -> bool {
    // get user from db MOCK RIGHT NOW
    username == "user" && password == "password"
}

pub async fn get_info_handler(header_map:HeaderMap) -> Result<Json<String>, StatusCode> {
    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                
                let token = auth_header_str.trim_start_matches("Bearer ").to_string();

                println!("Token: {}", token);

                match decode::<Claims>(
                    &token, 
                    &DecodingKey::from_secret("secret".as_ref()), 
                    &Validation::default()
                ) {
                    Ok(_) => {
                        let info = "You are valid user".to_string();
                        return Ok(Json(info));
                    },
                    Err(err) => {
                        println!("Error decoding token: {}", err);
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }
            }            
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}