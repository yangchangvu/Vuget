#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod lunar;
mod notes;
mod sync;
mod sysinfo;
mod weather;

use chrono::Local;
use serde_json::{json, Value};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

const GITHUB_REPO: &str = "yangchangvu/Vuget";

struct AppState {
    config: Mutex<config::Config>,
    notes: Mutex<notes::Notes>,
}

#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<config::Config, String> {
    let cfg = state.config.lock().unwrap();
    Ok(cfg.clone())
}

#[tauri::command]
async fn save_config(state: State<'_, AppState>, new_config: config::Config) -> Result<(), String> {
    let mut cfg = state.config.lock().unwrap();
    *cfg = new_config;
    cfg.save();
    Ok(())
}

#[tauri::command]
async fn get_lunar_date() -> Result<Value, String> {
    let now = Local::now().date_naive();
    match lunar::solar_to_lunar(now) {
        Some(ld) => Ok(json!({
            "day": ld.day,
            "month": ld.month,
            "year": ld.year,
            "leap": ld.leap,
            "yearName": ld.year_name,
        })),
        None => Ok(json!(null)),
    }
}

#[tauri::command]
async fn get_weather(lat: f64, lon: f64) -> Result<Value, String> {
    match weather::fetch_weather(lat, lon).await {
        Ok(data) => Ok(json!({
            "temp": data.temp,
            "humidity": data.humidity,
            "description": data.description,
            "icon": data.icon,
            "hourly": data.hourly,
            "daily": data.daily,
        })),
        Err(e) => Err(e),
    }
}

#[tauri::command]
async fn get_lunar_month(year: i32, month: u32) -> Result<Value, String> {
    use chrono::{Datelike, NaiveDate};
    let (ny, nm) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    let days = NaiveDate::from_ymd_opt(ny, nm, 1)
        .and_then(|d| d.pred_opt())
        .ok_or("Ngày không hợp lệ")?
        .day();
    let arr: Vec<Value> = (1..=days)
        .map(|d| match lunar::solar_to_lunar(NaiveDate::from_ymd_opt(year, month, d).unwrap()) {
            Some(ld) => json!({ "day": ld.day, "month": ld.month, "leap": ld.leap }),
            None => Value::Null,
        })
        .collect();
    Ok(Value::Array(arr))
}

#[tauri::command]
async fn get_sys_info() -> Result<Value, String> {
    let info = sysinfo::get_sys_info();
    Ok(json!({
        "cpuName": info.cpu_name,
        "cpuUsage": info.cpu_usage,
        "ramTotal": info.ram_total,
        "ramUsed": info.ram_used,
        "ramPercent": info.ram_percent,
        "diskTotal": info.disk_total,
        "diskUsed": info.disk_used,
        "diskPercent": info.disk_percent,
        "netRx": info.net_rx,
        "netTx": info.net_tx,
    }))
}

#[tauri::command]
async fn get_notes(state: State<'_, AppState>) -> Result<Value, String> {
    let notes = state.notes.lock().unwrap();
    Ok(json!({
        "items": notes.items,
    }))
}

#[tauri::command]
async fn add_note(state: State<'_, AppState>, title: String, body: String) -> Result<u32, String> {
    let mut notes = state.notes.lock().unwrap();
    let id = notes.add(title, body);
    Ok(id)
}

#[tauri::command]
async fn update_note(state: State<'_, AppState>, id: u32, title: String, body: String) -> Result<(), String> {
    let mut notes = state.notes.lock().unwrap();
    notes.update(id, title, body);
    Ok(())
}

#[tauri::command]
async fn delete_note(state: State<'_, AppState>, id: u32) -> Result<(), String> {
    let mut notes = state.notes.lock().unwrap();
    notes.remove(id);
    Ok(())
}

#[tauri::command]
async fn toggle_note_pinned(state: State<'_, AppState>, id: u32) -> Result<(), String> {
    let mut notes = state.notes.lock().unwrap();
    notes.toggle_pinned(id);
    Ok(())
}

#[tauri::command]
async fn toggle_note_hidden(state: State<'_, AppState>, id: u32) -> Result<(), String> {
    let mut notes = state.notes.lock().unwrap();
    notes.toggle_hidden(id);
    Ok(())
}

