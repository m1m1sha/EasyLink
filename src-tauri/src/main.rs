// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod commands;
mod constants;
mod utils;

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init());

    #[cfg(not(windows))]
    {
        use tauri_plugin_autostart::MacosLauncher;
        app.plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![constants::AUTOSTART]),
        ));
    }

    app.invoke_handler(tauri::generate_handler![
        commands::autostart,
        commands::autostart_status,
    ])
    .setup(|app| {
        let window = app.get_webview_window("main").unwrap();
        let _ = window.set_title(&format!("EasyLink {}", constants::VERSION));
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
