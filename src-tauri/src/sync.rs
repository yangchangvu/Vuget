use serde::{Deserialize, Serialize};
use serde_json::Value;

// URL Firebase được nhúng lúc build từ biến VUGET_FIREBASE_URL (xem build.rs + .env).
// Không hardcode trong source → không lộ trên git. Rỗng nếu build thiếu cấu hình.
const FIREBASE_BASE: &str = match option_env!("VUGET_FIREBASE_URL") {
    Some(u) => u,
    None => "",
};

fn ensure_configured() -> Result<(), String> {
    if FIREBASE_BASE.is_empty() {
        return Err("Chưa cấu hình máy chủ đồng bộ (thiếu VUGET_FIREBASE_URL khi build).".into());
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncPayload {
    #[serde(default)]
    pub updated_at: i64,
    #[serde(default)]
    pub items: Vec<Value>,
}

fn code_error(code: &str) -> Option<String> {
    if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
        return Some("Mã phải là 6 chữ số.".into());
    }
    None
}

fn notes_path(code: &str) -> String {
    format!("{}/notes/{}.json", FIREBASE_BASE, code)
}

fn meta_path(code: &str) -> String {
    format!("{}/meta/{}.json", FIREBASE_BASE, code)
}

fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(12))
        .build()
        .unwrap()
}

// Tải note từ mã — danh sách máy này bị ghi đè bằng bản cloud (nếu có).
pub async fn sync_pull(code: &str) -> Result<SyncPayload, String> {
    ensure_configured()?;
    if let Some(e) = code_error(code) { return Err(e); }
    let c = client();
    let res = c.get(notes_path(code)).send().await.map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("Máy chủ trả về {}.", res.status()));
    }
    let v: Option<SyncPayload> = res.json().await.map_err(|e| e.to_string())?;
    match v {
        Some(p) => Ok(p),
        None => Err("Mã chưa tồn tại.".into()),
    }
}

// Đẩy note lên /notes/{code}; ghi RIÊNG timestamp (số) vào /meta/{code} để GC quét nhẹ.
pub async fn sync_push(code: &str, items: Vec<Value>) -> Result<(), String> {
    ensure_configured()?;
    if let Some(e) = code_error(code) { return Err(e); }
    let c = client();
    let now = chrono::Utc::now().timestamp_millis();
    let payload = SyncPayload { updated_at: now, items };
    let notes_body = serde_json::to_string(&payload).unwrap();
    let r1 = c.put(notes_path(code)).header("Content-Type", "application/json").body(notes_body).send().await.map_err(|e| e.to_string())?;
    if !r1.status().is_success() {
        return Err(format!("Máy chủ trả về {}.", r1.status()));
    }
    // /meta chỉ chứa mốc thời gian (số) — nhẹ để đọc toàn bộ khi dọn rác.
    let r2 = c.put(meta_path(code)).header("Content-Type", "application/json").body(now.to_string()).send().await.map_err(|e| e.to_string())?;
    if !r2.status().is_success() {
        return Err(format!("Máy chủ trả về {}.", r2.status()));
    }
    Ok(())
}

// Tạo mã 6 số mới từ thời gian hệ thống (đủ ngẫu nhiên cho ít người dùng, không thêm crate).
pub async fn sync_new_code() -> Result<String, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let mix = (d.as_nanos() as u64) ^ d.as_secs().wrapping_mul(2_654_435_761);
    Ok(format!("{:06}", 100_000 + (mix % 900_000)))
}

// Xoá mọi mã >90 ngày (xóa được của người khác — đúng yêu cầu dọn gọn hệ thống).
// Đọc /meta.json (nhẹ) để lấy updated_at từng mã, xóa mã quá hạn. Không tải /notes/* cho mỗi mã.
pub async fn sync_prune_stale(days: u64) -> Result<usize, String> {
    ensure_configured()?;
    let c = client();
    let meta_root = format!("{}/meta.json", FIREBASE_BASE);
    let resp = c.get(&meta_root).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("Máy chủ trả về {}.", resp.status()));
    }
    let root: Option<Value> = resp.json().await.map_err(|e| e.to_string())?;
    let Some(Value::Object(map)) = root else { return Ok(0); };
    let cutoff = chrono::Utc::now().timestamp_millis() - (days as i64) * 24 * 60 * 60 * 1000;
    let mut stale: Vec<String> = Vec::new();
    for (code, v) in map {
        if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) { continue; }
        // /meta/{code} giờ là số trực tiếp, hoặc {updatedAt/updated_at} từ phiên cũ.
        let ts = v.as_i64().or_else(|| v.get("updated_at").and_then(|x| x.as_i64())).or_else(|| v.get("updatedAt").and_then(|x| x.as_i64())).unwrap_or(0);
        if ts != 0 && ts < cutoff { stale.push(code); }
    }
    let mut removed = 0usize;
    for code in stale {
        // Xóa cả notes + meta
        let ok1 = c.delete(notes_path(&code)).send().await.map(|r| r.status().is_success()).unwrap_or(false);
        let ok2 = c.delete(meta_path(&code)).send().await.map(|r| r.status().is_success()).unwrap_or(false);
        if ok1 || ok2 { removed += 1; }
    }
    Ok(removed)
}
