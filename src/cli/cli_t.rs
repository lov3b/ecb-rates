use clap::{Parser, ValueEnum};
use smol_str::SmolStr;

use super::{ShowDays, SortBy};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Which currencies do you want to fetch rates for?
    #[arg(long = "currencies", short = 'c')]
    pub currencies: Vec<SmolStr>,

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
    #[arg(value_enum, long = "sort-by", default_value_t = SortBy::Currency)]
    pub sort_by: SortBy,

    /// Recalculate to the perspective from an included currency
    #[arg(long = "perspective", short = 'p')]
    pub perspective: Option<SmolStr>,

    /// Invert the rate
    #[arg(long = "invert", short = 'i')]
    pub should_invert: bool,

    /// Max decimals to keep in price.
    #[arg(long = "max-decimals", short = 'd', default_value_t = 5)]
    pub max_decimals: u8,

    /// Amount of data
    #[arg(default_value_t = ShowDays::Days(1), long="show-days", short='s')]
    pub show_days: ShowDays,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum FormatOption {
    /// JSON output
    Json,
    /// Plain line-by-line output (with extra flags)
    Plain,
}
