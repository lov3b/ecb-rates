use anyhow::Context;
use clap::Parser as _;
use ecb_rates::caching::{Cache, CacheLine};
use ecb_rates::HeaderDescription;
use reqwest::{Client, IntoUrl};
use smol_str::StrExt;
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

fn main() -> ExitCode {
    let cli = Cli::parse();

    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(e) => {
            eprintln!("Failed to initialize asynchronous runtime: {:?}", e);
            return ExitCode::FAILURE;
        }
    };

    match runtime.block_on(async_main(cli)) {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Fatal: {:?}", e);
            ExitCode::FAILURE
        }
    }
}

async fn async_main(mut cli: Cli) -> anyhow::Result<()> {
    if cli.force_color {
        colored::control::set_override(true);
    }

    let mut header_description = HeaderDescription::new();
    let use_cache = !cli.no_cache;
    let view = cli
        .show_days
        .to_view()
        .context("It doesn't make any sence to fetch 0 days right?")?;
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
        let parsed = get_and_parse(view.to_ecb_url())
            .await
            .context("Failed to get/parse data from ECB")?;
        if !cache_ok {
            let not_equal_cache = cache.as_ref().map_or_else(
                || true,
                |cache_local| {
                    cache_local
                        .get_cache_line()
                        .map_or_else(|| true, |cache_line| cache_line == &parsed)
                },
            );

            if not_equal_cache && let Some(cache_safe) = cache.as_mut() {
                let cache_line = CacheLine::new(parsed.clone());
                cache_safe.set_cache_line(cache_line);
                cache_safe.save()?;
            }
        }
        parsed
    };

    cli.perspective = cli.perspective.map(|s| s.to_uppercase_smolstr());
    if let Some(currency) = cli.perspective.as_ref() {
        header_description.replace_eur(currency);
        change_perspective(&mut parsed, currency)
            .context("The currency wasn't in the data from the ECB!")?;
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
            .map(|x| x.to_uppercase_smolstr())
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
    Ok(())
}
