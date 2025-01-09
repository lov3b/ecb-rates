use clap::Parser as _;
use ecb_rates::cache::{Cache, CacheLine};
use ecb_rates::HeaderDescription;
use reqwest::{Client, IntoUrl};
use std::process::ExitCode;

use ecb_rates::cli::{Cli, FormatOption};
use ecb_rates::models::ExchangeRateResult;
use ecb_rates::parsing::parse;
use ecb_rates::table::{TableRef, TableTrait as _};
use ecb_rates::utils_calc::{change_perspective, filter_currencies, invert_rates, round};

async fn get_and_parse(url: impl IntoUrl) -> anyhow::Result<Vec<ExchangeRateResult>> {
    let client = Client::new();
    let xml_content = client.get(url).send().await?.text().await?;
    parse(&xml_content)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let mut cli = Cli::parse();
    if cli.force_color {
        colored::control::set_override(true);
    }

    let mut header_description = HeaderDescription::new();
    let use_cache = !cli.no_cache;
    let view = match cli.show_days.to_view() {
        Some(v) => v,
        None => {
            eprintln!("It doesn't make any sence to fetch 0 days right?");
            return ExitCode::SUCCESS;
        }
    };
    let mut cache = if use_cache { Cache::load(&view) } else { None };
    let cache_ok = cache.as_ref().map_or_else(
        || false,
        |c| c.get_cache_line().map_or_else(|| false, |cl| cl.is_valid()),
    );
    let mut parsed = if cache_ok {
        // These are safe unwraps
        cache
            .as_ref()
            .unwrap()
            .get_cache_line()
            .unwrap()
            .exchange_rate_results
            .clone()
    } else {
        let parsed = match get_and_parse(view.to_ecb_url()).await {
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
                        .get_cache_line()
                        .map_or_else(|| true, |cache_line| cache_line == &parsed)
                },
            );

            if not_equal_cache {
                if let Some(cache_safe) = cache.as_mut() {
                    let cache_line = CacheLine::new(parsed.clone());
                    cache_safe.set_cache_line(cache_line);
                    if let Err(e) = cache_safe.save() {
                        eprintln!("Failed to save to cache with: {:?}", e);
                    }
                }
            }
        }
        parsed
    };

    cli.perspective = cli.perspective.map(|s| s.to_uppercase());
    if let Some(currency) = cli.perspective.as_ref() {
        header_description.replace_eur(&currency);
        let error_occured = change_perspective(&mut parsed, &currency).is_none();
        if error_occured {
            eprintln!("The currency wasn't in the data from the ECB!");
            return ExitCode::FAILURE;
        }
    }

    if cli.should_invert {
        invert_rates(&mut parsed);
        header_description.invert();
    }

    round(&mut parsed, cli.max_decimals);

    if !cli.currencies.is_empty() {
        let currencies = cli
            .currencies
            .iter()
            .map(|x| x.to_uppercase())
            .collect::<Vec<_>>();

        filter_currencies(&mut parsed, &currencies);
    }

    parsed.reverse();
    let parsed = match cli.show_days.to_option() {
        Some(n) => {
            if parsed.len() <= n {
                parsed.as_slice()
            } else {
                &parsed[0..n]
            }
        }
        None => parsed.as_slice(),
    };

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
        FormatOption::Plain => {
            let rates = parsed
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
                .join("\n");
            let mut s = header_description.to_string();
            s.push_str(&rates);
            s
        }
    };

    println!("{}", &output);
    ExitCode::SUCCESS
}
