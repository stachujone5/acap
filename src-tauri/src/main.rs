// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod config;

use specta::collect_types;
use tauri_specta::ts;

fn main() {
    // Export types from rust functions into typescript file
    ts::export(
        collect_types![
            audio::record_audio,
            config::get_config,
            config::get_acap_files,
        ],
        "../src/utils/bindings.ts",
    )
    .expect("Failed to export ts bindings");


    // Attempt to create a config.toml in project's directory or terminate the process
    config::create_config_file();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            audio::record_audio,
            config::get_config,
            config::get_acap_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
