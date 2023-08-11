use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{fs, io::Write, path::PathBuf};
use toml;

//  Configuration file type
#[derive(Serialize, Deserialize, Type)]
pub struct Config {
    pub save_path: PathBuf,
    pub recording_duration_in_secs: u32,
}

// Creates and returns project directory or throws
// Linux:   /home/username/.config/acap
// Windows: C:\Users\username\AppData\Roaming\acap
// macOS:   /Users/username/Library/Application Support/acap
#[tauri::command]
#[specta::specta]
pub fn get_project_dir() -> Result<PathBuf, ()> {
    if let Some(base_dirs) = BaseDirs::new() {
        let project_dir = base_dirs.config_dir().to_path_buf().join("acap");

        fs::create_dir_all(&project_dir).map_err(|_| ())?;

        Ok(project_dir.to_path_buf())
    } else {
        Err(())
    }
}

// Creates config.toml file with default project configuration in the project's directory or terminates
pub fn create_config_file() {
    let project_dir = get_project_dir().unwrap();

    let config: Config = Config {
        save_path: project_dir.clone(),
        recording_duration_in_secs: 30,
    };

    let toml = toml::to_string(&config).unwrap();

    let mut config_file = fs::File::create(project_dir.clone().join("config.toml")).unwrap();

    config_file.write_all(toml.as_bytes()).unwrap()
}

// Reads the config.toml and returns a Config struct or throws
#[tauri::command]
#[specta::specta]
pub fn get_config() -> Result<Config, ()> {
    let project_dir = get_project_dir()?;
    let config_file_path = project_dir.join("config.toml");

    let toml_content = fs::read_to_string(config_file_path).map_err(|_| ())?;

    let config: Config = toml::from_str(&toml_content).map_err(|_| ())?;

    Ok(config)
}

// pub fn set_config(config: &Config) -> Result<Config, ()> {

// }

#[derive(Serialize, Deserialize, Type)]
pub struct AcapFile {
    name: String,
    path: PathBuf,
}

// Returns all .wav files living in the project's directory or throws
#[tauri::command]
#[specta::specta]
pub fn get_acap_files() -> Result<Vec<AcapFile>, ()> {
    let config = get_config()?;

    let all_files = fs::read_dir(config.save_path).map_err(|_| ())?;

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
