use crate::config::Config;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[tauri::command]
#[specta::specta]
pub fn record_audio() -> Result<(), String> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find default output device");

    let config = device
        .default_output_config()
        .expect("Failed to get default output config");

    println!("Default output config: {:?}", config);

    let config_file = Config::get_config();

    let mut path = config_file.save_path;

    let current_time = SystemTime::now();

    let unix_timestamp = current_time
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // name the file
    let file_name = format!("{}.wav", unix_timestamp);

    path.push(file_name);

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

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(
        config_file.recording_duration_in_secs.into(),
    ));

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

// Returns all .wav files living in the project's directory or throws
#[tauri::command]
#[specta::specta]
pub fn get_acap_files() -> Result<Vec<AcapFile>, ()> {
    let config: Config = Config::get_config();

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
