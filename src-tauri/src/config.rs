use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{fs, io::Write, path::PathBuf};
use toml;

#[derive(Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    System,
    Light,
    Dark,
}

//  Configuration file type
#[derive(Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub config_file_path: PathBuf,
    pub save_path: PathBuf,
    pub recording_duration_in_secs: u32,
    pub theme: Theme,
}

// Keys that can be updated from the frontend
#[derive(Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum ConfigUpdatableKey {
    SavePath(PathBuf),
    RecordingDurationInSecs(u32),
    Theme(Theme),
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
            config_file_path: project_dir.clone().join("config.toml"),
            theme: Theme::System,
        }
    }

    // Attempts to create a default config.toml file in project's directory
    pub fn new() -> Config {
        let config = Self::default();

        let toml = toml::to_string(&config).unwrap();
        let mut config_file = fs::File::create(&config.config_file_path).unwrap();

        config_file.write_all(toml.as_bytes()).unwrap();

        config
    }

    // Returns the project's config
    pub fn get_config() -> Self {
        let toml = fs::read_to_string(Self::default().config_file_path).unwrap_or_else(|_| {
            // If config got deleted while program was running - create one and return the default
            let config = Self::new();

            fs::read_to_string(config.config_file_path).unwrap()
        });

        let config: Config = toml::from_str(&toml).unwrap_or_else(|_| {
            // If someone modifies config manually using wrong values - create new one and return the default
            Self::new()
        });

        config
    }

    // Saves the project's config
    pub fn save_config(config: Config) -> Config {
        let toml = toml::to_string(&config).unwrap();

        fs::write(Self::default().config_file_path, toml).unwrap();

        config
    }

    // Updates a key in config and saves the new config.
    pub fn update_key(key: ConfigUpdatableKey) -> Config {
        let mut config = Self::get_config();

        match key {
            ConfigUpdatableKey::SavePath(value) => config.save_path = value,
            ConfigUpdatableKey::RecordingDurationInSecs(value) => {
                config.recording_duration_in_secs = value
            }
            ConfigUpdatableKey::Theme(value) => config.theme = value,
        }

        Self::save_config(config)
    }
}
