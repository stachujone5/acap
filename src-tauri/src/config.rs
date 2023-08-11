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
    pub project_dir: PathBuf,
}

impl Config {
    // Attempts find a default project's directory and returns a default config
    pub fn default() -> Self {
        // Creates project's directory or throws
        // Linux:   /home/username/.config/acap
        // Windows: C:\Users\username\AppData\Roaming\acap
        // macOS:   /Users/username/Library/Application Support/acap
        let base_dirs = BaseDirs::new().unwrap();

        let project_dir = base_dirs.config_dir().to_path_buf().join("acap");

        fs::create_dir_all(&project_dir).unwrap();

        Config {
            save_path: project_dir.clone(),
            recording_duration_in_secs: 30,
            project_dir: project_dir.clone(),
        }
    }

    // Attempts to create a default config.toml file in project's directory
    pub fn new() {
        let config = Self::default();
        let toml = toml::to_string(&config).unwrap();

        let mut config_file = fs::File::create(config.project_dir.join("config.toml")).unwrap();

        config_file.write_all(toml.as_bytes()).unwrap()
    }

    // Returns the project's config
    pub fn get_config() -> Self {
        let project_dir = Self::default().project_dir;
        let config_file_path = project_dir.join("config.toml");

        let toml = fs::read_to_string(config_file_path).unwrap();

        let config: Config = toml::from_str(&toml).unwrap();

        config
    }

    // Sets the project's config
    pub fn set_config(config: Config) -> Result<Config, ()> {
        let dir = Config::default().project_dir;
        let config_path = dir.join("config.toml");

        let toml = toml::to_string(&config).unwrap();

        fs::write(config_path, toml).unwrap();

        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Type)]
pub struct AcapFile {
    name: String,
    path: PathBuf,
}

// Returns all .wav files living in the project's directory or throws
#[tauri::command]
#[specta::specta]
pub fn get_acap_files() -> Result<Vec<AcapFile>, ()> {
    let config = Config::get_config();

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
