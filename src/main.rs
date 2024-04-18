use std::{
    fs::File,
    io::{self, stdin, Write},
    mem,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use basic_dsp::{
    window_functions::BlackmanHarrisWindow, Buffer, ComplexToRealTransformsOps, DspVec, NoBuffer,
    RealOps, SingleBuffer, TimeToFrequencyDomainOperations, ToRealVector, Vector,
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use utils::{plot_dsp, DspPlotter};

mod file;
mod utils;

const CHUNK_SIZE: usize = 2048;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    // process_file(&PathBuf::from(&args[1]), &PathBuf::from(&args[2]));
    audio_input();
}

fn process_file(input_file: &Path, output_file: &Path) {
    let mut out_file = File::create(output_file).unwrap();

    let samples = file::read_file(input_file).unwrap();

    let mut plot = utils::DspPlotter::new();

    for chunk_idx in (0..samples.len()).step_by(CHUNK_SIZE / 2) {
        if (samples.len() - chunk_idx) < CHUNK_SIZE {
            break;
        }

        let mut signal =
            Vec::from(&samples[chunk_idx..(chunk_idx + CHUNK_SIZE)]).to_real_time_vec();
        signal.set_delta(1.0f32 / 22050.0f32);

        let mut temp_buffer = SingleBuffer::new();

        let fft: DspVec<_, _, _, _> = signal
            .windowed_fft(&mut temp_buffer, &BlackmanHarrisWindow {})
            .magnitude();
        plot.plot(&fft);
        thread::sleep(Duration::from_millis(100));
    }
}

fn analyze_fft(slice: Vec<f32>, plotter: &mut utils::DspPlotter) {
    let mut signal = slice.to_real_time_vec();
    signal.set_delta(1.0f32 / 44100f32);

    let mut temp_buffer = SingleBuffer::new();

    let fft: DspVec<_, _, _, _> = signal
        .windowed_fft(&mut temp_buffer, &BlackmanHarrisWindow {})
        .magnitude();
    plotter.plot(&fft);
}

fn audio_input() {
    let host = cpal::default_host();

    let dev = host.default_input_device().unwrap();

    let config = dev.default_input_config().expect("no default config");

    let mut chunk_box: Vec<f32> = Vec::new();
    let mut plotter = DspPlotter::new();

    chunk_box.reserve(CHUNK_SIZE);

    let stream = dev
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _| {
                let mut data_ptr: usize = 0;
                while data_ptr < data.len() {
                    let data_rem = data.len() - data_ptr;
                    let chunk_rem = CHUNK_SIZE - chunk_box.len();

                    chunk_box.extend_from_slice(
                        &data[data_ptr..(data_ptr + usize::min(data_rem, chunk_rem))],
                    );
                    data_ptr += chunk_rem;

                    if chunk_box.len() == CHUNK_SIZE {
                        analyze_fft(
                            mem::replace(&mut chunk_box, {
                                let mut new_chunk = Vec::new();
                                new_chunk.reserve(CHUNK_SIZE);
                                new_chunk
                            }),
                            &mut plotter,
                        );
                    }
                }
            },
            move |err| eprintln!("Audio error: {err}"),
            None,
        )
        .unwrap();

    stream.play().expect("OH NAURRRRR!");

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
}
