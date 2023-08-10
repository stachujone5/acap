use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

#[tauri::command]
pub fn record_audio() -> Result<(), String> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find default output device");

    println!(
        "Output device: {}",
        device.name().expect("Failed to get the device name")
    );

    let config = device
        .default_output_config()
        .expect("Failed to get default output config");

    println!("Default output config: {:?}", config);

    let desktop_dir = dirs::desktop_dir().ok_or("Failed to get desktop directory")?;
    let mut path = desktop_dir.clone();
    path.push("sample.wav");

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
    std::thread::sleep(std::time::Duration::from_secs(3));
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
