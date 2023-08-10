// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod fs;

use specta::collect_types;
use tauri_specta::ts;

fn main() {
    ts::export(
        collect_types![audio::record_audio, fs::get_save_dir, fs::get_recordings,],
        "../src/utils/bindings.ts",
    )
    .expect("Failed to export ts bindings");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            audio::record_audio,
            fs::get_save_dir,
            fs::get_recordings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
