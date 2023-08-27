use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{fs, io::Write, path::PathBuf};
use toml;

#[derive(Serialize, Deserialize, Type, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    System,
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Type, Clone)]
pub enum FunctionKey {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

#[derive(Serialize, Deserialize, Type, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ConfigUpdatableKey {
    SavePath(PathBuf),
    RecordingDurationInSecs(u32),
    Theme(Theme),
    StartRecordingKey(FunctionKey),
}

//  Configuration file type
#[derive(Serialize, Deserialize, Type, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub config_file_path: PathBuf,
    pub save_path: PathBuf,
    pub recording_duration_in_secs: u32,
    pub theme: Theme,
    pub start_recording_key: FunctionKey,
}

impl Config {
    // Creates project's config and saves it in:
    // Linux:   /home/username/.config/acap
    // Windows: C:\Users\username\AppData\Roaming\acap
    // macOS:   /Users/username/Library/Application Support/acap
    pub fn new() -> Self {
        let base_dirs = BaseDirs::new().unwrap();

        let project_dir = base_dirs.config_dir().to_path_buf().join("acap");

        fs::create_dir_all(&project_dir).unwrap();

        let config_file_path = project_dir.clone().join("config.toml");

        if config_file_path.exists() {
            let toml = fs::read_to_string(&config_file_path).unwrap_or_else(|_| {
                // If config got deleted while program was running - create one and return the default
                let config = Self::new();

                fs::read_to_string(config.config_file_path).unwrap()
            });

            let config: Config = toml::from_str(&toml).unwrap_or_else(|_| {
                // If someone modifies config manually using wrong values - create new one and return the default
                Self::new()
            });

            return config;
        }

        let config = Config {
            save_path: project_dir.clone(),
            recording_duration_in_secs: 30,
            config_file_path,
            theme: Theme::System,
            start_recording_key: FunctionKey::F12,
        };

        let toml = toml::to_string(&config).unwrap();
        let mut config_file = fs::File::create(&config.config_file_path).unwrap();

        config_file.write_all(toml.as_bytes()).unwrap();

        config
    }

    // Returns the project's config
    pub fn get_config(&self) -> Self {
        let toml = fs::read_to_string(&self.config_file_path).unwrap_or_else(|_| {
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

    // Updates the project's config
    pub fn update_config(&self, callback: impl Fn(&mut Self)) -> Config {
        let mut config = self.get_config();
        callback(&mut config);

        let toml = toml::to_string(&config).unwrap();

        fs::write(&self.config_file_path, toml).unwrap();

        config
    }
}
