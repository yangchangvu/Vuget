const { invoke } = window.__TAURI__.core;

let currentPanel = 0;
const PANELS = ['panel-notes', 'panel-weather', 'panel-sysmon', 'panel-calendar', 'panel-settings', 'panel-about'];
const THEMES = [
  { id: 'red',       bg: '#221418', accent: '#d9665a' },
  { id: 'dark',      bg: '#121218', accent: '#6e8fff' },
  { id: 'midnight',  bg: '#0e1222', accent: '#7c6ee0' },
  { id: 'forest',    bg: '#101e18', accent: '#5fae7e' },
  { id: 'sand',      bg: '#282016', accent: '#d4a55a' },
];
const APP_VERSION = '0.2.3';

const I18N = {
  vi: {
    days: ['Chủ Nhật', 'Thứ Hai', 'Thứ Ba', 'Thứ Tư', 'Thứ Năm', 'Thứ Sáu', 'Thứ Bảy'],
    calHeaders: ['T2', 'T3', 'T4', 'T5', 'T6', 'T7', 'CN'],
    lunarSuffix: 'Âm lịch',
    leap: 'nhuận',
    notes: 'Ghi chú',
    addNote: '+ Thêm',
    addNotePlaceholder: 'Thêm note mới...',
    notesEmpty: 'Chưa có note nào — bấm "+ Thêm" để tạo.',
    noteTitlePlaceholder: 'Tiêu đề...',
    noteBodyPlaceholder: 'Nội dung...',
    pin: 'Ghim',
    hideContent: 'Ẩn nội dung',
    delete: 'Xóa',
    save: 'Lưu',
    cancel: 'Hủy',
    pinned: 'Đã ghim',
    hidden: 'Đã ẩn',
    showMore: 'xem thêm',
    showLess: 'thu gọn',
    weatherToday: 'Thời tiết hôm nay',
    systemMonitor: 'System Monitor',
    today: 'Hôm nay',
    month: 'Tháng',
    settings: 'Thiết lập',
    appearance: 'Giao diện',
    appearanceHint: 'Chọn tông màu',
    opacity: 'Độ trong suốt',
    language: 'Ngôn ngữ',
    autostart: 'Khởi động cùng Windows',
    version: 'Phiên bản',
    dragHint: 'Kéo để di chuyển widget',
    humidity: 'Độ ẩm',
    noData: 'Không có dữ liệu',
    readError: 'Không đọc được dữ liệu',
    hourly: 'Trong ngày',
    daily: 'Những ngày tới',
    cpu: 'CPU',
    ram: 'RAM',
    disk: 'Đĩa',
    network: 'Mạng',
    changelog: 'Lịch sử cập nhật',
  },
  en: {
    days: ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'],
    calHeaders: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
    lunarSuffix: 'Lunar',
    leap: 'leap',
    notes: 'Notes',
    addNote: '+ Add',
    addNotePlaceholder: 'Add a note...',
    notesEmpty: 'No notes yet — click "+ Add" to create.',
    noteTitlePlaceholder: 'Title...',
    noteBodyPlaceholder: 'Content...',
    pin: 'Pin',
    hideContent: 'Hide content',
    delete: 'Delete',
    save: 'Save',
    cancel: 'Cancel',
    pinned: 'Pinned',
    hidden: 'Hidden',
    showMore: 'show more',
    showLess: 'show less',
    weatherToday: "Today's weather",
    systemMonitor: 'System Monitor',
    today: 'Today',
    month: 'Month',
    settings: 'Settings',
    appearance: 'Appearance',
    appearanceHint: 'Choose color tone',
    opacity: 'Opacity',
    language: 'Language',
    autostart: 'Start with Windows',
    version: 'Version',
    dragHint: 'Drag to move widget',
    humidity: 'Humidity',
    noData: 'No data',
    readError: 'Cannot read data',
    hourly: 'Today',
    daily: 'Upcoming days',
    cpu: 'CPU',
    ram: 'RAM',
    disk: 'Disk',
    network: 'Network',
    changelog: 'Changelog',
  },
  zh: {
    days: ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六'],
    calHeaders: ['一', '二', '三', '四', '五', '六', '日'],
    lunarSuffix: '农历',
    leap: '闰',
    notes: '便签',
    addNote: '+ 新增',
    addNotePlaceholder: '添加便签...',
    notesEmpty: '还没有便签 — 点击「+ 新增」创建。',
    noteTitlePlaceholder: '标题...',
    noteBodyPlaceholder: '内容...',
    pin: '置顶',
    hideContent: '隐藏内容',
    delete: '删除',
    save: '保存',
    cancel: '取消',
    pinned: '已置顶',
    hidden: '已隐藏',
    showMore: '更多',
    showLess: '收起',
    weatherToday: '今日天气',
    systemMonitor: '系统监控',
    today: '今天',
    month: '月',
    settings: '设置',
    appearance: '外观',
    appearanceHint: '选择色调',
    opacity: '透明度',
    language: '语言',
    autostart: '开机启动',
    version: '版本',
    dragHint: '拖动以移动小部件',
    humidity: '湿度',
    noData: '无数据',
    readError: '无法读取数据',
    hourly: '今日',
    daily: '未来几天',
    cpu: 'CPU',
    ram: '内存',
    disk: '磁盘',
    network: '网络',
    changelog: '更新日志',
  },
};
let lang = 'vi';
function t(key) { return (I18N[lang] && I18N[lang][key]) || (I18N.vi[key]) || key; }

