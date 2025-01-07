use clap::Parser as _;
use ecb_rates::cache::Cache;
use reqwest::{Client, IntoUrl};
use std::{borrow::BorrowMut, collections::HashMap, error::Error, process::ExitCode};

use ecb_rates::cli::{Cli, FormatOption};
use ecb_rates::models::ExchangeRateResult;
use ecb_rates::parsing::parse;
use ecb_rates::table::Table;

async fn get_and_parse(url: impl IntoUrl) -> Result<Vec<ExchangeRateResult>, Box<dyn Error>> {
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
                let rates = unsafe { (*rates_ptr).borrow_mut() };
                rates.remove_entry(key_to_remove);
            });
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let use_cache = !cli.no_cache;
    let cache = if use_cache { Cache::load() } else { None };
    let cache_ok = cache.as_ref().map_or_else(|| false, |c| c.validate());
    let mut parsed = if cache_ok {
        cache.as_ref().unwrap().exchange_rate_results.clone()
    } else {
        match get_and_parse(cli.resolution.to_ecb_url()).await {
            Ok(k) => k,
            Err(e) => {
                eprintln!("Failed to get/parse data from ECB: {}", e);
                return ExitCode::FAILURE;
            }
        }
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
                for json_value in json_values.iter_mut() {
                    if let Some(map) = json_value.as_object_mut() {
                        map.remove_entry("time");
                    }
                }
            }

            if cli.compact {
                serde_json::to_string(&json_values)
            } else {
                serde_json::to_string_pretty(&json_values)
            }
            .expect("Failed to parse content as JSON")
        }
        FormatOption::Plain => parsed
            .iter()
            .map(|x| {
                let t: Table = x.clone().into();
                format!("{}", t)
            })
            .collect::<Vec<_>>()
            .join("\n"),
    };

    println!("{}", &output);
    if !cache_ok {
        if let Err(e) = Cache::new(parsed).save() {
            eprintln!("Failed to save to cache with: {:?}", e);
        }
    }
    ExitCode::SUCCESS
}
