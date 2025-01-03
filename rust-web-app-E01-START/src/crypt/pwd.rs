use super::{Error, Result};
use crate::config;
use crate::crypt::{encrypt_into_b64u, EncryptContent};

pub fn encrypt_pwd(enc_content: &EncryptContent) -> Result<String> {
    let key = &config().PWD_KEY;
    let enc = encrypt_into_b64u(key, enc_content)?;

    // 01 to know what schema is used
    Ok(format!("#01#{}", enc))
}

pub fn validate_password(enc_content: &EncryptContent, ref_pwd: &str) -> Result<()> {
    let key = &config().PWD_KEY;
    let pwd = encrypt_pwd(enc_content)?;

    if pwd == ref_pwd {Ok(())} 
    else {Err(Error::PwdNotMatching)}
}