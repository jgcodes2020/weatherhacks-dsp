use std::{error::Error, fs::File, path::Path};

use hound::WavReader;

pub fn read_file(path: &Path) -> Result<Vec<f32>, Box<dyn Error>> {
    let reader = WavReader::new(File::open(path)?)?;

    let spec = reader.spec();
    match (spec.sample_format, spec.bits_per_sample) {
        (hound::SampleFormat::Float, 32) => {
            let samples = reader.into_samples::<f32>();
            if spec.channels == 1 {
                Ok(samples.map(|x| x.unwrap()).collect())
            } else {
                Ok(samples
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if i % (spec.channels as usize) == 0 {
                            Some(x.unwrap())
                        } else {
                            None
                        }
                    })
                    .collect())
            }
        }
        (hound::SampleFormat::Int, 16) => {
            let samples = reader.into_samples::<i16>();
            if spec.channels == 1 {
                Ok(samples.map(|x| x.unwrap() as f32).collect())
            } else {
                Ok(samples
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if i % (spec.channels as usize) == 0 {
                            Some(x.unwrap() as f32)
                        } else {
                            None
                        }
                    })
                    .collect())
            }
        }
        _ => panic!(),
    }
}
