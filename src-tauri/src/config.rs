use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn config_dir() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("RedWidget");
    fs::create_dir_all(&dir).ok();
    dir
}

fn config_path() -> PathBuf {
    config_dir().join("settings.json")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: i32,
    pub height: i32,
    pub opacity: f64,
    pub font_size: u32,
    pub clock_24h: bool,
    pub show_lunar: bool,
    pub autostart: bool,
    pub weather_lat: f64,
    pub weather_lon: f64,
    pub weather_location: String,
    pub weather_interval_min: u64,
    pub sysmon_interval_s: u64,
    pub default_panel: u32,
    pub theme: String,
    pub language: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            width: 620,
            height: 420,
            opacity: 0.92,
            font_size: 14,
            clock_24h: true,
            show_lunar: true,
            autostart: true,
            weather_lat: 16.0544,
            weather_lon: 108.2022,
            weather_location: String::from("Đà Nẵng"),
            weather_interval_min: 15,
            sysmon_interval_s: 30,
            default_panel: 0,
            theme: String::from("red"),
            language: String::from("vi"),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        match fs::read_to_string(config_path()) {
            Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        if let Ok(s) = serde_json::to_string_pretty(self) {
            fs::write(config_path(), s).ok();
        }
    }
}
