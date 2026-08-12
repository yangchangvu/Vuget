# Desktop Personal Widget

Tôi muốn xây một ứng dụng widget nhỏ dành cho Windows Desktop, chủ yếu dùng cho cá nhân và có thể chia sẻ cho bạn bè.

Hãy tự chọn **công nghệ, framework, kiến trúc và thư viện phù hợp nhất**. Không bắt buộc C#, WPF hay bất kỳ công nghệ cụ thể nào.

Ưu tiên theo thứ tự:

1. Nhẹ
2. Mượt
3. Ổn định
4. Ít ảnh hưởng hiệu suất máy
5. Giao diện đẹp, tối giản
6. Dễ build và chia sẻ
7. Dễ mở rộng thêm widget sau này

Không cần over-engineering.

---

# 1. Desktop Widget

Ứng dụng phải là một widget được ghim trực tiếp trên Windows Desktop.

Mục tiêu là cảm giác như một phần của Desktop, không giống một ứng dụng thông thường.

Widget có thể được đặt ở vị trí bất kỳ trên Desktop và ghi nhớ vị trí.

Bố cục tổng thể:

```text
┌──────────────────────────────────────────────────────────────┐
│                 │                                            │
│                 │                                            │
│      LEFT       │                 RIGHT PANEL                │
│                 │                                            │
│                 │                                            │
└──────────────────────────────────────────────────────────────┘
```

Phần bên trái nhỏ hơn phần bên phải.

Tỷ lệ ban đầu có thể khoảng:

```text
Left  : 25–30%
Right : 70–75%
```

Có thể cho phép resize widget.

---

# 2. Left Panel

Phần bên trái là thông tin cơ bản, luôn hiển thị từ trên xuống dưới.

## 2.1. Giờ

Hiển thị giờ hiện tại thật rõ ràng.

Ví dụ:

```text
21:45
```

Có thể hỗ trợ giây nhưng mặc định không cần.

Không cần cập nhật quá mức cần thiết.

---

## 2.2. Ngày dương

Ví dụ:

```text
Thứ Tư
12/08/2026
```

---

## 2.3. Ngày âm

Ví dụ:

```text
19/06 Âm lịch
```

Có thể hiển thị thêm thông tin Can Chi nếu phù hợp.

Lịch âm nên ưu tiên dữ liệu/tính toán local.

Nếu có API đáng tin cậy thì có thể dùng API trước, nhưng ứng dụng không được phụ thuộc hoàn toàn vào Internet.

---

## 2.4. Thời tiết hiện tại

Hiển thị tối giản:

```text
32°C
Độ ẩm 68%
```

Có thể có icon thời tiết.

Ưu tiên lấy từ public API đáng tin cậy.

Không cần API key nếu có nguồn public phù hợp.

Nếu API không khả dụng:

- Dùng nguồn/API khác
- Hoặc fallback sang dữ liệu local nếu có thể
- Hoặc hiển thị trạng thái không có dữ liệu

Không được để lỗi API làm ứng dụng crash.

Không cần cập nhật realtime.

Ví dụ chỉ cập nhật mỗi 10–30 phút là đủ.

---

# 3. Right Panel

Phần bên phải là khu vực hiển thị các loại thông tin khác nhau.

**Chỉ hiển thị một panel tại một thời điểm.**

Không chia thành nhiều card nhỏ cùng lúc.

Ví dụ:

```text
┌────────────────────────────────────────────┐
│                                            │
│              WEATHER PANEL                 │
│                                            │
│        32°C    ☀️                          │
│        Hôm nay                              │
│                                            │
│        23:00  29°C                         │
│        06:00  27°C                         │
│        12:00  33°C                         │
│        18:00  31°C                         │
│                                            │
│        Ngày mai  ☁️  28–33°C               │
│        Ngày kia ☀️  27–34°C                │
│                                            │
└────────────────────────────────────────────┘
```

---

# 4. Chuyển đổi Panel bằng Scroll

Đây là interaction quan trọng.

Khi chuột hover trên **Right Panel**:

- Scroll lên → chuyển sang panel trước
- Scroll xuống → chuyển sang panel tiếp theo

Không cần nút chuyển panel.

Ví dụ:

```text
Scroll ↓

Weather
   ↓
System Monitor
   ↓
Calendar
   ↓
Notes
   ↓
...
```

Scroll ngược lại thì quay về panel trước.

Chuyển panel phải:

- Mượt
- Nhanh
- Có animation nhẹ
- Không gây cảm giác lag
- Không reload toàn bộ application

Có thể dùng fade/slide animation nhẹ.

Không cần animation cầu kỳ.

