mod error;
pub mod pwd;
pub mod token;

use std::result;

pub use self::error::{Error, Result};

use base64_url;
use hmac::{Mac, Hmac};
use sha2::Sha512;

pub struct EncryptContent {
    pub content: String,
    pub salt: String,
}

pub fn encrypt_into_b64u(
    key: &[u8],
    enc_content: &EncryptContent
) -> Result<String> {

    let EncryptContent { content, salt } = enc_content;
    let mut hamac_sha512 = Hmac::<Sha512>::new_from_slice(key)
    .map_err(|_| Error::KeyFailHmac)?;

    hamac_sha512.update(content.as_bytes());
    hamac_sha512.update(salt.as_bytes());

    let hmac_result = hamac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();

    let result = base64_url::encode(&result_bytes);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use rand::{Rng, RngCore};

    #[test]
    fn test_encrypt_into_b64u_ok() -> Result<()> {
        let mut fx_key = [0u8; 64]; // 512 bits or 64 bytes
        rand::thread_rng().fill_bytes(&mut fx_key);

        let fx_enc_content = EncryptContent {
            content: "hello world".to_string(),
            salt: "kopparsulfat".to_string()
        };

        // todo: need to precompute fx_res
        let fx_res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;
        let res = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

        Ok(())
    }
}