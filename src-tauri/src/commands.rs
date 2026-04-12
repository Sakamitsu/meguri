use crate::data::{self, Action, AppData, Settings, StatEntry};
use chrono::Utc;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
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

const WIDGET_SIZE: f64 = 160.0;
const TIKTOK_WIDTH: f64 = 244.0;
const TIKTOK_HEIGHT: f64 = 270.0;
const TITLEBAR_HEIGHT: f64 = 28.0;
const CONTENT_HEIGHT: f64 = TIKTOK_HEIGHT - TITLEBAR_HEIGHT;

#[tauri::command]
pub async fn open_tiktok(app: AppHandle, widget_position: String) -> Result<(), String> {
    if app.get_webview("tiktok").is_some() {
        return Ok(());
    }

    let main_window = app.get_window("main").ok_or("Main window not found")?;
    let tiktok_above = widget_position.contains("bottom");
    let widget_right = widget_position.contains("right");

    // Current widget position on screen
    let pos = main_window.outer_position().map_err(|e| e.to_string())?;
    let scale = main_window.scale_factor().map_err(|e| e.to_string())?;
    let cx = pos.x as f64 / scale;
    let cy = pos.y as f64 / scale;

    // Keep widget visually in place after expansion
    let new_x = if widget_right { cx - (TIKTOK_WIDTH - WIDGET_SIZE) } else { cx };
    let new_y = if tiktok_above { cy - TIKTOK_HEIGHT } else { cy };

    // TikTok webview Y within the expanded window
    let webview_y = if tiktok_above { TITLEBAR_HEIGHT } else { WIDGET_SIZE + TITLEBAR_HEIGHT };

    // Resize & reposition main window
    main_window
        .set_size(tauri::LogicalSize::new(TIKTOK_WIDTH, WIDGET_SIZE + TIKTOK_HEIGHT))
        .map_err(|e| e.to_string())?;
    main_window
        .set_position(tauri::LogicalPosition::new(new_x, new_y))
        .map_err(|e| e.to_string())?;

    // Add TikTok as child webview inside main window
    let url = "https://www.tiktok.com/foryou"
        .parse()
        .map_err(|e| format!("Invalid URL: {}", e))?;

    main_window
        .add_child(
            tauri::webview::WebviewBuilder::new("tiktok", tauri::WebviewUrl::External(url))
                .devtools(true)
                .on_navigation(|_url| true),
            tauri::LogicalPosition::new(0.0, webview_y),
            tauri::LogicalSize::new(TIKTOK_WIDTH, CONTENT_HEIGHT),
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn close_tiktok(app: AppHandle, widget_position: String) -> Result<(), String> {
    if let Some(webview) = app.get_webview("tiktok") {
        webview.close().map_err(|e| e.to_string())?;
    }

    let main_window = app.get_window("main").ok_or("Main window not found")?;
    let tiktok_above = widget_position.contains("bottom");
    let widget_right = widget_position.contains("right");

    let pos = main_window.outer_position().map_err(|e| e.to_string())?;
    let scale = main_window.scale_factor().map_err(|e| e.to_string())?;
    let cx = pos.x as f64 / scale;
    let cy = pos.y as f64 / scale;

    // Reverse the expansion offset
    let new_x = if widget_right { cx + (TIKTOK_WIDTH - WIDGET_SIZE) } else { cx };
    let new_y = if tiktok_above { cy + TIKTOK_HEIGHT } else { cy };

    main_window
        .set_size(tauri::LogicalSize::new(WIDGET_SIZE, WIDGET_SIZE))
        .map_err(|e| e.to_string())?;
    main_window
        .set_position(tauri::LogicalPosition::new(new_x, new_y))
        .map_err(|e| e.to_string())?;

    Ok(())
}

