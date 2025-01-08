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

    /// Force color in output. Normally it will disable color in pipes
    #[arg(long = "force-color")]
    pub force_color: bool,

    /// Sort by the currency name (in alphabetical order), or by the rate value (low -> high)
    #[arg(value_enum, long = "sort-by", short = 's', default_value_t = SortBy::Currency)]
    pub sort_by: SortBy,

    /// Amount of data
    #[arg(value_enum, default_value_t = Resolution::TODAY, long="resolution", short='r')]
    pub resolution: Resolution,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortBy {
    Currency,
    Rate,
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

impl SortBy {
    pub fn get_comparer(&self) -> fn(&(&str, f64), &(&str, f64)) -> std::cmp::Ordering {
        match self {
            Self::Currency => |a, b| a.0.cmp(&b.0),
            Self::Rate => |a, b| a.1.total_cmp(&b.1),
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
