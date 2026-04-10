mod commands;
mod data;
mod images;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_data = data::load_or_create();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
