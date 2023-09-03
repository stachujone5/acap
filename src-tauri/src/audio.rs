use crate::config::Config;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use rdev::{listen, Event, EventType};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const RECORDING_DURATION_IN_SECS: u64 = 600;
const MAIN_RECORDING_FILE_NAME: &str = "main_recording.wav";

pub fn record_audio(config_file: tauri::State<'_, Config>) -> Result<(), String> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find default output device");

    let config = device
        .default_output_config()
        .expect("Failed to get default output config");

    println!("Default output config: {:?}", config);

    let mut path = config_file.save_path.clone();

    path.push(MAIN_RECORDING_FILE_NAME);

    let spec = wav_spec_from_config(&config);
    let writer = hound::WavWriter::create(path, spec).map_err(|_| "Failed to create WAV writer")?;

    let writer = Arc::new(Mutex::new(Some(writer)));

    println!("Begin recording...");

    let writer_2 = writer.clone();

    let err_fn = |_| {
        eprintln!("An error occurred on stream: Something went wrong");
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
            err_fn,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
            err_fn,
            None,
        ),
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
            err_fn,
            None,
        ),
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
            err_fn,
            None,
        ),
        _ => return Err("Unsupported sample format".to_string()),
    }
    .map_err(|_| "Failed to build input stream")?;

    stream.play().map_err(|_| "Failed to play audio stream")?;

    std::thread::sleep(std::time::Duration::from_secs(RECORDING_DURATION_IN_SECS));

    drop(stream);
    writer
        .lock()
        .unwrap()
        .take()
        .unwrap()
        .finalize()
        .map_err(|_| "Failed to finalize writer")?;

    println!("Recording complete!");
    Ok(())
}

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    if format.is_float() {
        hound::SampleFormat::Float
    } else {
        hound::SampleFormat::Int
    }
}

fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
    hound::WavSpec {
        channels: config.channels() as _,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: (config.sample_format().sample_size() * 8) as _,
        sample_format: sample_format(config.sample_format()),
    }
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

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

            if path.is_file()
                && path.file_name().unwrap() != MAIN_RECORDING_FILE_NAME
                && path.extension().map(|ext| ext == "wav").unwrap_or(false)
            {
                Some(AcapFile { name, path })
            } else {
                None
            }
        })
        .collect();

    Ok(acap_files)
}
