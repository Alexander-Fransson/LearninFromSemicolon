use std::fmt::Display;
use std::str::FromStr;

use base64_url::base64::write;
use tracing_subscriber::fmt::format;
use crate::utils::{decode_b64u, encode_b64u, now_utc, now_utc_plus_sec_str, parse_utc};

use crate::config;
use crate::crypt::{Error, Result};

use super::{encrypt_into_b64u, EncryptContent};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub ident: String,
    pub exp: String,
    pub sign_b64u: String,
}

// fix me: from str & display

impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> Result<Self> {
        let parts: Vec<&str> = token_str.split('.').collect();

        let (ident_b64u, exp, sign_b64u) = (
            parts[0],
            parts[1],
            parts[2],
        );

        Ok(
            Self {
                ident: decode_b64u(ident_b64u).map_err(|_| Error::TokenCannotDecodeIndent)?,
                exp: decode_b64u(exp).map_err(|_| Error::TokenCannotDecodeExp)?,
                sign_b64u: sign_b64u.to_string(),
            }
        )
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}.{}.{}",
            encode_b64u(&self.ident),
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
    ident: &str,
    duration_sec: f64,
    salt: &str,
    key: &[u8]
) -> Result<Token> {
    let ident = ident.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);
    let sign_b64u = _token_sign_into_b64u(&ident, &exp, salt, key)?;

    let token = Token {
        ident,
        exp,
        sign_b64u
    };

    Ok(token)
}

// create token signature from token parts and salt
fn _token_sign_into_b64u(   
    ident: &str,
    exp: &str,   
    salt: &str,
    key: &[u8]
) -> Result<String> {
    let content = format!("{}.{}", encode_b64u(ident), encode_b64u(exp));
    let sinature = encrypt_into_b64u(
        key,
        &EncryptContent {
            content,
            salt: salt.to_string(),
        }
    )?;

    Ok(sinature)   
}

fn _validate_token_sign_and_exp(
    origin_token: &Token,
    salt: &str,
    key: &[u8]
) -> Result<()> {
    let new_sign_b64u = _token_sign_into_b64u(&origin_token.ident, &origin_token.exp, salt, key)?;

    // validate signature
    if new_sign_b64u != origin_token.sign_b64u {
        return Err(Error::TokenSignatureNotMatching);
    }

    // validate exp
    let origin_exp = parse_utc(&origin_token.exp)
    .map_err(|_| Error::TokenExpNotIso)?;

    let now = now_utc();

    if origin_exp < now {
        return Err(Error::TokenExpired);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use anyhow::{Result};
    use super::*;

    #[ignore]
    #[test]
    fn test_token_display_ok() -> Result<()> {
        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2025-01-11T15:30:00Z".to_string(),
            sign_b64u: "some_b64u".to_string(),
        };

        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyNS0wMS0xMVQxNTozMDowMFo.some_b64u";

        assert_eq!(fx_token.to_string(), fx_token_str);

        Ok(())
    }

    #[ignore]
    #[test]
    fn test_token_from_str_ok() -> Result<()> {
        let fx_token_str = "ZngtaWRlbnQtMDE.MjAyNS0wMS0xMVQxNTozMDowMFo.some_b64u";

        let fx_token = Token {
            ident: "fx-ident-01".to_string(),
            exp: "2025-01-11T15:30:00Z".to_string(),
            sign_b64u: "some_b64u".to_string(),
        };

        let token: Token = fx_token_str.parse()?;

        assert_eq!(token, fx_token);

        Ok(())
    }

    #[test]
    fn test_validate_web_token_ok() -> Result<()> {
        let fx_user = "demo1";
        let fx_salt = "salt";
        let fx_durateion = 0.02;
        let token_key = &config().TOKEN_KEY;
        let fx_token = _generate_token(&fx_user, fx_durateion, fx_salt, token_key)?;

        thread::sleep(Duration::from_millis(10));
        let res = validate_web_token(&fx_token, &fx_salt);

        res?;

        Ok(())
    }

    #[test]
    fn test_validate_web_token_err_expired() -> Result<()> {
        let fx_user = "demo1";
        let fx_salt = "salt";
        let fx_durateion = 0.01;
        let token_key = &config().TOKEN_KEY;
        let fx_token = _generate_token(&fx_user, fx_durateion, fx_salt, token_key)?;

        thread::sleep(Duration::from_millis(20));
        let res = validate_web_token(&fx_token, &fx_salt);

        assert!(res.is_err());
        assert!(
            matches!(res, Err(Error::TokenExpired)),
            "Should have mathched Error::TokenExpired but was: {res:?}"
        );

        Ok(())
    }
}