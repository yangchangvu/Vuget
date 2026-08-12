const { invoke } = window.__TAURI__.core;

let currentPanel = 0;
const PANELS = ['panel-weather', 'panel-sysmon', 'panel-calendar', 'panel-notes'];
let config = null;
let calYear, calMonth;

// ---------- Clock ----------
function updateClock() {
  const now = new Date();
  const h = String(now.getHours()).padStart(2, '0');
  const m = String(now.getMinutes()).padStart(2, '0');
  document.getElementById('clock-time').textContent = `${h}:${m}`;

  const days = ['Chủ Nhật', 'Thứ Hai', 'Thứ Ba', 'Thứ Tư', 'Thứ Năm', 'Thứ Sáu', 'Thứ Bảy'];
  const dd = String(now.getDate()).padStart(2, '0');
  const mm = String(now.getMonth() + 1).padStart(2, '0');
  const yyyy = now.getFullYear();
  document.getElementById('clock-date').innerHTML = `${days[now.getDay()]}<br>${dd}/${mm}/${yyyy}`;
}

async function updateLunar() {
  try {
    const res = await invoke('get_lunar_date');
    if (res) {
      document.getElementById('lunar-date').textContent =
        `${res.day}/${res.month}${res.leap ? ' nhuận' : ''} Âm lịch · ${res.yearName}`;
    }
  } catch (e) {
    document.getElementById('lunar-date').textContent = '';
  }
}

// ---------- Weather ----------
let weatherTimer = null;
async function updateWeather() {
  try {
    const res = await invoke('get_weather', {
      lat: config.weather_lat,
      lon: config.weather_lon,
    });
    document.getElementById('weather-icon').textContent = res.icon;
    document.getElementById('weather-temp').textContent = `${Math.round(res.temp)}°C`;
    document.getElementById('weather-humidity').textContent = `Độ ẩm ${Math.round(res.humidity)}%`;
    document.getElementById('current-temp').textContent = `${Math.round(res.temp)}°C`;
    document.getElementById('current-desc').textContent = res.description;

    const fc = document.getElementById('weather-forecast');
    let html = '';
    if (res.hourly && res.hourly.length) {
      html += '<div class="forecast-title">Trong ngày</div><div class="forecast-row">';
      html += res.hourly.map(h =>
        `<div class="forecast-card"><span class="fc-time">${h.time}</span><span class="fc-icon">${h.icon}</span><span class="fc-temp">${Math.round(h.temp)}°</span></div>`).join('');
      html += '</div>';
    }
    if (res.daily && res.daily.length) {
      html += '<div class="forecast-title">Những ngày tới</div><div class="forecast-row">';
      html += res.daily.map(d =>
        `<div class="forecast-card"><span class="fc-time">${escapeHtml(d.label)}</span><span class="fc-icon">${d.icon}</span><span class="fc-temp">${Math.round(d.min)}–${Math.round(d.max)}°</span></div>`).join('');
      html += '</div>';
    }
    fc.innerHTML = html;
  } catch (e) {
    document.getElementById('weather-temp').textContent = 'N/A';
    document.getElementById('weather-humidity').textContent = 'Không có dữ liệu';
  }
}

// ---------- System Monitor ----------
let sysTimer = null;
async function updateSysInfo() {
  try {
    const res = await invoke('get_sys_info');
    const ramGB = (res.ramUsed / 2**30).toFixed(1);
    const ramTotalGB = (res.ramTotal / 2**30).toFixed(1);
    document.getElementById('sysinfo-grid').innerHTML = `
      <div class="sysinfo-item">
        <div class="sysinfo-label">CPU · ${escapeHtml(res.cpuName)}</div>
        <div class="sysinfo-value">${res.cpuUsage.toFixed(0)}%</div>
        <div class="sysinfo-bar"><div class="sysinfo-bar-fill" style="width:${res.cpuUsage}%"></div></div>
      </div>
      <div class="sysinfo-item">
        <div class="sysinfo-label">RAM</div>
        <div class="sysinfo-value">${ramGB} / ${ramTotalGB} GB · ${res.ramPercent.toFixed(0)}%</div>
        <div class="sysinfo-bar"><div class="sysinfo-bar-fill" style="width:${res.ramPercent}%"></div></div>
      </div>
      <div class="sysinfo-item">
        <div class="sysinfo-label">Disk</div>
        <div class="sysinfo-value">${fmtBytes(res.diskUsed)} / ${fmtBytes(res.diskTotal)} · ${res.diskPercent.toFixed(0)}%</div>
        <div class="sysinfo-bar"><div class="sysinfo-bar-fill" style="width:${res.diskPercent}%"></div></div>
      </div>
      <div class="sysinfo-item">
        <div class="sysinfo-label">Network</div>
        <div class="sysinfo-value net-row">
          <span class="net-in">↓ ${fmtRate(res.netRx)}</span>
          <span class="net-out">↑ ${fmtRate(res.netTx)}</span>
        </div>
      </div>`;
  } catch (e) {
    document.getElementById('sysinfo-grid').innerHTML = '<div class="sysinfo-item">Không đọc được dữ liệu</div>';
  }
  refreshScrollable();
}