#[tauri::command]
async fn reorder_notes(state: State<'_, AppState>, ids: Vec<u32>) -> Result<(), String> {
    let mut notes = state.notes.lock().unwrap();
    notes.reorder(ids);
    Ok(())
}

// ---------- Cloud sync (Firebase RTDB) ----------

fn notes_as_values(state: &State<'_, AppState>) -> Vec<Value> {
    let notes = state.notes.lock().unwrap();
    notes.items.iter().filter_map(|n| serde_json::to_value(n).ok()).collect()
}

#[tauri::command]
async fn sync_pull(state: State<'_, AppState>, code: String) -> Result<u32, String> {
    let code = code.trim().to_string();
    let payload = sync::sync_pull(&code).await?;
    let items: Vec<notes::Note> = serde_json::from_value(Value::Array(payload.items))
        .map_err(|e| format!("Dữ liệu cloud không hợp lệ: {}", e))?;
    let count = items.len() as u32;
    {
        let mut notes = state.notes.lock().unwrap();
        notes.set_items(items);
    }
    {
        let mut cfg = state.config.lock().unwrap();
        cfg.sync_code = code;
        cfg.save();
    }
    Ok(count)
}

#[tauri::command]
async fn sync_push(state: State<'_, AppState>) -> Result<String, String> {
    let code = { state.config.lock().unwrap().sync_code.clone() };
    if code.is_empty() {
        return Err("Chưa liên kết mã đồng bộ nào.".into());
    }
    let items = notes_as_values(&state);
    sync::sync_push(&code, items).await?;
    Ok(code)
}

#[tauri::command]
async fn sync_new_code(state: State<'_, AppState>) -> Result<String, String> {
    let code = sync::sync_new_code().await?;
    let items = notes_as_values(&state);
    sync::sync_push(&code, items).await?;
    {
        let mut cfg = state.config.lock().unwrap();
        cfg.sync_code = code.clone();
        cfg.save();
    }
    Ok(code)
}

#[tauri::command]
async fn sync_unlink(state: State<'_, AppState>) -> Result<(), String> {
    let mut cfg = state.config.lock().unwrap();
    cfg.sync_code = String::new();
    cfg.save();
    Ok(())
}

#[tauri::command]
async fn sync_prune() -> Result<usize, String> {
    sync::sync_prune_stale(90).await
}

#[tauri::command]
async fn check_update(current_version: String) -> Result<Value, String> {
    // Ponytail: dùng reqwest sẵn có + GitHub public API, không cần thêm crate.
    let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);
    let client = reqwest::Client::builder()
        .user_agent("Vuget-UpdateChecker")
        .build()
        .map_err(|e| e.to_string())?;
    let resp: Value = client.get(&url).send().await.map_err(|e| e.to_string())?.json().await.map_err(|e| e.to_string())?;
    let tag = resp["tag_name"].as_str().unwrap_or("").trim_start_matches('v').to_string();
    let has_update = if tag.is_empty() {
        false
    } else {
        match (semver::Version::parse(&tag), semver::Version::parse(&current_version)) {
            (Ok(remote), Ok(local)) => remote > local,
            _ => false,
        }
    };
    Ok(json!({
        "hasUpdate": has_update,
        "latestVersion": if tag.is_empty() { current_version.clone() } else { tag },
        "htmlUrl": resp["html_url"].as_str().unwrap_or(""),
    }))
}

#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn restart_app(app: AppHandle) {
    app.restart();
}

#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    // Webview Tauri không mở được link ngoài; dùng trình duyệt mặc định của Windows.
    if !url.starts_with("https://") {
        return Err("invalid url".into());
    }
    std::process::Command::new("cmd")
        .args(["/C", "start", "", &url])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

// Đồng bộ khóa registry Run (start with Windows) theo config
fn sync_autostart(enabled: bool) {
    use winreg::enums::*;
    use winreg::RegKey;
    let Ok(run) = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(
        r"Software\Microsoft\Windows\CurrentVersion\Run",
        KEY_SET_VALUE,
    ) else {
        return;
    };
    if enabled {
        if let Ok(exe) = std::env::current_exe() {
            let _ = run.set_value("Vuget", &exe.to_string_lossy().to_string());
        }
    } else {
        let _ = run.delete_value("Vuget");
    }
}

