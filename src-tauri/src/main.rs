// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;

use config::{Config, ConfigUpdatableKey};
use specta::collect_types;
use std::thread;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_specta::ts;

#[tauri::command]
#[specta::specta]
fn get_config(config: tauri::State<'_, Config>) -> Config {
    config.get_config()
}

#[tauri::command]
#[specta::specta]
fn update_config(key: ConfigUpdatableKey, config: tauri::State<'_, Config>) -> Config {
    config.update_config(|config| match &key {
        ConfigUpdatableKey::SavePath(val) => config.save_path = val.to_owned(),
        ConfigUpdatableKey::RecordingDurationInSecs(val) => {
            config.recording_duration_in_secs = val.to_owned()
        }
        ConfigUpdatableKey::Theme(val) => config.theme = val.to_owned(),
        ConfigUpdatableKey::StartRecordingKey(val) => config.start_recording_key = val.to_owned(),
    })
}

#[tauri::command(async)]
#[specta::specta]
fn record_main_audio(config: tauri::State<'_, Config>) {
    audio::record_audio(config).unwrap();
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    mode: String,
    message: String,
}

fn main() {
    // Tray menu
    let quit = CustomMenuItem::new("quit", "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    // Export types from rust functions into typescript file
    ts::export(
        collect_types![
            get_config,
            update_config,
            audio::get_acap_files,
            record_main_audio
        ],
        "../src/utils/bindings.ts",
    )
    .expect("Failed to export ts bindings");

    let config = Config::new();

    tauri::Builder::default()
        .manage(config)
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
            get_config,
            update_config,
            audio::get_acap_files,
            record_main_audio
        ])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(move |app| {
            let wv = app.get_window("main").unwrap();

            thread::spawn(move || {
                audio::run_listener(move |s: &str, s1: &str| {
                    if let Err(err) = wv.emit(
                        "keypress",
                        Payload {
                            mode: String::from(s),
                            message: String::from(s1),
                        },
                    ) {
                        eprintln!("Error while emitting event: {:?}", err);
                    }
                })
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
