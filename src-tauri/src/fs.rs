use serde::{Deserialize, Serialize};
use specta::Type;
use std::{fs, path::PathBuf};

use tauri::api::path::document_dir;

#[tauri::command]
#[specta::specta]
pub fn get_save_dir() -> Result<PathBuf, ()> {
    let default_directory = document_dir();

    match default_directory {
        Some(mut default_directory) => {
            default_directory.push("acap");

            Ok(default_directory)
        }
        None => Err(()),
    }
}

#[derive(Serialize, Deserialize, Type)]
pub struct File {
    name: String,
    path: PathBuf,
}

#[tauri::command]
#[specta::specta]
pub fn get_recordings() -> Result<Vec<File>, ()> {
    let save_dir = get_save_dir()?;

    let files_in_save_dir = fs::read_dir(save_dir).map_err(|_| ())?;

    let wav_files = files_in_save_dir
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let name = entry.file_name().to_str()?.to_string();

            if path.is_file() && path.extension().map(|ext| ext == "wav").unwrap_or(false) {
                Some(File { name, path })
            } else {
                None
            }
        })
        .collect();

    Ok(wav_files)
}
