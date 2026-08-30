<!-- BEGIN BUILD-LOCATION -->
## Chỗ để bản build dùng được (shared builds folder)

Bản build Vuget đã sẵn sàng để chạy/ cài đặt trên máy này nằm ở `C:/Users/Valentines/Desktop/dev/Exts/builds/Vuget/` (mỗi project một thư mục con; thư mục gốc `builds/` chỉ chứa script copy `collect.ps1` — không có file inventory nào). Sau khi build THÀNH CÔNG, hãy copy artifact vào đó — `target/release` trong source tree giữ nguyên chỗ của nó, còn bản trong `builds/` mới là thứ mọi người thực sự chạy.

```powershell
# Build (Tauri 2, crate tên vuget; frontend là HTML/CSS/JS tĩnh trong src/, không có bundler, không cần npm install)
cd C:/Users/Valentines/Desktop/dev/Exts/Vuget/src-tauri
cargo build --release

# Copy bản dùng được vào shared builds folder
Copy-Item target/release/vuget.exe C:/Users/Valentines/Desktop/dev/Exts/builds/Vuget/ -Force
# Hoặc chạy gộp cho cả 4 project: powershell -File C:/Users/Valentines/Desktop/dev/Exts/builds/collect.ps1
```

- Artifact: `vuget.exe` — file thực thi đơn lẻ, self-contained (~5 MB), phân phối trực tiếp được.
- Điều kiện build: chỉ cần Rust/Cargo; tùy chọn đặt env var `VUGET_FIREBASE_URL` lúc compile (build.rs + `src-tauri/.env.example`) để bật đồng bộ note qua Firebase — build thiếu nó vẫn chạy nhưng tính năng sync báo "not configured".
- Test thuật toán lịch âm: `cd src-tauri && cargo test` (unit tests trong `src-tauri/src/lunar.rs`).
- Dữ liệu người dùng (settings, notes) nằm ở `%APPDATA%/Vuget/`.
- Release chính thức do GitHub Actions sinh ra: `.github/workflows/release.yml`.
- `C:/Users/Valentines/Desktop/dev/Exts/builds/` là output sinh ra, KHÔNG bao giờ commit vào repo — nó nằm ngoài repo nên không cần dòng gitignore nào.
<!-- END BUILD-LOCATION -->
