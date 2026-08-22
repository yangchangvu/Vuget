use std::fs;

fn main() {
    // Nhúng URL Firebase lúc build. Ưu tiên biến môi trường (GitHub Actions secret);
    // nếu không có thì đọc từ file .env cạnh crate (không commit). Rỗng nếu thiếu.
    let url = std::env::var("VUGET_FIREBASE_URL")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| read_dotenv("VUGET_FIREBASE_URL"))
        .unwrap_or_default();
    println!("cargo:rustc-env=VUGET_FIREBASE_URL={}", url);
    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rerun-if-env-changed=VUGET_FIREBASE_URL");

    tauri_build::build()
}

fn read_dotenv(key: &str) -> Option<String> {
    let content = fs::read_to_string(".env").ok()?;
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                return Some(v.trim().trim_matches('"').to_string());
            }
        }
    }
    None
}
