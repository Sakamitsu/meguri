mod commands;
mod data;
mod images;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_data = data::load_or_create();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(AppState::new(app_data))
        .invoke_handler(tauri::generate_handler![
            commands::load_data,
            commands::save_settings,
            commands::get_random_image,
            commands::add_action,
            commands::update_action,
            commands::delete_action,
            commands::set_active_action,
            commands::log_session,
            commands::get_stats,
            commands::open_tiktok,
            commands::close_tiktok,
            commands::destroy_tiktok,
            commands::sync_tiktok_webview,
            commands::open_tiktok_menu,
            commands::reset_tiktok_size,
            commands::tiktok_eval,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
