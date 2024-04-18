use clap::Parser;

#[derive(Parser)]
pub struct WeatherHacksCli {
    /// Adjusts output for interactive use. Requires an install of gnuplot.
    #[arg(short = 'i', long)]
    pub interactive: bool
}