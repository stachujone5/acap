use crate::config::Config;
use rdev::{listen, Event, EventType};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs::{self};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Type)]
pub struct AcapFile {
    name: String,
    path: PathBuf,
}

pub fn run_listener<F>(emit: F)
where
    F: Fn(&str, &str) + 'static,
{
    if let Err(error) = listen(move |event| callback(event, &emit)) {
        println!("Error: {:?}", error)
    }
}

fn callback<F: Fn(&str, &str)>(event: Event, emit: &F) {
    match event.event_type {
        EventType::KeyPress(key) => match key {
            rdev::Key::F1 => emit("KeyPress", "F1"),
            rdev::Key::F2 => emit("KeyPress", "F2"),
            rdev::Key::F3 => emit("KeyPress", "F3"),
            rdev::Key::F4 => emit("KeyPress", "F4"),
            rdev::Key::F5 => emit("KeyPress", "F5"),
            rdev::Key::F6 => emit("KeyPress", "F6"),
            rdev::Key::F7 => emit("KeyPress", "F7"),
            rdev::Key::F8 => emit("KeyPress", "F8"),
            rdev::Key::F9 => emit("KeyPress", "F9"),
            rdev::Key::F10 => emit("KeyPress", "F10"),
            rdev::Key::F11 => emit("KeyPress", "F11"),
            rdev::Key::F12 => emit("KeyPress", "F12"),
            _ => (),
        },
        _ => (),
    }
}
// Returns all .wav files living in the project's directory or throws
#[tauri::command]
#[specta::specta]
pub fn get_acap_files(config: tauri::State<'_, Config>) -> Result<Vec<AcapFile>, ()> {
    let config = config.get_config();
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
