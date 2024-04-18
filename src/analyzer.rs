use basic_dsp::{
    window_functions::BlackmanHarrisWindow, ComplexToRealTransformsOps, DspVec, SingleBuffer,
    TimeToFrequencyDomainOperations, ToRealVector, Vector,
};
use circular_buffer::CircularBuffer;
use gnuplot::Figure;

use crate::{utils::DspPlotter, CLI};


pub const FFT_CHUNK_SIZE: usize = 8192;
pub const NUM_AVG: usize = 20;

pub struct Analyzer {
    rec_low: CircularBuffer<NUM_AVG, f32>,
    rec_high: CircularBuffer<NUM_AVG, f32>,
    plot: Option<DspPlotter>
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            rec_low: CircularBuffer::new(),
            rec_high: CircularBuffer::new(),
            plot: if CLI.get().unwrap().interactive {
                Some(DspPlotter::new())
            }
            else {
                None
            }
        }
    }

    pub fn analyze_chunk(&mut self, slice: Vec<f32>) {
        let mut signal = slice.to_real_time_vec();
        signal.set_delta(1.0f32 / 44100f32);

        // eliminate DC bias in the time-domain, this should make the FFT less noisy on the left
        {
            let mean = signal.data.iter().sum::<f32>() / (signal.data.len() as f32);
            signal.data.iter_mut().for_each(|x| *x -= mean);
        }

        let mut temp_buffer = SingleBuffer::new();

        // take the FT to extract frequencies
        let fft: DspVec<_, _, _, _> = signal
            .windowed_fft(&mut temp_buffer, &BlackmanHarrisWindow {})
            .magnitude();

        if let Some(plot) = &mut self.plot { 
            plot.plot(&fft[0..1024].to_real_freq_vec());
        }

        {
            const LOW_BIN: usize = 210;
            const LOW_WINDOW: usize = 50;

            let fft_signal = &fft[(LOW_BIN - LOW_WINDOW)..(LOW_BIN + LOW_WINDOW)];
            let low_mean = (fft_signal.iter().sum::<f32>() as f32) / (fft_signal.len() as f32);

            self.rec_low.push_back(low_mean);
        }
        {
            const HIGH_BIN: usize = 750;
            const HIGH_WINDOW: usize = 100;

            let fft_signal = &fft[(HIGH_BIN - HIGH_WINDOW)..(HIGH_BIN + HIGH_WINDOW)];
            let high_mean = (fft_signal.iter().sum::<f32>() as f32) / (fft_signal.len() as f32);

            self.rec_high.push_back(high_mean);
        }
    }

    pub fn rain_score(&self) -> f32 {

        fn high_weight_fn(x: f32) -> f32 {
            const COEFFS: [f32; 3] = [-0.09168839f32, 1.138311f32, -2.533034f32];
            f32::max(COEFFS[0].mul_add(x, COEFFS[1]).mul_add(x, COEFFS[2]), 0f32)
        }

        let high_avg = self.rec_high.iter().sum::<f32>() / (NUM_AVG as f32);
        high_weight_fn(high_avg)
    }

    
}
