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

    /// Don't show time in output
    #[arg(long = "no-time")]
    pub no_time: bool,

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

    /// Recalculate to the perspective from an included currency
    #[arg(long = "perspective", short = 'p')]
    pub perspective: Option<String>,

    /// Invert the rate
    #[arg(long = "invert", short = 'i')]
    pub should_invert: bool,

    //// Max decimals to keep in price.
    #[arg(long = "max-decimals", short = 'd', default_value_t = 5)]
    pub max_decimals: u8,

    /// Amount of data
    #[arg(value_enum, default_value_t = View::TODAY, long="view", short='v')]
    pub view: View,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortBy {
    Currency,
    Rate,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum View {
    #[clap(name = "last-day")]
    TODAY,
    #[clap(name = "last-90-days")]
    HistDays90,
    #[clap(name = "all-days")]
    HistDaysAll,
}

impl View {
    pub fn to_ecb_url(&self) -> &'static str {
        match self {
            View::TODAY => ecb_url::TODAY,
            View::HistDays90 => ecb_url::hist::DAYS_90,
            View::HistDaysAll => ecb_url::hist::DAYS_ALL,
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            View::TODAY => "today",
            View::HistDays90 => "last-90-days",
            View::HistDaysAll => "all-days",
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
