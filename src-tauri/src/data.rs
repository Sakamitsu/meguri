use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub images_path: String,
    pub timer_minutes: u32,
    pub confirmation_minutes: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            images_path: String::new(),
            timer_minutes: 10,
            confirmation_minutes: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatEntry {
    pub action_name: String,
    pub duration_minutes: u32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub settings: Settings,
    pub actions: Vec<Action>,
    pub stats: Vec<StatEntry>,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            actions: Vec::new(),
            stats: Vec::new(),
        }
    }
}

pub fn data_file_path() -> PathBuf {
    std::env::current_exe()
        .expect("cannot determine exe path")
        .parent()
        .expect("exe has no parent dir")
        .join("meguri_data.json")
}

pub fn load_or_create() -> AppData {
    let path = data_file_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        let data = AppData::default();
        save(&data);
        data
    }
}

pub fn save(data: &AppData) {
    let path = data_file_path();
    let tmp = path.with_extension("json.tmp");
    let content = serde_json::to_string_pretty(data).expect("failed to serialize data");
    fs::write(&tmp, content).expect("failed to write tmp data file");
    fs::rename(&tmp, &path).expect("failed to rename tmp data file");
}
