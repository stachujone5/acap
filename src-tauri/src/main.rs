// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;

use std::path::PathBuf;

use config::Config;
use specta::collect_types;
use tauri_specta::ts;

#[tauri::command]
#[specta::specta]
fn get_config() -> Config {
    Config::get_config()
}

#[tauri::command]
#[specta::specta]
fn set_config_save_path(save_path: PathBuf) -> Result<Config, ()> {
    let config = Config::get_config();
    Config::set_config(Config {
        save_path,
        ..config
    })
}

fn main() {
    // Export types from rust functions into typescript file
    ts::export(
        collect_types![
            audio::record_audio,
            get_config,
            set_config_save_path,
            config::get_acap_files,
        ],
        "../src/utils/bindings.ts",
    )
    .expect("Failed to export ts bindings");

    // Attempt to create a config.toml in project's directory or terminate the process
    Config::new();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            audio::record_audio,
            get_config,
            set_config_save_path,
            config::get_acap_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
