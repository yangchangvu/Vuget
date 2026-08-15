#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod lunar;
mod notes;
mod sysinfo;
mod weather;

use chrono::Local;
use serde_json::{json, Value};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

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

            // Đặt cửa sổ vào vị trí đã lưu; nếu chưa có thì canh giữa màn hình chính
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
                if matches!(event, tauri::WindowEvent::Moved(_) | tauri::WindowEvent::Resized(_)) {
                    schedule_save(h2.clone());
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
            reorder_notes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
