use std::{fmt::Display, io::{self, Write}, vec};

use basic_dsp::{numbers::RealNumber, Domain, DspVec, MetaData, NumberSpace, ToSlice, Vector};
use gnuplot::{Figure, PlotOption::Caption};

pub fn write_csv_row(output: &mut impl Write, mut source: impl Iterator<Item = impl Display>) -> Result<(), io::Error>{
    if let Some(first) = source.next() {
        write!(output, "{}", first)?;
        while let Some(next) = source.next() {
            write!(output, ",{}", next)?;
        }
        writeln!(output)?;
    }
    Ok(())
}

pub struct DspPlotter {
    fg: Figure
}

impl DspPlotter {
    pub fn new() -> Self {
        Self {
            fg: Figure::new()
        }
    }

    pub fn plot<N: NumberSpace, D: Domain>(&mut self, vector: &DspVec<Vec<f32>, f32, N, D>) {
        let dx = vector.delta();

        let xvals = (0..vector.len()).map(|x| (x as f32) * vector.delta());
        let yvals = vector.data.clone();

        self.fg.clear_axes();
        self.fg.axes2d().lines(
            xvals,
            yvals,
            &[Caption("test")]
        );

        self.fg.show_and_keep_running().unwrap();
    }
}

pub fn plot_dsp<N: NumberSpace, D: Domain>(vector: &DspVec<Vec<f32>, f32, N, D>) {
    let dx = vector.delta();

    let mut fg = Figure::new();

    let xvals = (0..vector.len()).map(|x| (x as f32) * vector.delta());
    let yvals = vector.data.clone();

    fg.clear_axes();
    let plot = fg.axes2d().lines(
        xvals,
        yvals,
        &[Caption("test")]
    );

    fg.show_and_keep_running().unwrap();
}