use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn notes_path() -> PathBuf {
    let dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("RedWidget");
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
    pub text: String,
}

impl Notes {
    pub fn load() -> Self {
        match fs::read_to_string(notes_path()) {
            Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        if let Ok(s) = serde_json::to_string_pretty(self) {
            fs::write(notes_path(), s).ok();
        }
    }

    pub fn add(&mut self, text: String) -> u32 {
        let id = self.items.iter().map(|n| n.id).max().unwrap_or(0) + 1;
        self.items.push(Note { id, text });
        self.save();
        id
    }

    pub fn update(&mut self, id: u32, text: String) {
        if let Some(note) = self.items.iter_mut().find(|n| n.id == id) {
            note.text = text;
            self.save();
        }
    }

    pub fn remove(&mut self, id: u32) {
        self.items.retain(|n| n.id != id);
        self.save();
    }
}
