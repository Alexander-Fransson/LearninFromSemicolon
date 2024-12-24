use anyhow::{Ok, Result};
use rand::RngCore;

fn main() -> Result<()> {
    let mut key = [0u8; 64]; // 512 bits or 64 bytes
    rand::thread_rng().fill_bytes(&mut key);
    println!("Generated key for  Hmac: {:?}", key);
    
    let b64u = base64_url::encode(&key);
    println!("Generated key b64u encoded: {}", b64u);

    Ok(())
}