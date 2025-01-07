use clap::{arg, Parser, ValueEnum};

use crate::ecb_url;

#[derive(Debug, Parser)]
#[command(author, version, about)]

pub struct Cli {
    /// Which currencies do you want to fetch rates for?
    #[arg(long = "currencies", short = 'c')]
    pub currencies: Vec<String>,

    #[arg(value_enum, default_value_t = FormatOption::Plain)]
    pub command: FormatOption,

    /// Show the time in the output
    #[arg(long = "display-time", default_value_t = true)]
    pub display_time: bool,

    /// Print currencies in a compact single line
    #[arg(long = "compact")]
    pub compact: bool,

    /// Override the cache
    #[arg(long = "no-cache")]
    pub no_cache: bool,

    /// Amount of data
    #[arg(value_enum, default_value_t = Resolution::TODAY, long="resolution", short='r')]
    pub resolution: Resolution,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Resolution {
    TODAY,
    HistDays90,
    HistDay,
}

impl Resolution {
    pub fn to_ecb_url(&self) -> &'static str {
        match self {
            Resolution::TODAY => ecb_url::TODAY,
            Resolution::HistDays90 => ecb_url::hist::DAYS_90,
            Resolution::HistDay => ecb_url::hist::DAILY,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FormatOption {
    /// JSON output
    Json,
    /// Plain line-by-line output (with extra flags)
    Plain,
}