function fmtBytes(b) {
  if (b >= 2**40) return (b / 2**40).toFixed(1) + ' TB';
  if (b >= 2**30) return (b / 2**30).toFixed(0) + ' GB';
  return (b / 2**20).toFixed(0) + ' MB';
}

function fmtRate(bps) {
  if (bps >= 2**30) return (bps / 2**30).toFixed(1) + ' GB/s';
  if (bps >= 2**20) return (bps / 2**20).toFixed(1) + ' MB/s';
  if (bps >= 1024) return (bps / 1024).toFixed(0) + ' KB/s';
  return bps + ' B/s';
}

// ---------- Calendar ----------
async function renderCalendar() {
  const grid = document.getElementById('calendar-grid');
  const title = document.getElementById('cal-title');
  title.textContent = `Tháng ${calMonth + 1} ${calYear}`;

  const days = ['T2', 'T3', 'T4', 'T5', 'T6', 'T7', 'CN'];
  let html = days.map(d => `<div class="cal-day-header">${d}</div>`).join('');

  const first = new Date(calYear, calMonth, 1);
  let startDay = first.getDay() - 1;
  if (startDay < 0) startDay = 6;
  const daysInMonth = new Date(calYear, calMonth + 1, 0).getDate();
  const prevDays = new Date(calYear, calMonth, 0).getDate();

  const today = new Date();
  const isCurrentMonth = today.getFullYear() === calYear && today.getMonth() === calMonth;

  // Ngày âm cho từng ngày trong tháng (tính local trong Rust)
  let lunar = [];
  try {
    lunar = await invoke('get_lunar_month', { year: calYear, month: calMonth + 1 });
  } catch (e) { lunar = []; }

  for (let i = 0; i < startDay; i++) {
    html += `<div class="cal-day other-month">${prevDays - startDay + 1 + i}</div>`;
  }
  for (let d = 1; d <= daysInMonth; d++) {
    const isToday = isCurrentMonth && d === today.getDate();
    const ld = lunar[d - 1];
    const lunarHtml = ld ? `<div class="cal-day-lunar">${ld.day === 1 ? (ld.leap ? 'N' : '') + ld.month + '/' : ''}${ld.day}</div>` : '';
    html += `<div class="cal-day${isToday ? ' today' : ''}">${d}${lunarHtml}</div>`;
  }
  grid.innerHTML = html;
  refreshScrollable();
}

document.getElementById('cal-prev').addEventListener('click', () => {
  calMonth--; if (calMonth < 0) { calMonth = 11; calYear--; } renderCalendar();
});
document.getElementById('cal-next').addEventListener('click', () => {
  calMonth++; if (calMonth > 11) { calMonth = 0; calYear++; } renderCalendar();
});
document.getElementById('cal-today').addEventListener('click', () => {
  const now = new Date(); calYear = now.getFullYear(); calMonth = now.getMonth(); renderCalendar();
});

// ---------- Notes ----------
async function loadNotes() {
  const res = await invoke('get_notes');
  const list = document.getElementById('notes-list');
  list.innerHTML = '';
  if (res.items.length === 0) {
    list.innerHTML = '<div class="notes-empty">Chưa có note nào — gõ vào ô phía trên để thêm.</div>';
    refreshScrollable();
    return;
  }
  for (const note of res.items) {
    const item = document.createElement('div');
    item.className = 'note-item';
    item.innerHTML = `
      <div class="note-text" contenteditable="true">${escapeHtml(note.text)}</div>
      <button class="note-delete">✕</button>`;
    const textEl = item.querySelector('.note-text');
    textEl.addEventListener('blur', async () => {
      const newText = textEl.textContent.trim();
      if (newText !== note.text) await invoke('update_note', { id: note.id, text: newText });
    });
    item.querySelector('.note-delete').addEventListener('click', async () => {
      await invoke('delete_note', { id: note.id });
      loadNotes();
    });
    list.appendChild(item);
  }
  refreshScrollable();
}

async function addNoteFromInput() {
  const input = document.getElementById('note-input');
  const text = input.value.trim();
  if (!text) return;
  await invoke('add_note', { text });
  input.value = '';
  loadNotes();
}

document.getElementById('add-note-btn').addEventListener('click', addNoteFromInput);
document.getElementById('note-input').addEventListener('keydown', e => {
  if (e.key === 'Enter') addNoteFromInput();
});

