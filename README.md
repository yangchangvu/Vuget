# Vuget

Desktop personal widget cho Windows — đồng hồ, lịch âm/dương, thời tiết,
system monitor và ghi chú trong một cửa sổ nhỏ, trong suốt, ghim trực tiếp trên desktop.

## Tính năng

**Left panel** (cố định)
- Đồng hồ 24h, ngày dương, thứ
- Ngày âm + can chi — tính local bằng thuật toán thiên văn (không cần mạng), có unit test
- Thời tiết thu gọn (nhiệt độ, độ ẩm, icon)

**Right panel** (một panel tại một thời điểm, scroll để đổi)

| Panel | Nội dung |
|-------|---------|
| **Weather** | Hiện tại + dự báo theo giờ / theo ngày · nguồn [Open-Meteo](https://open-meteo.com) (miễn phí, không API key) |
| **System Monitor** | CPU, RAM, Disk, Network ↓↑ · interval 30s |
| **Calendar** | Lịch tháng dương + ngày âm, highlight hôm nay |
| **Notes** | Thêm/sửa/xóa ghi chú · tiêu đề/nội dung, ghim/ẩn, kéo thả · lưu local |

**Tương tác**
- Hover right panel + scroll → đổi panel (carousel)
- Click dot indicator → nhảy thẳng tới panel
- Kéo left panel / thanh mép trên để di chuyển · vị trí + kích thước tự nhớ
- Nội dung tràn → kéo chuột để cuộn, progress bar mảnh chỉ hiện khi kéo
- Window frameless · trong suốt · không hiện trên taskbar

## Tech

- [Tauri 2](https://tauri.app) — Rust backend + WebView2, exe nhỏ, nhẹ, ổn định
- Frontend HTML/CSS/JS thuần — không framework, không bundler, zero dependency
- `sysinfo` cho CPU/RAM/Disk/Network · `reqwest` + Open-Meteo cho thời tiết
- Lịch âm: thuật toán thiên văn (new moon + đông chí + trung khí, múi giờ +7)

## Run

```sh
cd src-tauri
cargo run              # dev
cargo build --release  # build: src-tauri/target/release/vuget.exe (~4.5 MB)
cargo test             # test lịch âm
```

## Distribute

Bản release là một file exe duy nhất (`src-tauri/target/release/vuget.exe`).
Gửi qua Zalo/Drive/USB — người nhận double-click là chạy.

- **Yêu cầu**: Windows 10/11 có WebView2 (gần như luôn có sẵn; thiếu thì cài [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/))
- **SmartScreen**: exe chưa ký số nên lần đầu Windows sẽ cảnh báo — chọn *More info → Run anyway*
- **Download**: [Releases](https://github.com/yangchangvu/Vuget/releases)

## Cấu trúc

```
src/                      frontend HTML/CSS/JS
├── index.html
├── style.css
└── app.js

src-tauri/                Tauri + Rust backend
├── src/
│   ├── main.rs           Tauri setup + remember window position
│   ├── lunar.rs          thuật toán lịch âm (+ tests)
│   ├── weather.rs        Open-Meteo fetch + forecast
│   ├── sysinfo.rs        CPU/RAM/Disk/Network (2-pass CPU)
│   ├── notes.rs          notes lưu local
│   └── config.rs         settings JSON
├── tauri.conf.json
└── capabilities/
```

Dữ liệu người dùng (settings, notes) lưu tại `%APPDATA%/Vuget/`.
Không telemetry, không analytics.
