# RedWidget — Desktop Personal Widget

Widget cá nhân ghim trên Windows Desktop: đồng hồ, lịch âm/dương, thời tiết,
system monitor và ghi chú — trong một cửa sổ nhỏ, trong suốt, tối giản,
cảm giác như một phần của desktop.

```
┌──────────────────────────────────────────────────────────────┐
│  00:28                    │  SYSTEM MONITOR                  │
│  Thứ Năm                  │  ┌ CPU ──────────────── 17% ─   │
│  13/08/2026               │  ┌ RAM ─── 25.0/31.8 GB ───┐ │   │
│  1/7 Âm lịch · Bính Ngọ   │  ┌ Disk ── 1.1/2.3 TB ──── │   │
│                           │  ┌ Network ↓ 8KB/s ↑ 3KB/s┐ │   │
│  🌤 31°C · Độ ẩm 58%      │  •                         │   │
└──────────────────────────────────────────────────────────────┘
```

## Tính năng

**Left panel (cố định)**
- Đồng hồ 24h, ngày dương, thứ
- Ngày âm + can chi năm, hỗ trợ tháng nhuận — tính **local** bằng thuật toán
  thiên văn (không cần Internet), có unit test
- Thời tiết thu gọn (nhiệt độ, độ ẩm, icon)

**Right panel (1 panel tại một thời điểm, scroll để đổi)**
1. **Weather** — thời tiết hiện tại + card dự báo theo giờ (trên) và theo ngày
   (dưới). Nguồn [Open-Meteo](https://open-meteo.com) — miễn phí, không cần API key
2. **System Monitor** — CPU, RAM, Disk, Network in/out. Interval mặc định 30s
3. **Calendar** — lịch tháng dương + ngày âm nhỏ bên dưới, highlight hôm nay
4. **Notes** — thêm/sửa/xóa ghi chú, lưu local

**Tương tác**
- Hover right panel + **scroll** để đổi panel (carousel trượt theo hướng cuộn)
- Hàng **dots** bên trái cho biết đang ở panel nào, bấm để nhảy thẳng tới
- Kéo **left panel** hoặc thanh nắm mép trên để di chuyển widget; vị trí +
  kích thước tự nhớ và khôi phục
- Nội dung tràn: kéo chuột để cuộn, thanh progress mảnh chỉ hiện khi kéo
- Window frameless, trong suốt, không hiện trên taskbar

## Công nghệ

- **Tauri 2** (Rust backend + WebView2) — exe nhỏ, nhẹ, ổn định
- Frontend **HTML/CSS/JS thuần** — không framework, không bundler, zero dependency
- `sysinfo` cho CPU/RAM/Disk/Network; `reqwest` + Open-Meteo cho thời tiết
- Lịch âm: thuật toán thiên văn (new moon + đông chí + trung khí, múi giờ +7),
  kiểm chứng bằng `cargo test`

## Yêu cầu

- Windows 10/11 (WebView2 có sẵn)
- Rust toolchain (rustup) + MSVC build tools

## Chạy

```bash
cd src-tauri
cargo run              # dev
cargo build --release  # bản release: target/release/redwidget.exe
```

Test lịch âm:

```bash
cargo test
```

## Chia sẻ cho bạn bè

Bản release là **1 file exe duy nhất** (~4.5 MB), không cần cài đặt:

```
src-tauri/target/release/redwidget.exe
```

Gửi file này qua Zalo/Drive/USB — người nhận double-click là chạy.
Yêu cầu máy nhận: Windows 10/11 có WebView2 (gần như luôn có sẵn;
nếu thiếu thì cài WebView2 Runtime từ Microsoft).
Vì exe chưa ký số, lần đầu Windows SmartScreen sẽ cảnh báo —
chọn *More info → Run anyway*.

## Cấu trúc

```
├── src/                  # frontend (HTML/CSS/JS thuần)
│   ├── index.html
│   ├── style.css
│   └── app.js
└── src-tauri/
    ├── src/
    │   ├── main.rs       # Tauri commands + window setup + nhớ vị trí
    │   ├── lunar.rs      # thuật toán lịch âm (+ tests)
    │   ├── weather.rs    # Open-Meteo fetch + parse forecast
    │   ├── sysinfo.rs    # CPU/RAM/Disk/Network (2-pass CPU)
    │   ├── notes.rs      # notes lưu local
    │   └── config.rs     # settings lưu JSON
    ├── tauri.conf.json
    └── capabilities/
```

Dữ liệu người dùng (settings, notes) lưu trong `%APPDATA%/RedWidget/`.
Không telemetry, không analytics.

## Roadmap

- Settings UI (opacity, theme, font size, 12/24h, interval, default panel)
- Chọn vị trí thời tiết (geocoding ưu tiên địa danh Việt Nam)
- Start with Windows, click-through mode
- Panel mới: countdown, todo, battery, Spotify, crypto...
