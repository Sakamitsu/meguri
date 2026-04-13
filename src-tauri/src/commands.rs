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

#[derive(serde::Serialize)]
pub struct ImageResponse {
    pub data_url: String,
    pub path: String,
}

#[tauri::command]
pub fn get_random_image(state: State<'_, AppState>) -> Option<ImageResponse> {
    let data = state.lock().unwrap();
    crate::images::get_random_image_base64(&data.settings.images_path).map(|r| ImageResponse {
        data_url: r.data_url,
        path: r.path,
    })
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

fn expand_window(app: &AppHandle, widget_position: &str) -> Result<(), String> {
    let main_window = app.get_window("main").ok_or("Main window not found")?;
    let tiktok_above = widget_position.contains("bottom");
    let widget_right = widget_position.contains("right");

    let pos = main_window.outer_position().map_err(|e| e.to_string())?;
    let scale = main_window.scale_factor().map_err(|e| e.to_string())?;
    let cx = pos.x as f64 / scale;
    let cy = pos.y as f64 / scale;

    let new_x = if widget_right { cx - (TIKTOK_WIDTH - WIDGET_SIZE) } else { cx };
    let new_y = if tiktok_above { cy - TIKTOK_HEIGHT } else { cy };

    main_window
        .set_size(tauri::LogicalSize::new(TIKTOK_WIDTH, WIDGET_SIZE + TIKTOK_HEIGHT))
        .map_err(|e| e.to_string())?;
    main_window
        .set_position(tauri::LogicalPosition::new(new_x, new_y))
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn collapse_window(app: &AppHandle, widget_position: &str) -> Result<(), String> {
    let main_window = app.get_window("main").ok_or("Main window not found")?;
    let tiktok_above = widget_position.contains("bottom");
    let widget_right = widget_position.contains("right");

    let pos = main_window.outer_position().map_err(|e| e.to_string())?;
    let scale = main_window.scale_factor().map_err(|e| e.to_string())?;
    let cx = pos.x as f64 / scale;
    let cy = pos.y as f64 / scale;

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

fn webview_y(widget_position: &str) -> f64 {
    if widget_position.contains("bottom") {
        TITLEBAR_HEIGHT
    } else {
        WIDGET_SIZE + TITLEBAR_HEIGHT
    }
}

#[tauri::command]
pub async fn open_tiktok(app: AppHandle, widget_position: String, tiktok_url: String) -> Result<(), String> {
    let main_window = app.get_window("main").ok_or("Main window not found")?;

    // Webview already exists — expand window and restore webview size
    if let Some(webview) = app.get_webview("tiktok") {
        expand_window(&app, &widget_position)?;
        let wy = webview_y(&widget_position);
        webview
            .set_position(tauri::LogicalPosition::new(0.0, wy))
            .map_err(|e| e.to_string())?;
        webview
            .set_size(tauri::LogicalSize::new(TIKTOK_WIDTH, CONTENT_HEIGHT))
            .map_err(|e| e.to_string())?;
        main_window.set_resizable(true).map_err(|e| e.to_string())?;
        main_window.set_min_size(Some(tauri::LogicalSize::new(WIDGET_SIZE, WIDGET_SIZE + TITLEBAR_HEIGHT + 50.0))).map_err(|e| e.to_string())?;
        return Ok(());
    }

    let wy = webview_y(&widget_position);

    expand_window(&app, &widget_position)?;

    // Create webview only on first open
    let raw_url = if tiktok_url.is_empty() {
        "https://www.tiktok.com/foryou".to_string()
    } else {
        tiktok_url
    };
    let url = raw_url.parse().map_err(|e| format!("Invalid URL: {}", e))?;

    main_window
        .add_child(
            tauri::webview::WebviewBuilder::new("tiktok", tauri::WebviewUrl::External(url))
                .devtools(true)
                .on_navigation(|_url| true),
            tauri::LogicalPosition::new(0.0, wy),
            tauri::LogicalSize::new(TIKTOK_WIDTH, CONTENT_HEIGHT),
        )
        .map_err(|e| e.to_string())?;

    main_window.set_resizable(true).map_err(|e| e.to_string())?;
    main_window.set_min_size(Some(tauri::LogicalSize::new(WIDGET_SIZE, WIDGET_SIZE + TITLEBAR_HEIGHT + 50.0))).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn close_tiktok(app: AppHandle, widget_position: String) -> Result<(), String> {
    // Hide webview by shrinking to zero, then collapse window
    if let Some(webview) = app.get_webview("tiktok") {
        webview
            .set_size(tauri::LogicalSize::new(0.0, 0.0))
            .map_err(|e| e.to_string())?;
    }
    let main_window = app.get_window("main").ok_or("Main window not found")?;
    main_window.set_min_size(None::<tauri::LogicalSize<f64>>).map_err(|e| e.to_string())?;
    collapse_window(&app, &widget_position)?;
    main_window.set_resizable(false).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn destroy_tiktok(app: AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview("tiktok") {
        webview.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn open_tiktok_menu(app: AppHandle, widget_position: String) -> Result<(), String> {
    // Toggle: close if already open
    if let Some(win) = app.get_window("tiktok-menu") {
        win.close().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let main_window = app.get_window("main").ok_or("Main window not found")?;
    let pos = main_window.outer_position().map_err(|e| e.to_string())?;
    let scale = main_window.scale_factor().map_err(|e| e.to_string())?;
    let size = main_window.outer_size().map_err(|e| e.to_string())?;

    let cx = pos.x as f64 / scale;
    let cy = pos.y as f64 / scale;
    let win_w = size.width as f64 / scale;
    let win_h = size.height as f64 / scale;

    // Position right below the titlebar
    let titlebar_top = if widget_position.contains("bottom") { cy } else { cy + WIDGET_SIZE };

    let menu_w = 220.0;
    let menu_h = 44.0;
    let menu_x = cx;
    let menu_y = titlebar_top + TITLEBAR_HEIGHT;

    tauri::WebviewWindowBuilder::new(
        &app,
        "tiktok-menu",
        tauri::WebviewUrl::App("index.html?view=tiktok-menu".into()),
    )
    .title("Menu")
    .inner_size(menu_w, menu_h)
    .position(menu_x, menu_y)
    .resizable(false)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .transparent(true)
    .shadow(false)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn reset_tiktok_size(app: AppHandle, _widget_position: String) -> Result<(), String> {
    let main_window = app.get_window("main").ok_or("Main window not found")?;
    let default_w = TIKTOK_WIDTH;
    let default_h = WIDGET_SIZE + TIKTOK_HEIGHT;

    main_window
        .set_size(tauri::LogicalSize::new(default_w, default_h))
        .map_err(|e| e.to_string())?;

    // Webview sync happens automatically via the resize listener
    Ok(())
}

#[tauri::command]
pub async fn tiktok_eval(app: AppHandle, js: String) -> Result<(), String> {
    let webview = app.get_webview("tiktok").ok_or("TikTok webview not found")?;
    webview.eval(&js).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn sync_tiktok_webview(app: AppHandle, widget_position: String) -> Result<(), String> {
    let webview = match app.get_webview("tiktok") {
        Some(wv) => wv,
        None => return Ok(()),
    };

    let main_window = app.get_window("main").ok_or("Main window not found")?;
    let win_size = main_window.inner_size().map_err(|e| e.to_string())?;
    let scale = main_window.scale_factor().map_err(|e| e.to_string())?;
    let win_w = win_size.width as f64 / scale;
    let win_h = win_size.height as f64 / scale;

    let content_h = (win_h - WIDGET_SIZE - TITLEBAR_HEIGHT).max(0.0);
    let wy = webview_y(&widget_position);

    webview
        .set_position(tauri::LogicalPosition::new(0.0, wy))
        .map_err(|e| e.to_string())?;
    webview
        .set_size(tauri::LogicalSize::new(win_w, content_h))
        .map_err(|e| e.to_string())?;

    Ok(())
}

