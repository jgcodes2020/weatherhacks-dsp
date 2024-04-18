use clap::Parser;

#[derive(Parser)]
pub struct WeatherHacksCli {
    /// Adjusts output for interactive use. Requires an install of gnuplot.
    #[arg(short = 'i', long)]
    pub interactive: bool,

    /// Time (in seconds) between successive updates. Defaults to 0.5s.
    #[arg(short = 'f', long, value_name = "INTERVAL", default_value_t = 0.5f64)]
    pub poll_interval: f64
}

impl Default for WeatherHacksCli {
    fn default() -> Self {
        Self { interactive: false, poll_interval: 0.5f64 }
    }
}