function applyLang() {
  document.documentElement.lang = lang;
  document.querySelectorAll('[data-i18n]').forEach(el => {
    el.textContent = t(el.dataset.i18n);
  });
  document.querySelectorAll('[data-i18n-ph]').forEach(el => {
    el.placeholder = t(el.dataset.i18nPh);
  });
  document.querySelectorAll('[data-i18n-title]').forEach(el => {
    el.title = t(el.dataset.i18nTitle);
  });
  const inp = document.getElementById('note-input');
  if (inp) inp.placeholder = t('addNotePlaceholder');
}
let config = null;
let calYear, calMonth;

// ---------- Clock ----------
function updateClock() {
  const now = new Date();
  const h = String(now.getHours()).padStart(2, '0');
  const m = String(now.getMinutes()).padStart(2, '0');
  document.getElementById('clock-time').textContent = `${h}:${m}`;

  const days = t('days');
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
        `${res.day}/${res.month}${res.leap ? ' ' + t('leap') : ''} ${t('lunarSuffix')} · ${res.yearName}`;
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
    document.getElementById('weather-humidity').textContent = `${t('humidity')} ${Math.round(res.humidity)}%`;
    document.getElementById('current-temp').textContent = `${Math.round(res.temp)}°C`;
    document.getElementById('current-desc').textContent = res.description;

    const fc = document.getElementById('weather-forecast');
    let html = '';
    if (res.hourly && res.hourly.length) {
      html += `<div class="forecast-title">${t('hourly')}</div><div class="forecast-row">`;
      html += res.hourly.map(h =>
        `<div class="forecast-card"><span class="fc-time">${h.time}</span><span class="fc-icon">${h.icon}</span><span class="fc-temp">${Math.round(h.temp)}°</span></div>`).join('');
      html += '</div>';
    }
    if (res.daily && res.daily.length) {
      html += `<div class="forecast-title">${t('daily')}</div><div class="forecast-row">`;
      html += res.daily.map(d =>
        `<div class="forecast-card"><span class="fc-time">${escapeHtml(d.label)}</span><span class="fc-icon">${d.icon}</span><span class="fc-temp">${Math.round(d.min)}–${Math.round(d.max)}°</span></div>`).join('');
      html += '</div>';
    }
    fc.innerHTML = html;
  } catch (e) {
    document.getElementById('weather-temp').textContent = 'N/A';
    document.getElementById('weather-humidity').textContent = t('noData');
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
        <div class="sysinfo-label">${t('cpu')} · ${escapeHtml(res.cpuName)}</div>
        <div class="sysinfo-value">${res.cpuUsage.toFixed(0)}%</div>
        <div class="sysinfo-bar"><div class="sysinfo-bar-fill" style="width:${res.cpuUsage}%"></div></div>
      </div>
      <div class="sysinfo-item">
        <div class="sysinfo-label">${t('ram')}</div>
        <div class="sysinfo-value">${ramGB} / ${ramTotalGB} GB · ${res.ramPercent.toFixed(0)}%</div>
        <div class="sysinfo-bar"><div class="sysinfo-bar-fill" style="width:${res.ramPercent}%"></div></div>
      </div>
      <div class="sysinfo-item">
        <div class="sysinfo-label">${t('disk')}</div>
        <div class="sysinfo-value">${fmtBytes(res.diskUsed)} / ${fmtBytes(res.diskTotal)} · ${res.diskPercent.toFixed(0)}%</div>
        <div class="sysinfo-bar"><div class="sysinfo-bar-fill" style="width:${res.diskPercent}%"></div></div>
      </div>
      <div class="sysinfo-item">
        <div class="sysinfo-label">${t('network')}</div>
        <div class="sysinfo-value net-row">
          <span class="net-in">↓ ${fmtRate(res.netRx)}</span>
          <span class="net-out">↑ ${fmtRate(res.netTx)}</span>
        </div>
      </div>`;
  } catch (e) {
    document.getElementById('sysinfo-grid').innerHTML = `<div class="sysinfo-item">${t('readError')}</div>`;
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
  title.textContent = `${t('month')} ${calMonth + 1} ${calYear}`;

  const days = t('calHeaders');
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
function stripHtml(html) {
  const d = document.createElement('div');
  d.innerHTML = html;
  return (d.textContent || '').trim();
}

function notePreview(note) {
  if (note.hidden) return '••••••';
  const txt = stripHtml(note.body || note.text || '');
  return txt.slice(0, 60) + (txt.length > 60 ? '…' : '');
}

let notesExpanded = false;
const NOTES_PREVIEW_COUNT = 5;

async function loadNotes() {
  const res = await invoke('get_notes');
  const list = document.getElementById('notes-list');
  list.innerHTML = '';
  const sorted = [...res.items].sort((a, b) => (b.pinned ? 1 : 0) - (a.pinned ? 1 : 0));
  if (sorted.length === 0) {
    list.innerHTML = `<div class="notes-empty">${t('notesEmpty')}</div>`;
    refreshScrollable();
    return;
  }
  const visible = notesExpanded ? sorted : sorted.slice(0, NOTES_PREVIEW_COUNT);
  for (const note of visible) {
    const item = document.createElement('div');
    item.className = 'note-card' + (note.pinned ? ' pinned' : '');
    item.dataset.id = note.id;
    item.innerHTML = `
      <div class="note-grip" title="Kéo để sắp xếp">⠿</div>
      <div class="note-main">
        <div class="note-card-title">${escapeHtml(note.title || t('noteTitlePlaceholder'))}</div>
        <div class="note-card-preview">${escapeHtml(notePreview(note))}</div>
      </div>
      ${note.pinned ? '<span class="note-badge pin">★</span>' : ''}
      ${note.hidden ? '<span class="note-badge hide">⊘</span>' : ''}
      <button class="note-card-delete">✕</button>`;
    item.addEventListener('click', e => {
      if (e.target.closest('.note-grip, .note-card-delete')) return;
      openNoteModal(note.id);
    });
    item.querySelector('.note-card-delete').addEventListener('click', async e => {
      e.stopPropagation();
      await invoke('delete_note', { id: note.id });
      loadNotes();
    });
    list.appendChild(item);
  }
  // Nút xem thêm nếu còn note ẩn
  if (!notesExpanded && sorted.length > NOTES_PREVIEW_COUNT) {
    const more = document.createElement('div');
    more.className = 'notes-more';
    more.innerHTML = `+${sorted.length - NOTES_PREVIEW_COUNT} ${t('showMore')}`;
    more.addEventListener('click', () => { notesExpanded = true; loadNotes(); });
    list.appendChild(more);
  } else if (notesExpanded && sorted.length > NOTES_PREVIEW_COUNT) {
    const less = document.createElement('div');
    less.className = 'notes-more';
    less.textContent = t('showLess');
    less.addEventListener('click', () => { notesExpanded = false; loadNotes(); });
    list.appendChild(less);
  }
  bindNoteDrag(list);
  refreshScrollable();
}

// ---------- Note modal ----------
let currentNoteId = null;

async function openNoteModal(id) {
  currentNoteId = id;
  const res = await invoke('get_notes');
  const note = res.items.find(n => n.id === id);
  if (!note) return;
  const modal = document.getElementById('note-modal');
  document.getElementById('note-modal-title').value = note.title || '';
  const body = document.getElementById('note-modal-body');
  body.innerHTML = note.body || '';
  document.getElementById('nm-pin').classList.toggle('active', note.pinned);
  document.getElementById('nm-hide').classList.toggle('active', note.hidden);
  modal.classList.add('open');
  setTimeout(() => document.getElementById('note-modal-title').focus(), 50);
}

function closeNoteModal() {
  document.getElementById('note-modal').classList.remove('open');
  currentNoteId = null;
}

async function saveCurrentNote() {
  if (currentNoteId == null) return;
  const title = document.getElementById('note-modal-title').value.trim();
  const body = document.getElementById('note-modal-body').innerHTML;
  await invoke('update_note', { id: currentNoteId, title, body });
  closeNoteModal();
  loadNotes();
}

document.getElementById('note-add-btn').addEventListener('click', async () => {
  const id = await invoke('add_note', { title: '', body: '' });
  await loadNotes();
  openNoteModal(id);
});

document.getElementById('nm-save').addEventListener('click', saveCurrentNote);
document.getElementById('nm-cancel').addEventListener('click', closeNoteModal);
document.getElementById('nm-delete').addEventListener('click', async () => {
  if (currentNoteId == null) return;
  await invoke('delete_note', { id: currentNoteId });
  closeNoteModal();
  loadNotes();
});
document.getElementById('nm-pin').addEventListener('click', async function() {
  if (currentNoteId == null) return;
  await invoke('toggle_note_pinned', { id: currentNoteId });
  this.classList.toggle('active');
  loadNotes();
});
document.getElementById('nm-hide').addEventListener('click', async function() {
  if (currentNoteId == null) return;
  await invoke('toggle_note_hidden', { id: currentNoteId });
  this.classList.toggle('active');
  loadNotes();
});

// Toolbar format
document.querySelectorAll('.nt-btn').forEach(btn => {
  btn.addEventListener('click', e => {
    e.preventDefault();
    const cmd = btn.dataset.cmd;
    const val = btn.dataset.val || null;
    const body = document.getElementById('note-modal-body');
    if (cmd === 'insertCheckbox') {
      toggleCheckboxLine(body);
    } else if (cmd === 'formatBlock') {
      document.execCommand('formatBlock', false, val);
    } else {
      document.execCommand(cmd, false, null);
    }
    body.focus();
  });
});

// Toggle checkbox cho dòng hiện tại — rule chặt chẽ, chống spam
let lastCheckboxToggle = 0;

function toggleCheckboxLine(body) {
  // Rule 1: Debounce 250ms — chống spam click
  const now = Date.now();
  if (now - lastCheckboxToggle < 250) return;
  lastCheckboxToggle = now;

  const sel = window.getSelection();
  if (sel.rangeCount === 0) return;

  // Rule 2: Tìm block element chứa caret (div, p, li)
  let block = sel.anchorNode;
  while (block && block !== body) {
    if (block.nodeType === 1 && ['DIV', 'P', 'LI'].includes(block.tagName)) break;
    block = block.parentNode;
  }
  if (!block || block === body) {
    // Fallback: tạo block mới
    document.execCommand('formatBlock', false, 'div');
    block = body.querySelector('div:last-child') || body;
  }

  // Rule 3: Kiểm tra checkbox hiện tại trong block (idempotency)
  const existing = block.querySelector(':scope > input.note-check');
  if (existing) {
    // Đã có → bỏ, không tạo mới
    existing.remove();
    block.classList.remove('note-check-line');
    return;
  }

  // Rule 4: Không tạo trong blockquote (giữ ngữ cảnh quote)
  if (block.tagName === 'LI' || block.closest('blockquote')) {
    // Trong list/blockquote: thêm checkbox vào đầu nội dung, giữ nguyên cấu trúc
    const cb = document.createElement('input');
    cb.type = 'checkbox';
    cb.className = 'note-check';
    block.insertBefore(cb, block.firstChild);
    block.classList.add('note-check-line');
    return;
  }

  // Rule 5: Chỉ tạo khi dòng có text hoặc rỗng (không giữa từ)
  // Thêm checkbox đầu dòng
  const cb = document.createElement('input');
  cb.type = 'checkbox';
  cb.className = 'note-check';
  block.insertBefore(cb, block.firstChild);
  block.classList.add('note-check-line');
}

// Đóng modal: Esc, click overlay = Hủy (không lưu)
document.getElementById('note-modal').addEventListener('click', e => {
  if (e.target.id === 'note-modal') closeNoteModal();
});
document.addEventListener('keydown', e => {
  if (e.key === 'Escape' && document.getElementById('note-modal').classList.contains('open')) {
    closeNoteModal();
  }
});

// ---------- Drag-drop sắp xếp note ----------
function bindNoteDrag(list) {
  let dragEl = null, dragId = null, placeholder = null;

  list.querySelectorAll('.note-grip').forEach(grip => {
    grip.addEventListener('mousedown', e => {
      e.preventDefault();
      const card = grip.closest('.note-card');
      dragEl = card;
      dragId = Number(card.dataset.id);
      card.classList.add('dragging');

      const rect = card.getBoundingClientRect();
      const offsetY = e.clientY - rect.top;

      placeholder = document.createElement('div');
      placeholder.className = 'note-card placeholder';
      placeholder.style.height = rect.height + 'px';

      card.parentNode.insertBefore(placeholder, card);

      card.style.position = 'fixed';
      card.style.zIndex = '1000';
      card.style.width = rect.width + 'px';
      card.style.left = rect.left + 'px';
      card.style.top = (e.clientY - offsetY) + 'px';

      function onMove(ev) {
        card.style.top = (ev.clientY - offsetY) + 'px';
        movePlaceholder(list, ev.clientY);
      }
      function onUp() {
        window.removeEventListener('mousemove', onMove);
        window.removeEventListener('mouseup', onUp);
        if (placeholder.parentNode) {
          list.insertBefore(card, placeholder);
          placeholder.remove();
        }
        card.classList.remove('dragging');
        card.style.cssText = '';
        const ids = [...list.querySelectorAll('.note-card:not(.placeholder)')].map(c => Number(c.dataset.id));
        invoke('reorder_notes', { ids });
        dragEl = null;
        placeholder = null;
      }
      window.addEventListener('mousemove', onMove);
      window.addEventListener('mouseup', onUp);
    });
  });
}

// Tính xem placeholder nên chèn ở đâu: dựa trên item mà chuột đang nằm trong.
// Kéo ngang qua item khác (chưa cần quá nửa) đã swap — ngưỡng 1/2.
function movePlaceholder(list, y) {
  const cards = [...list.querySelectorAll('.note-card:not(.dragging):not(.placeholder)')];
  let target = null, before = true;
  for (const card of cards) {
    const box = card.getBoundingClientRect();
    if (y < box.top + box.height * 0.5) {
      target = card; before = true; break;
    } else if (y < box.bottom) {
      target = card; before = false; break;
    }
  }
  if (target == null) {
    list.appendChild(placeholder);
  } else if (before) {
    list.insertBefore(placeholder, target);
  } else {
    list.insertBefore(placeholder, target.nextSibling);
  }
}

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
  if (PANELS[currentPanel] === 'panel-settings') renderSettings();
  if (PANELS[currentPanel] === 'panel-about') renderAbout();
  refreshScrollable();
}

// Scroll: panel kéo chuột để scroll, wheel đổi panel; chỉ modal body scroll bằng wheel
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
  });
  window.addEventListener('mouseup', () => {
    if (!dragging) return;
    dragging = false;
    el.classList.remove('dragging');
  });
});

function refreshScrollable() {
  panelEls.forEach(el =>
    el.classList.toggle('scrollable', el.scrollHeight > el.clientHeight + 4));
}

document.getElementById('right-panel').addEventListener('wheel', (e) => {
  // Chỉ modal body được scroll bằng wheel, còn lại wheel đổi panel
  if (e.target.closest('#note-modal-body')) return;
  e.preventDefault();
  if (Math.abs(e.deltaY) < 5) return;
  const now = Date.now();
  if (now - lastSwitch < 400) return;
  lastSwitch = now;
  const dir = e.deltaY > 0 ? 1 : -1;
  switchPanel(currentPanel + dir, dir);
}, { passive: false });

// ---------- Theme & Settings ----------
function applyTheme(id) {
  if (id === 'red' || id === undefined) document.documentElement.removeAttribute('data-theme');
  else document.documentElement.setAttribute('data-theme', id);
  document.getElementById('app').style.background = `rgba(${hexToRgb(themeBg(id))}, ${config.opacity})`;
}

function themeBg(id) {
  return (THEMES.find(t => t.id === id) || THEMES[0]).bg;
}

function hexToRgb(hex) {
  const n = parseInt(hex.slice(1), 16);
  return `${(n >> 16) & 255}, ${(n >> 8) & 255}, ${n & 255}`;
}

function buildThemeGrid() {
  const grid = document.getElementById('theme-grid');
  grid.innerHTML = THEMES.map(t => `
    <div class="theme-chip${t.id === config.theme ? ' active' : ''}" data-theme="${t.id}">
      <div class="swatch" style="background:linear-gradient(135deg, ${t.bg} 60%, ${t.accent})"></div>
    </div>`).join('');
  grid.querySelectorAll('.theme-chip').forEach(chip => {
    chip.addEventListener('click', async () => {
      config.theme = chip.dataset.theme;
      applyTheme(config.theme);
      grid.querySelectorAll('.theme-chip').forEach(c => c.classList.toggle('active', c.dataset.theme === config.theme));
      await invoke('save_config', { newConfig: config });
    });
  });
}

function renderSettings() {
  buildThemeGrid();
  const op = document.getElementById('opacity-range');
  const opv = document.getElementById('opacity-value');
  op.value = config.opacity;
  opv.textContent = Math.round(config.opacity * 100) + '%';
  document.getElementById('toggle-autostart').classList.toggle('on', config.autostart);
  document.querySelectorAll('.lang-pill').forEach(p => {
    p.classList.toggle('active', p.dataset.lang === lang);
  });
  refreshScrollable();
}

document.getElementById('opacity-range').addEventListener('input', async e => {
  config.opacity = parseFloat(e.target.value);
  document.getElementById('opacity-value').textContent = Math.round(config.opacity * 100) + '%';
  applyTheme(config.theme);
});
document.getElementById('opacity-range').addEventListener('change', async e => {
  await invoke('save_config', { newConfig: config });
});

document.getElementById('toggle-autostart').addEventListener('click', async function() {
  config.autostart = !config.autostart;
  this.classList.toggle('on', config.autostart);
  await invoke('save_config', { newConfig: config });
});

document.querySelectorAll('.lang-pill').forEach(pill => {
  pill.addEventListener('click', async () => {
    lang = config.language = pill.dataset.lang;
    applyLang();
    updateClock();
    updateLunar();
    renderCalendar();
    renderSettings();
    await invoke('save_config', { newConfig: config });
  });
});

// ---------- About / Changelog ----------
const CHANGELOG = [
  { ver: '0.2.3', date: '2026-08-15', vi: 'Checkbox thông minh (debounce, idempotency, chống spam), single-instance, scrollbar bo tròn, notes list 5 gần nhất + xem thêm, scroll panel bằng kéo chuột.', en: 'Smart checkbox (debounce, idempotency, anti-spam), single-instance, rounded scrollbar, notes list shows 5 recent + expand, panel scroll by drag.', zh: '智能复选框（防抖、幂等、防刷屏）、单实例、圆角滚动条、便签列表显示5条+展开、面板拖拽滚动。' },
  { ver: '0.2.2', date: '2026-08-15', vi: 'Sửa checkbox không tạo ô thừa, thêm nút Hủy, overlay che toàn màn hình, scrollbar đẹp, scroll modal không đổi panel.', en: 'Fixed checkbox creating extra boxes, added Cancel button, full-screen overlay, improved scrollbar, modal scroll does not switch panels.', zh: '修复复选框多余框、新增取消按钮、全屏遮罩、改进滚动条、模态框滚动不切换面板。' },
  { ver: '0.2.1', date: '2026-08-15', vi: 'Nâng cấp Notes: tiêu đề + nội dung, modal editor (bold/checkbox/list), kéo-thả sắp xếp, ghim/ẩn. Chọn ngôn ngữ dạng pill. Hiển thị tác giả.', en: 'Notes upgrade: title + body, modal editor (bold/checkbox/list), drag-drop reorder, pin/hide. Language pills. Author info.', zh: '便签升级：标题+正文、模态编辑器（加粗/勾选/列表）、拖拽排序、置顶/隐藏。语言切换改为按钮。作者信息。' },
  { ver: '0.2.0', date: '2026-08-14', vi: 'Thêm 5 theme (đỏ/tối/đêm/rừng/cát), i18n Việt-Anh-Trung, panel phiên bản.', en: 'Added 5 themes, EN/Vi/Zh i18n, version panel.', zh: '新增 5 主题、中英越三语、版本面板。' },
  { ver: '0.1.2', date: '2026-08-14', vi: 'Khởi động cùng Windows (registry Run), nhớ vị trí cửa sổ.', en: 'Start with Windows, remember window position.', zh: '开机启动、记住窗口位置。' },
  { ver: '0.1.0', date: '2026-08-12', vi: 'Desktop personal widget MVP: đồng hồ, thời tiết, lịch âm, notes, system monitor.', en: 'Desktop widget MVP: clock, weather, lunar calendar, notes, sysmon.', zh: '桌面小组件 MVP：时钟、天气、农历、便签、系统监控。' },
];

function renderAbout() {
  const c = document.getElementById('about-content');
  const entries = CHANGELOG.map(e => {
    const dateStr = e.date;
    const desc = e[lang] || e.vi;
    return `<div class="changelog-item">
      <div class="changelog-ver">v${e.ver}<span class="changelog-date">${dateStr}</span></div>
      <div class="changelog-desc">${escapeHtml(desc)}</div>
    </div>`;
  }).join('');
  c.innerHTML = `
    <div class="about-hero">
      <div class="about-app-row">
        <span class="about-app-name">Vuget</span>
        <span class="about-app-ver">v${APP_VERSION}</span>
      </div>
      <div class="about-author">
        <span class="about-author-name">Dương Trường Vũ</span>
        <span class="about-author-sep">·</span>
        <span class="about-author-meta">yang.changvu@gmail.com</span>
      </div>
    </div>
    <div class="about-section-title">${t('changelog')}</div>
    <div class="changelog-list">${entries}</div>`;
  refreshScrollable();
}

// ---------- Init ----------
async function init() {
  config = await invoke('get_config');
  lang = config.language || 'vi';

  const now = new Date();
  calYear = now.getFullYear();
  calMonth = now.getMonth();

  applyLang();
  applyTheme(config.theme);
  updateClock();
  setInterval(updateClock, 10000);
  updateLunar();

  renderCalendar();
  buildDots();
  updateWeather();
  loadNotes();
  weatherTimer = setInterval(updateWeather, (config.weather_interval_min || 15) * 60000);
  sysTimer = setInterval(updateSysInfo, (config.sysmon_interval_s || 30) * 1000);

  switchPanel(config.default_panel || 0, 0);
}

init();
