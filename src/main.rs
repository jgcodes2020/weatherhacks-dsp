use std::{
    fs::File, io::{self, stdin, Write}, mem, path::Path, sync::OnceLock, thread, time::Duration
};

use analyzer::{Analyzer, FFT_CHUNK_SIZE};
use basic_dsp::{
    window_functions::BlackmanHarrisWindow, Buffer, ComplexToRealTransformsOps, DspVec, ElementaryOps, NoBuffer, RealOps, SingleBuffer, TimeToFrequencyDomainOperations, ToRealVector, Vector
};
use clap::Parser;
use cli::WeatherHacksCli;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

mod cli;
mod file;
mod utils;
mod analyzer;

const CENTER_BIN: usize = 210;
const STD_DEV: usize = 50;

static CLI: OnceLock<WeatherHacksCli> = OnceLock::new();
fn get_cli() -> &'static WeatherHacksCli {
    CLI.get().unwrap()
}

fn main() {
    let cli = CLI.get_or_init(|| {
        WeatherHacksCli::parse()
    });
    record_live();
}

fn record_live() {
    let cli = CLI.get().unwrap();

    // audio recording config
    let host = cpal::default_host();
    let dev = host.default_input_device().unwrap();
    let config = dev.default_input_config().expect("no default config");

    // FFT chunking stuff
    let mut chunk_box: Vec<f32> = Vec::new();
    let mut proc = Analyzer::new();
    chunk_box.reserve(FFT_CHUNK_SIZE);

    let stream = dev
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _| {
                let mut data_ptr: usize = 0;
                // this loop bunches data into chunks that are a nice size for FFT
                while data_ptr < data.len() {
                    let data_rem = data.len() - data_ptr;
                    let chunk_rem = FFT_CHUNK_SIZE - chunk_box.len();

                    chunk_box.extend_from_slice(
                        &data[data_ptr..(data_ptr + usize::min(data_rem, chunk_rem))],
                    );
                    data_ptr += chunk_rem;

                    // feed the FFT
                    if chunk_box.len() == FFT_CHUNK_SIZE {
                        proc.analyze_chunk(mem::replace(&mut chunk_box, {
                            let mut new_chunk = Vec::new();
                            new_chunk.reserve(FFT_CHUNK_SIZE);
                            new_chunk
                        }))
                    }
                }
                if cli.interactive {
                    // print usually flushes on newline, no newline so need manual flush
                    print!("\x1B[2Kscore: {}\x1B[G", proc.rain_score());
                    io::stdout().flush().unwrap();
                }
                else {
                    // println flushes
                    println!("{}", proc.rain_score());
                }
            },
            move |err| panic!("Audio error: {err}"),
            None,
        )
        .unwrap();

    stream.play().expect("BIG AUDIO ERROR!!!! AAAAAAA!");

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
}

#[allow(unused)]
fn process_file(input_file: &Path, output_file: &Path) {
    let mut out_file = File::create(output_file).unwrap();

    let samples = file::read_file(input_file).unwrap();

    let mut plot = utils::DspPlotter::new();

    for chunk_idx in (0..samples.len()).step_by(FFT_CHUNK_SIZE / 2) {
        if (samples.len() - chunk_idx) < FFT_CHUNK_SIZE {
            break;
        }

        let mut signal =
            Vec::from(&samples[chunk_idx..(chunk_idx + FFT_CHUNK_SIZE)]).to_real_time_vec();
        signal.set_delta(1.0f32 / 22050.0f32);

        let mut temp_buffer = SingleBuffer::new();

        let fft: DspVec<_, _, _, _> = signal
            .windowed_fft(&mut temp_buffer, &BlackmanHarrisWindow {})
            .magnitude();

        plot.plot(&fft[0..(fft.len() / 2)].to_real_freq_vec());
        thread::sleep(Duration::from_millis(100));
    }
}

