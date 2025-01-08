use clap::Parser as _;
use ecb_rates::cache::{Cache, CacheLine};
use reqwest::{Client, IntoUrl};
use std::{borrow::BorrowMut, collections::HashMap, process::ExitCode};

use ecb_rates::cli::{Cli, FormatOption};
use ecb_rates::models::ExchangeRateResult;
use ecb_rates::parsing::parse;
use ecb_rates::table::{TableRef, TableTrait as _};

async fn get_and_parse(url: impl IntoUrl) -> anyhow::Result<Vec<ExchangeRateResult>> {
    let client = Client::new();
    let xml_content = client.get(url).send().await?.text().await?;
    parse(&xml_content)
}

fn filter_currencies(exchange_rate_results: &mut [ExchangeRateResult], currencies: &[String]) {
    for exchange_rate in exchange_rate_results {
        let rates_ptr: *mut HashMap<String, f64> = &mut exchange_rate.rates;
        exchange_rate
            .rates
            .keys()
            .filter(|x| !currencies.contains(x))
            .for_each(|key_to_remove| {
                /* This is safe, since we:
                 * 1. Already have a mutable reference.
                 * 2. Don't run the code in paralell
                 */
                let rates = unsafe { (*rates_ptr).borrow_mut() };
                rates.remove_entry(key_to_remove);
            });
    }
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
                .map_or_else(|| false, |cl| cl.validate())
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
            if let Some(cache_safe) = cache.as_mut() {
                let cache_line = CacheLine::new(parsed.clone());
                cache_safe.set_cache_line(cli.resolution, cache_line);
                if let Err(e) = cache_safe.save() {
                    eprintln!("Failed to save to cache with: {:?}", e);
                }
            }
        }
        parsed
    };

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

            if !cli.display_time {
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
                t.sort();
                format!("{}", t)
            })
            .collect::<Vec<_>>()
            .join("\n"),
    };

    println!("{}", &output);
    ExitCode::SUCCESS
}