function escapeHtml(s) {
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

// ---------- Panel switching ----------
let lastSwitch = 0;
const panelEls = PANELS.map(id => document.getElementById(id));
const dotsEl = document.getElementById('panel-dots');

function buildDots() {
  dotsEl.innerHTML = PANELS.map((_, i) => `<span class="dot" data-i="${i}"></span>`).join('');
}

// Bấm vào dot để nhảy thẳng tới panel
dotsEl.addEventListener('click', e => {
  const dot = e.target.closest('.dot');
  if (!dot) return;
  const i = Number(dot.dataset.i);
  if (i !== currentPanel) switchPanel(i, i > currentPanel ? 1 : -1);
});

function updateDots() {
  dotsEl.querySelectorAll('.dot').forEach((d, i) =>
    d.classList.toggle('active', i === currentPanel));
}

function switchPanel(idx, dir) {
  const prev = currentPanel;
  currentPanel = ((idx % PANELS.length) + PANELS.length) % PANELS.length;
  if (currentPanel === prev) {
    const t = panelEls[currentPanel];
    if (!t.classList.contains('active')) {
      t.classList.add('active');
      updateDots();
      refreshScrollable();
    }
    return;
  }
  const d = dir !== undefined ? dir : (currentPanel > prev ? 1 : -1);

  const prevEl = panelEls[prev];
  const target = panelEls[currentPanel];

  prevEl.classList.remove('active');
  prevEl.style.zIndex = '1';
  prevEl.style.animation = `${d > 0 ? 'slideOutToTop' : 'slideOutToBottom'} 0.2s cubic-bezier(0.3, 0.7, 0.4, 1) both`;
  target.classList.add('active');
  target.style.zIndex = '2';
  target.style.animation = `${d > 0 ? 'slideInFromBottom' : 'slideInFromTop'} 0.2s cubic-bezier(0.3, 0.7, 0.4, 1) both`;
  for (const el of [prevEl, target]) {
    el.addEventListener('animationend', () => {
      el.style.animation = '';
      el.style.zIndex = '';
    }, { once: true });
  }
  updateDots();

  if (PANELS[currentPanel] === 'panel-sysmon') updateSysInfo();
  if (PANELS[currentPanel] === 'panel-weather') updateWeather();
  if (PANELS[currentPanel] === 'panel-notes') loadNotes();
  refreshScrollable();
}

// Kéo chuột để cuộn nội dung khi panel tràn (wheel vẫn dành cho đổi panel)
const rightPanelEl = document.getElementById('right-panel');
const scrollThumb = document.getElementById('scroll-thumb');
let scrollHideTimer = null;

function updateScrollProgress(el) {
  const max = el.scrollHeight - el.clientHeight;
  if (max <= 0) return;
  const trackH = el.clientHeight;
  const thumbH = Math.max(28, trackH * (el.clientHeight / el.scrollHeight));
  scrollThumb.style.height = thumbH + 'px';
  scrollThumb.style.top = (el.scrollTop / max) * (trackH - thumbH) + 'px';
}

function flashScrollProgress(el) {
  updateScrollProgress(el);
  rightPanelEl.classList.add('scrolling');
  clearTimeout(scrollHideTimer);
  scrollHideTimer = setTimeout(() => rightPanelEl.classList.remove('scrolling'), 700);
}

panelEls.forEach(el => {
  let dragging = false, startY = 0, startScroll = 0;
  el.addEventListener('mousedown', e => {
    if (e.target.closest('button, [contenteditable], input, textarea, a')) return;
    if (el.scrollHeight <= el.clientHeight + 4) return;
    dragging = true;
    startY = e.clientY;
    startScroll = el.scrollTop;
    el.classList.add('dragging');
    e.preventDefault();
  });
  window.addEventListener('mousemove', e => {
    if (!dragging) return;
    el.scrollTop = startScroll - (e.clientY - startY);
    flashScrollProgress(el);
  });
  window.addEventListener('mouseup', () => {
    if (!dragging) return;
    dragging = false;
    el.classList.remove('dragging');
    clearTimeout(scrollHideTimer);
    scrollHideTimer = setTimeout(() => rightPanelEl.classList.remove('scrolling'), 700);
  });
});

function refreshScrollable() {
  panelEls.forEach(el =>
    el.classList.toggle('scrollable', el.scrollHeight > el.clientHeight + 4));
}

document.getElementById('right-panel').addEventListener('wheel', (e) => {
  e.preventDefault();
  if (Math.abs(e.deltaY) < 5) return;
  const now = Date.now();
  if (now - lastSwitch < 400) return;
  lastSwitch = now;
  const dir = e.deltaY > 0 ? 1 : -1;
  switchPanel(currentPanel + dir, dir);
}, { passive: false });

// ---------- Init ----------
async function init() {
  config = await invoke('get_config');

  const now = new Date();
  calYear = now.getFullYear();
  calMonth = now.getMonth();

  updateClock();
  setInterval(updateClock, 10000);
  updateLunar();

  renderCalendar();
  loadNotes();
  buildDots();
  updateWeather();
  weatherTimer = setInterval(updateWeather, (config.weather_interval_min || 15) * 60000);
  sysTimer = setInterval(updateSysInfo, (config.sysmon_interval_s || 30) * 1000);

  switchPanel(config.default_panel || 0, 0);
}

init();
