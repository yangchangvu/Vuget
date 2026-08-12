use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, Disks, Networks, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysInfo {
    pub cpu_name: String,
    pub cpu_usage: f32,
    pub ram_total: u64,
    pub ram_used: u64,
    pub ram_percent: f32,
    pub disk_total: u64,
    pub disk_used: u64,
    pub disk_percent: f32,
    pub net_rx: u64,
    pub net_tx: u64,
}

struct GlobalSys {
    sys: System,
    nets: Networks,
    last: Instant,
}

// CPU usage trên Windows cần 2 lần refresh cách nhau >= 200ms mới có số liệu.
// Giữ state toàn cục để mỗi lần gọi là một "lần đo" kế tiếp, không tạo System mới.
static STATE: Mutex<Option<GlobalSys>> = Mutex::new(None);

pub fn get_sys_info() -> SysInfo {
    let mut state = STATE.lock().unwrap();

    if state.is_none() {
        let mut sys = System::new();
        sys.refresh_cpu_specifics(CpuRefreshKind::nothing().with_cpu_usage());
        sys.refresh_memory();
        *state = Some(GlobalSys {
            sys,
            nets: Networks::new_with_refreshed_list(),
            last: Instant::now(),
        });
    }

    let gs = state.as_mut().unwrap();

    // Đảm bảo khoảng cách tối thiểu giữa 2 lần đo CPU (sysinfo yêu cầu >= 200ms).
    let elapsed = gs.last.elapsed();
    if elapsed < Duration::from_millis(200) {
        std::thread::sleep(Duration::from_millis(200) - elapsed);
    }

    gs.sys.refresh_cpu_specifics(CpuRefreshKind::nothing().with_cpu_usage());
    gs.sys.refresh_memory();
    gs.nets.refresh(true);

    let measure_secs = gs.last.elapsed().as_secs_f64().max(0.05);
    gs.last = Instant::now();

    let cpu_name = gs.sys.cpus().first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".into());
    let cpu_usage = gs.sys.global_cpu_usage();

    let ram_total = gs.sys.total_memory();
    let ram_used = gs.sys.used_memory();
    let ram_percent = if ram_total > 0 {
        (ram_used as f32 / ram_total as f32) * 100.0
    } else {
        0.0
    };

    let disks = Disks::new_with_refreshed_list();
    let disk_total: u64 = disks.list().iter().map(|d| d.total_space()).sum();
    let disk_used: u64 = disks.list().iter().map(|d| d.total_space().saturating_sub(d.available_space())).sum();
    let disk_percent = if disk_total > 0 {
        (disk_used as f32 / disk_total as f32) * 100.0
    } else {
        0.0
    };

    let (rx, tx): (u64, u64) = gs.nets.list().values()
        .fold((0, 0), |(r, t), n| (r + n.received(), t + n.transmitted()));
    let net_rx = (rx as f64 / measure_secs) as u64;
    let net_tx = (tx as f64 / measure_secs) as u64;

    SysInfo {
        cpu_name,
        cpu_usage,
        ram_total,
        ram_used,
        ram_percent,
        disk_total,
        disk_used,
        disk_percent,
        net_rx,
        net_tx,
    }
}
