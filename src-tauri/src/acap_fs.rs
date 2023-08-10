use dirs::home_dir;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{fs, path::PathBuf};

use tauri::api::path::document_dir;

const DEFAULT_DIR_NAME: &str = "acap";

// Return absolute path to acap dir, if not present create acap folder in /home/documents/acap if there is no /documents create one, if there is no home - throw
#[tauri::command]
#[specta::specta]
pub fn get_acap_dir() -> Result<PathBuf, ()> {
    match document_dir() {
        Some(dir) => {
            let full_path = dir.join(DEFAULT_DIR_NAME);

            fs::create_dir_all(&full_path).map_err(|_| ())?;

            Ok(full_path)
        }
        None => match home_dir() {
            Some(dir) => {
                let full_path = dir.join(DEFAULT_DIR_NAME);

                fs::create_dir_all(&full_path).map_err(|_| ())?;

                Ok(full_path)
            }

            None => Err(()),
        },
    }
}

#[derive(Serialize, Deserialize, Type)]
pub struct AcapFile {
    name: String,
    path: PathBuf,
}

// Returns all .wav files living in /home/documents/acap or throws
#[tauri::command]
#[specta::specta]
pub fn get_acap_files() -> Result<Vec<AcapFile>, ()> {
    let acap_dir = get_acap_dir()?;

    let all_files = fs::read_dir(acap_dir).map_err(|_| ())?;

    let acap_files = all_files
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let name = entry.file_name().to_str()?.to_string();

            if path.is_file() && path.extension().map(|ext| ext == "wav").unwrap_or(false) {
                Some(AcapFile { name, path })
            } else {
                None
            }
        })
        .collect();

    Ok(acap_files)
}
