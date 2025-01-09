use clap::Parser as _;
use ecb_rates::cache::{Cache, CacheLine};
use reqwest::{Client, IntoUrl};
use std::process::ExitCode;

use ecb_rates::cli::{Cli, FormatOption};
use ecb_rates::models::ExchangeRateResult;
use ecb_rates::parsing::parse;
use ecb_rates::table::{TableRef, TableTrait as _};
use ecb_rates::utils_calc::{change_perspective, filter_currencies};

async fn get_and_parse(url: impl IntoUrl) -> anyhow::Result<Vec<ExchangeRateResult>> {
    let client = Client::new();
    let xml_content = client.get(url).send().await?.text().await?;
    parse(&xml_content)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    if cli.force_color {
        colored::control::set_override(true);
    }

    let use_cache = !cli.no_cache;
    let mut cache = if use_cache { Cache::load() } else { None };
    let cache_ok = cache.as_ref().map_or_else(
        || false,
        |c| {
            c.get_cache_line(cli.resolution)
                .map_or_else(|| false, |cl| cl.is_valid())
        },
    );
    let mut parsed = if cache_ok {
        // These are safe unwraps
        cache
            .as_ref()
            .unwrap()
            .get_cache_line(cli.resolution)
            .unwrap()
            .exchange_rate_results
            .clone()
    } else {
        let parsed = match get_and_parse(cli.resolution.to_ecb_url()).await {
            Ok(k) => k,
            Err(e) => {
                eprintln!("Failed to get/parse data from ECB: {}", e);
                return ExitCode::FAILURE;
            }
        };

        if !cache_ok {
            let not_equal_cache = cache.as_ref().map_or_else(
                || true,
                |cache_local| {
                    cache_local
                        .get_cache_line(cli.resolution)
                        .map_or_else(|| true, |cache_line| cache_line == &parsed)
                },
            );

            if not_equal_cache {
                if let Some(cache_safe) = cache.as_mut() {
                    let cache_line = CacheLine::new(parsed.clone());
                    cache_safe.set_cache_line(cli.resolution, cache_line);
                    if let Err(e) = cache_safe.save() {
                        eprintln!("Failed to save to cache with: {:?}", e);
                    }
                }
            }
        }
        parsed
    };

    if let Some(currency) = cli.perspective.map(|s| s.to_uppercase()) {
        let error_occured = change_perspective(&mut parsed, &currency).is_none();
        if error_occured {
            eprintln!("The currency wasn't in the data from the ECB!");
            return ExitCode::FAILURE;
        }
    }

    if !cli.currencies.is_empty() {
        let currencies = cli
            .currencies
            .iter()
            .map(|x| x.to_uppercase())
            .collect::<Vec<_>>();

        filter_currencies(&mut parsed, &currencies);
    }

    let output = match cli.command {
        FormatOption::Json => {
            let mut json_values = parsed
                .iter()
                .map(|x| serde_json::to_value(x).expect("Failed to parse content as JSON value"))
                .collect::<Vec<_>>();

            if cli.no_time {
                json_values
                    .iter_mut()
                    .filter_map(|json_value| json_value.as_object_mut())
                    .for_each(|map| {
                        map.remove_entry("time");
                    });
            }

            let to_string_json = if cli.compact {
                serde_json::to_string
            } else {
                serde_json::to_string_pretty
            };
            to_string_json(&json_values).expect("Failed to parse content as JSON")
        }
        FormatOption::Plain => parsed
            .iter()
            .map(|x| {
                let mut t: TableRef = x.into();
                if cli.no_time {
                    t.disable_header();
                }
                t.sort(&cli.sort_by);
                t.to_string()
            })
            .collect::<Vec<_>>()
            .join("\n"),
    };

    println!("{}", &output);
    ExitCode::SUCCESS
}