Khi chuột không nằm trên Right Panel thì scroll của Desktop/ứng dụng khác phải hoạt động bình thường.

---

# 5. Panel 1 – Weather

Panel thời tiết là panel đầu tiên.

Hiển thị:

## Thời tiết hôm nay

- Nhiệt độ hiện tại
- Cảm giác như
- Độ ẩm
- Trạng thái thời tiết
- Icon
- Nhiệt độ cao/thấp
- Có thể thêm lượng mưa nếu dữ liệu có

## Dự báo trong ngày

Hiển thị một số mốc giờ:

```text
09:00  ☀️  29°C
12:00  ☀️  32°C
15:00  ⛅  33°C
18:00  🌧️  30°C
21:00  ☁️  28°C
```

Không cần quá nhiều dữ liệu.

## 2–3 ngày tiếp theo

Ví dụ:

```text
Thứ Năm    ☀️   27–34°C
Thứ Sáu    🌧️   26–31°C
Thứ Bảy    ⛅   27–33°C
```

Ưu tiên API public tốt.

Có thể cho người dùng chọn location.

Nếu không xác định được location:

- Cho phép nhập thủ công
- Không cần GPS nếu không cần thiết

Weather không cần realtime.

Có thể refresh khoảng 15–30 phút hoặc hợp lý hơn tùy API.

Cache dữ liệu.

---

# 6. Panel 2 – System Monitor

Hiển thị thông tin máy tính.

Ví dụ:

```text
CPU
Intel ...
Usage       18%
Temperature 52°C

GPU
RTX ...
Usage       32%
Temperature 48°C

RAM
14.2 / 32 GB
44%
```

Có thể hiển thị thêm:

- CPU usage
- CPU temperature
- GPU usage
- GPU temperature
- RAM usage
- Disk usage
- Network
- FPS nếu có thể

Nhưng không hiển thị tất cả cùng lúc.

Cho phép người dùng chọn khoảng thời gian cập nhật:

```text
1s
3s
5s
10s
30s
```

Mặc định:

```text
30s
```

Đây chỉ là widget để xem vui và tiện theo dõi, **không phải monitoring tool chuyên nghiệp**.

Ưu tiên hiệu suất.

Không được vì widget mà liên tục đọc sensor hoặc GPU data gây ảnh hưởng máy.

Nếu một loại sensor không đọc được thì bỏ qua hoặc hiển thị `N/A`.

Không được crash.

---

# 7. Panel 3 – Calendar

Hiển thị lịch theo tháng.

Có thể chuyển:

- Tháng trước
- Tháng sau
- Về hôm nay

Hiển thị đồng thời:

- Ngày dương
- Ngày âm

Ví dụ:

```text
        THÁNG 8 2026

T2 T3 T4 T5 T6 T7 CN

        1  2
3  4  5  6  7  8  9
10 11 12 13 14 15 16
17 18 19 20 21 22 23
24 25 26 27 28 29 30
31

Ngày hôm nay được highlight.

Ngày âm có thể hiển thị nhỏ bên dưới ngày dương.
```

Không cần animation nặng.

---

# 8. Panel 4 – Personal Notes

Một panel ghi chú cá nhân đơn giản.

Ví dụ:

```text
┌──────────────────────────────────────┐
│ Notes                                │
│                                      │
│ - Mua ổ cứng                         │
│ - Fix login                          │
│ - Nhớ deploy server                  │
│                                      │
└──────────────────────────────────────┘
```

Cho phép:

- Thêm note
- Sửa note
- Xóa note
- Lưu local

Không cần account.

Không cần cloud.

Không cần server.

Dữ liệu note chỉ nằm trên máy người dùng.

---

# 9. Các Panel trong tương lai

Thiết kế hệ thống để sau này có thể thêm:

- Countdown
- Todo
- Network monitor
- Disk monitor
- Battery
- Spotify/music information
- Crypto/stock
- Calendar events
- Random quote
- System uptime
- Internet speed
- Quick launcher
- Personal shortcuts
- AI information
- Other small widgets

Nhưng hiện tại chỉ cần:

1. Weather
2. System Monitor
3. Calendar
4. Notes

Không cần implement những panel tương lai.

---

# 10. Data Strategy

Nguyên tắc rất quan trọng:

**Public API trước, local fallback sau.**

Ví dụ Weather:

```text
Public API
   ↓
Success → cache + display

Failure
   ↓
Alternative API / cached data

Failure
   ↓
Fallback local calculation/data nếu hợp lý

Failure
   ↓
Display "No data"
```

Không được để một API chết làm app chết.

Đối với dữ liệu có thể tính toán local:

