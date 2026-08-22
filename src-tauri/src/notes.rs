use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn notes_path() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Vuget");
    fs::create_dir_all(&dir).ok();
    dir.join("notes.json")
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Notes {
    pub items: Vec<Note>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: u32,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub text: String,
}

impl Notes {
    pub fn load() -> Self {
        let mut notes = match fs::read_to_string(notes_path()) {
            Ok(s) => serde_json::from_str::<Notes>(&s).unwrap_or_default(),
            Err(_) => Self::default(),
        };
        notes.migrate();
        notes
    }

    // Chuyển note cũ (chỉ có text) sang title + body
    fn migrate(&mut self) {
        for n in &mut self.items {
            if n.title.is_empty() && !n.text.is_empty() {
                let mut lines = n.text.lines();
                n.title = lines.next().unwrap_or("").trim().to_string();
                n.body = lines.collect::<Vec<_>>().join("\n");
                n.text.clear();
            }
        }
    }

    pub fn save(&self) {
        if let Ok(s) = serde_json::to_string_pretty(self) {
            fs::write(notes_path(), s).ok();
        }
    }

    pub fn add(&mut self, title: String, body: String) -> u32 {
        let id = self.items.iter().map(|n| n.id).max().unwrap_or(0) + 1;
        self.items.push(Note {
            id,
            title,
            body,
            pinned: false,
            hidden: false,
            text: String::new(),
        });
        self.save();
        id
    }

    pub fn update(&mut self, id: u32, title: String, body: String) {
        if let Some(note) = self.items.iter_mut().find(|n| n.id == id) {
            note.title = title;
            note.body = body;
            self.save();
        }
    }

    pub fn remove(&mut self, id: u32) {
        self.items.retain(|n| n.id != id);
        self.save();
    }

    pub fn toggle_pinned(&mut self, id: u32) {
        if let Some(note) = self.items.iter_mut().find(|n| n.id == id) {
            note.pinned = !note.pinned;
            self.save();
        }
    }

    pub fn toggle_hidden(&mut self, id: u32) {
        if let Some(note) = self.items.iter_mut().find(|n| n.id == id) {
            note.hidden = !note.hidden;
            self.save();
        }
    }

    pub fn reorder(&mut self, ids: Vec<u32>) {
        self.items.sort_by_key(|n| {
            ids.iter().position(|&id| id == n.id).unwrap_or(usize::MAX)
        });
        self.save();
    }

    // Ghi đè toàn bộ danh sách (dùng khi kéo note từ cloud về).
    pub fn set_items(&mut self, items: Vec<Note>) {
        self.items = items;
        self.save();
    }
}
