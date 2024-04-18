use std::{fmt::Display, io::{self, Write}, vec};

use basic_dsp::{numbers::RealNumber, Domain, DspVec, MetaData, NumberSpace, ToSlice, Vector};
use gnuplot::{AlignType, AutoOption, AxesCommon, Figure, LabelOption, PlotOption::Caption};

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

    pub fn plot<S: ToSlice<f32>, N: NumberSpace, D: Domain>(&mut self, vector: &DspVec<S, f32, N, D>) {

        let xvals = (0..vector.len()).map(|x| (x as f32) * vector.delta());
        let yvals = vector.data.to_slice().to_vec();

        self.fg.clear_axes();
        
        let axes = self.fg.axes2d().lines(
            xvals,
            yvals,
            &[Caption("test")]
        );

        axes.set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(vector.delta() as f64 * (vector.len() as f64)));
        axes.set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(50.0));
        axes.set_x_label("Bin number", &[LabelOption::TextAlign(AlignType::AlignCenter)]);

        self.fg.show_and_keep_running().unwrap();
    }
}