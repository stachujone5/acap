// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;

use std::panic;

use config::{Config, ConfigUpdatableKey};
use specta::collect_types;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_specta::ts;

#[tauri::command]
#[specta::specta]
fn get_config() -> Config {
    Config::get_config()
}

#[tauri::command]
#[specta::specta]
fn update_config_key(key: ConfigUpdatableKey) -> Config {
    Config::update_key(key)
}

fn main() {
    // Tray menu
    let quit = CustomMenuItem::new("quit", "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    // Export types from rust functions into typescript file
    ts::export(
        collect_types![
            audio::record_audio,
            get_config,
            update_config_key,
            audio::get_acap_files,
        ],
        "../src/utils/bindings.ts",
    )
    .expect("Failed to export ts bindings");

    // Create a new config if there is no config.toml in the project's directory
    let config = panic::catch_unwind(|| Config::get_config());

    if let Err(_) = config {
        Config::new();
    }

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => app.get_window("main").unwrap().show().unwrap(),
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => app.exit(0),
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            audio::record_audio,
            get_config,
            update_config_key,
            audio::get_acap_files
        ])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
