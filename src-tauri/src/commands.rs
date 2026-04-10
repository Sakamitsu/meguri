use crate::data::{self, Action, AppData, Settings, StatEntry};
use chrono::Utc;
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

pub type AppState = Mutex<AppData>;

#[tauri::command]
pub fn load_data(state: State<'_, AppState>) -> AppData {
    state.lock().unwrap().clone()
}

#[tauri::command]
pub fn save_settings(settings: Settings, state: State<'_, AppState>) {
    let mut data = state.lock().unwrap();
    data.settings = settings;
    data::save(&data);
}

#[tauri::command]
pub fn get_random_image(state: State<'_, AppState>) -> Option<String> {
    let data = state.lock().unwrap();
    crate::images::get_random_image_base64(&data.settings.images_path)
}

#[tauri::command]
pub fn add_action(name: String, state: State<'_, AppState>) -> Action {
    let mut data = state.lock().unwrap();
    let action = Action {
        id: Uuid::new_v4().to_string(),
        name,
        active: data.actions.is_empty(),
    };
    data.actions.push(action.clone());
    data::save(&data);
    action
}

#[tauri::command]
pub fn update_action(id: String, name: String, state: State<'_, AppState>) {
    let mut data = state.lock().unwrap();
    if let Some(action) = data.actions.iter_mut().find(|a| a.id == id) {
        action.name = name;
    }
    data::save(&data);
}

#[tauri::command]
pub fn delete_action(id: String, state: State<'_, AppState>) {
    let mut data = state.lock().unwrap();
    data.actions.retain(|a| a.id != id);
    if !data.actions.is_empty() && !data.actions.iter().any(|a| a.active) {
        data.actions[0].active = true;
    }
    data::save(&data);
}

#[tauri::command]
pub fn set_active_action(id: String, state: State<'_, AppState>) {
    let mut data = state.lock().unwrap();
    for action in &mut data.actions {
        action.active = action.id == id;
    }
    data::save(&data);
}

#[tauri::command]
pub fn log_session(action_name: String, duration_minutes: u32, state: State<'_, AppState>) {
    let mut data = state.lock().unwrap();
    data.stats.push(StatEntry {
        action_name,
        duration_minutes,
        timestamp: Utc::now(),
    });
    data::save(&data);
}

#[tauri::command]
pub fn get_stats(state: State<'_, AppState>) -> Vec<StatEntry> {
    state.lock().unwrap().stats.clone()
}
