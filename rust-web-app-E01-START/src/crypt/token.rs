use std::fmt::Display;

use base64_url::base64::write;
use crate::utils::encode_b64u;

use crate::config;
use crate::crypt::{Error, Result};

pub struct Token {
    pub indent: String,
    pub exp: String,
    pub sign_b64u: String,
}

// fix me: from str & display

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}.{}.{}",
            encode_b64u(&self.indent),
            encode_b64u(&self.exp), 
            self.sign_b64u
        )
    }
}

////////////////////////////////////
/// web token gen and validation

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = config();
    _generate_token(user, config.TOKEN_DURATION, salt, &config.TOKEN_KEY)    
}

pub fn validate_web_token(oringinal_token: &Token, salt: &str) -> Result<()> {
    let config = config();
    _validate_token_sign_and_exp(oringinal_token, salt, &config.TOKEN_KEY)?;

    Ok(())
}

////////////////////////////////////
/// private token gen and validation

fn _generate_token(
    indent: &str,
    duration_sec: f64,
    salt: &str,
    key: &[u8]
) -> Result<Token> {
    todo!()
}

// create token signature from token parts and salt
fn _token_sign_into_b64u(   
    indent: &str,
    exp: f64,   
    salt: &str,
    key: &[u8]
) -> Result<String> {
    todo!()
}

fn _validate_token_sign_and_exp(
    origin_token: &Token,
    salt: &str,
    key: &[u8]
) -> Result<()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use super::*;

    #[test]
    fn test_token_display_ok() -> Result<()> {
        let fx_token = Token {
            indent: "fx-ident-01".to_string(),
            exp: "2025-01-11T15:30:00Z".to_string(),
            sign_b64u: "some_b64u".to_string(),
        };

        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyNS0wMS0xMVQxNTozMDowMFo.some_b64u";

        assert_eq!(fx_token.to_string(), fx_token_str);

        Ok(())
    }
}