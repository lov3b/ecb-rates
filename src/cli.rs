use clap::{arg, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]

pub struct Cli {
    /// Which currencies do you want to fetch rates for?
    #[arg(long = "currencies", short = 'c')]
    pub currencies: Vec<String>,
    /// Which subcommand (output format) we are using
    #[command(subcommand)]
    pub command: FormatCommand,
}

/// Subcommand enum for output format
#[derive(Debug, Subcommand)]
pub enum FormatCommand {
    /// Minimal JSON output
    JSONMin,
    /// Pretty-printed JSON output
    JSONPretty,
    /// Plain line-by-line output (with extra flags)
    Plain {
        /// Show the time in the output
        #[arg(long = "display-time")]
        display_time: bool,
        /// Print currencies in a compact single line
        #[arg(long = "compact")]
        compact: bool,
    },
}