- Time → local system time
- Gregorian calendar → local system
- Lunar calendar → local algorithm/library
- System information → local system APIs

Không cần server riêng.

Không thu thập user data.

Không telemetry mặc định.

Không analytics mặc định.

---

# 11. Performance

Đây là một trong những ưu tiên cao nhất.

Widget phải cực kỳ nhẹ.

Đặc biệt:

- Không update realtime nếu không cần
- Không render liên tục khi nội dung không thay đổi
- Không gọi API liên tục
- Không đọc sensor liên tục
- Không polling CPU/GPU mỗi frame
- Không chạy background task nặng
- Cache dữ liệu API
- Chỉ cập nhật panel đang hiển thị nếu có thể
- Panel không hiển thị không cần refresh liên tục

Ví dụ:

Weather:

```text
Refresh mỗi 15–30 phút
```

System Monitor:

```text
Mặc định 30 giây
```

Clock:

```text
Chỉ cập nhật phần giờ cần thiết
```

Calendar:

```text
Không cần cập nhật liên tục
```

Notes:

```text
Chỉ cập nhật khi user thay đổi
```

Mục tiêu là widget có thể chạy cả ngày mà người dùng gần như không cảm nhận được nó đang chạy.

---

# 12. Startup

Có tùy chọn:

```text
Start with Windows
```

Mặc định bật hoặc cho người dùng chọn.

Sau khi Windows khởi động, widget tự xuất hiện trên Desktop.

Không yêu cầu user phải mở thủ công mỗi lần.

---

# 13. Desktop behavior

Widget phải:

- Nằm trên Desktop
- Không che Desktop icons nếu có thể
- Không gây cản trở thao tác Desktop
- Có thể di chuyển
- Nhớ vị trí
- Nhớ kích thước
- Hỗ trợ nhiều màn hình
- Hỗ trợ Windows scaling/DPI
- Không hiện taskbar nếu không cần
- Không tạo cảm giác như một app đang mở bình thường

Có chế độ edit để người dùng di chuyển/rescale widget.

Có thể có click-through mode nếu phù hợp.

---

# 14. UI/UX

Phong cách:

- Tối giản
- Hiện đại
- Sạch
- Không quá nhiều màu
- Không quá nhiều border
- Không giống dashboard doanh nghiệp
- Không giống app monitoring chuyên nghiệp
- Cảm giác như một phần của Desktop

Widget nên có:

- Border radius nhẹ
- Shadow nhẹ
- Transparency/opacity phù hợp
- Typography đẹp
- Khoảng cách thoáng

Không làm UI quá nặng.

---

# 15. Settings

Có menu Settings đơn giản.

Các thiết lập cơ bản:

- Widget position
- Widget size
- Opacity
- Theme
- Font size
- 12/24h
- Show/hide lunar date
- Weather location
- Weather refresh interval
- System monitor refresh interval
- Start with Windows
- Click-through
- Default panel

Có thể lưu tất cả local.

---

# 16. Error handling

Mọi external data đều phải được xem là có thể lỗi.

Ví dụ:

```text
Internet mất
API timeout
API thay đổi
API hết quota
Weather service chết
Sensor không tồn tại
GPU không hỗ trợ
Explorer restart
Monitor disconnect
```

Không được crash.

Không popup error liên tục.

Ưu tiên:

```text
Cached data
↓
Fallback
↓
N/A / No data
```

và tiếp tục chạy bình thường.

---

# 17. Quan trọng nhất

Đây là một **desktop companion widget**, không phải một ứng dụng business.

Không cần phức tạp.

Không cần backend.

Không cần login.

Không cần database server.

Không cần realtime.

Không cần analytics.

Không cần hệ thống plugin phức tạp ngay từ đầu.

Hãy ưu tiên:

**nhẹ + mượt + ổn định + đẹp + tiện.**

Hãy tự chọn công nghệ và implementation phù hợp nhất.

Nếu có nhiều cách triển khai, ưu tiên cách có ít overhead và ít dependency hơn.

Trước tiên hãy xây một MVP hoàn chỉnh với:

```text
┌─────────────────────────────────────────────────────┐
│                 │                                   │
│      CLOCK      │                                   │
│      DATE       │          ONE ACTIVE PANEL         │
│      LUNAR      │                                   │
│      WEATHER    │                                   │
│                 │                                   │
└─────────────────────────────────────────────────────┘
```

Left panel luôn cố định.

Right panel chỉ có **một nội dung tại một thời điểm**.

Hover chuột vào Right Panel + scroll để chuyển panel.

Sau khi MVP hoạt động ổn định, mới bổ sung settings, polish UI và các tính năng phụ.

Không over-engineer.