// Kill mọi tiến trình vuget.exe khác đang chạy (single instance)
fn kill_other_instances() {
    use std::process::{Command, id};
    let self_pid = id();
    if let Ok(out) = Command::new("tasklist").args(["/FI", "IMAGENAME eq vuget.exe", "/FO", "CSV", "/NH"]).output() {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 2 { continue; }
            let pid_str = parts[1].trim_matches('"');
            if let Ok(pid) = pid_str.parse::<u32>() {
                if pid != self_pid as u32 {
                    let _ = Command::new("taskkill").args(["/F", "/PID", &pid.to_string()]).output();
                }
            }
        }
    }
}

fn main() {
    kill_other_instances();
    let state = AppState {
        config: Mutex::new(config::Config::load()),
        notes: Mutex::new(notes::Notes::load()),
    };

    tauri::Builder::default()
        .manage(state)
        .setup(|app| {
            let handle = app.handle().clone();

            // System tray để thoát / restart khi Alt+F4 bị chặn
            use tauri::menu::{Menu, MenuItem};
            let quit_item = MenuItem::with_id(&handle, "quit", "Quit Vuget", true, None::<&str>)?;
            let restart_item = MenuItem::with_id(&handle, "restart", "Restart", true, None::<&str>)?;
            let menu = Menu::with_items(&handle, &[&restart_item, &quit_item])?;
            let _tray = tauri::tray::TrayIconBuilder::new()
                .tooltip("Vuget")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    "restart" => app.restart(),
                    _ => {}
                })
                .build(&handle)?;

            // Đặt cửa sổ vào vị trí đã lưu; nếu chưa có thì canh giữa màn hình chính.
            // Cửa sổ khai báo focus:false trong tauri.conf.json → khi autostart (hoặc mở tay)
            // nó hiện ra phía sau app đang dùng, không cướp focus; click vào widget mới focus.
            let window = handle.get_webview_window("main").unwrap();
            {
                let state = handle.state::<AppState>();
                let cfg = state.config.lock().unwrap();
                if let (Some(x), Some(y)) = (cfg.x, cfg.y) {
                    let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
                } else {
                    let _ = window.center();
                }
                sync_autostart(cfg.autostart);
            }

            // Sau mỗi lần kéo/resize xong (debounce 500ms) thì lưu vị trí + kích thước
            static LAST_MOVE: Mutex<Option<std::time::Instant>> = Mutex::new(None);
            fn schedule_save(h: AppHandle) {
                *LAST_MOVE.lock().unwrap() = Some(std::time::Instant::now());
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    let is_last = LAST_MOVE
                        .lock()
                        .unwrap()
                        .map(|t| t.elapsed() >= std::time::Duration::from_millis(500))
                        .unwrap_or(true);
                    if !is_last {
                        return;
                    }
                    if let Some(w) = h.get_webview_window("main") {
                        if let (Ok(pos), Ok(size)) = (w.outer_position(), w.outer_size()) {
                            let state = h.state::<AppState>();
                            let mut cfg = state.config.lock().unwrap();
                            cfg.x = Some(pos.x);
                            cfg.y = Some(pos.y);
                            cfg.width = size.width as i32;
                            cfg.height = size.height as i32;
                            cfg.save();
                        }
                    }
                });
            }
            let h2 = handle.clone();
            window.on_window_event(move |event| {
                match event {
                    tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_) => {
                        schedule_save(h2.clone());
                    }
                    // Widget desktop không nên đóng bằng Alt+F4 — chặn CloseRequested.
                    // Thoát app qua tray menu (nếu có) hoặc task manager nếu thực sự cần.
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            get_lunar_date,
            get_lunar_month,
            get_weather,
            get_sys_info,
            get_notes,
            add_note,
            update_note,
            delete_note,
            toggle_note_pinned,
            toggle_note_hidden,
            reorder_notes,
            sync_pull,
            sync_push,
            sync_new_code,
            sync_unlink,
            sync_prune,
            check_update,
            quit_app,
            restart_app,
            open_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
