use chrono::{Datelike, NaiveDate, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourPoint {
    pub time: String,
    pub temp: f64,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayPoint {
    pub label: String,
    pub icon: String,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub temp: f64,
    pub humidity: f64,
    pub code: u32,
    pub description: String,
    pub icon: String,
    pub hourly: Vec<HourPoint>,
    pub daily: Vec<DayPoint>,
    pub timestamp: u64,
}

const VN_DAYS: [&str; 7] = ["Chủ Nhật", "Thứ Hai", "Thứ Ba", "Thứ Tư", "Thứ Năm", "Thứ Sáu", "Thứ Bảy"];

pub async fn fetch_weather(lat: f64, lon: f64) -> Result<WeatherData, String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}\
         &current=temperature_2m,relative_humidity_2m,weather_code\
         &hourly=temperature_2m,weather_code\
         &daily=weather_code,temperature_2m_max,temperature_2m_min\
         &timezone=auto&forecast_days=4",
        lat, lon
    );

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get(&url)
        .header("User-Agent", "RedWidget/0.1")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }

    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

    let temp = json["current"]["temperature_2m"]
        .as_f64()
        .ok_or("Missing temperature")?;
    let humidity = json["current"]["relative_humidity_2m"]
        .as_f64()
        .ok_or("Missing humidity")?;
    let code = json["current"]["weather_code"]
        .as_u64()
        .ok_or("Missing weather code")? as u32;

    let (description, icon) = weather_code_to_info(code);

    // 4 mốc giờ sắp tới, cách nhau 3 tiếng
    let mut hourly = Vec::new();
    let now_prefix = Utc::now().format("%Y-%m-%dT%H").to_string();
    if let (Some(times), Some(temps), Some(codes)) = (
        json["hourly"]["time"].as_array(),
        json["hourly"]["temperature_2m"].as_array(),
        json["hourly"]["weather_code"].as_array(),
    ) {
        for (i, t) in times.iter().enumerate() {
            if hourly.len() >= 4 {
                break;
            }
            let Some(ts) = t.as_str() else { continue };
            // "2026-08-13T15:00" — so sánh chuỗi được vì zero-padded
            let hour: u32 = ts.get(11..13).and_then(|h| h.parse().ok()).unwrap_or(0);
            if ts.len() >= 13 && &ts[..13] > now_prefix.as_str() && hour % 3 == 0 {
                hourly.push(HourPoint {
                    time: format!("{:02}:00", hour),
                    temp: temps.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0),
                    icon: weather_code_to_info(codes.get(i).and_then(|v| v.as_u64()).unwrap_or(0) as u32).1,
                });
            }
        }
    }

    // 3 ngày tiếp theo (bỏ hôm nay)
    let mut daily = Vec::new();
    if let (Some(times), Some(maxs), Some(mins), Some(codes)) = (
        json["daily"]["time"].as_array(),
        json["daily"]["temperature_2m_max"].as_array(),
        json["daily"]["temperature_2m_min"].as_array(),
        json["daily"]["weather_code"].as_array(),
    ) {
        for i in 1..4.min(times.len()) {
            let Some(ts) = times[i].as_str() else { continue };
            let label = NaiveDate::parse_from_str(ts, "%Y-%m-%d")
                .ok()
                .map(|d| VN_DAYS[d.weekday().num_days_from_sunday() as usize].to_string())
                .unwrap_or_else(|| ts.to_string());
            daily.push(DayPoint {
                label,
                icon: weather_code_to_info(codes.get(i).and_then(|v| v.as_u64()).unwrap_or(0) as u32).1,
                min: mins.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0),
                max: maxs.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0),
            });
        }
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(WeatherData {
        temp,
        humidity,
        code,
        description,
        icon,
        hourly,
        daily,
        timestamp,
    })
}

fn weather_code_to_info(code: u32) -> (String, String) {
    match code {
        0 => ("Trời trong".into(), "☀️".into()),
        1 => ("Ít mây".into(), "🌤️".into()),
        2 => ("Mây rải rác".into(), "⛅".into()),
        3 => ("Nhiều mây".into(), "☁️".into()),
        45 | 48 => ("Sương mù".into(), "🌫️".into()),
        51..=57 => ("Mưa phùn".into(), "🌦️".into()),
        61..=67 => ("Mưa".into(), "🌧️".into()),
        71..=77 => ("Tuyết".into(), "❄️".into()),
        80..=82 => ("Mưa rào".into(), "🌧️".into()),
        95 | 96 | 99 => ("Dông".into(), "⛈️".into()),
        _ => ("Không rõ".into(), "❓".into()),
    }
}
