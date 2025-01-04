use clap::Parser as _;
use reqwest::{Client, IntoUrl};
use std::{borrow::BorrowMut, collections::HashMap, error::Error, process::ExitCode};

use ecb_rates::cli::{Cli, FormatOption};
use ecb_rates::models::ExchangeRateResult;
use ecb_rates::parsing::parse;

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

    let mut parsed = match get_and_parse(cli.resolution.to_ecb_url()).await {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Failed to get/parse data from ECB: {}", e);
            return ExitCode::FAILURE;
        }
    };

    if !cli.currencies.is_empty() {
        filter_currencies(&mut parsed, &cli.currencies);
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
        FormatOption::Plain => {
            struct StringCur<'a> {
                time: &'a String,
                cur: String,
            }

            let separator = if cli.compact { ", " } else { "\n" };

            let string_curred = parsed.iter().map(|entry| {
                let s = entry
                    .rates
                    .iter()
                    .map(|(cur, rate)| format!("{}: {}", cur, rate))
                    .collect::<Vec<_>>()
                    .join(&separator);

                StringCur {
                    time: &entry.time,
                    cur: s,
                }
            });

            let time_sep = if cli.compact { ": " } else { "\n" };
            let mut buf = String::new();
            for sc in string_curred {
                if cli.display_time {
                    buf.push_str(&sc.time);
                    buf.push_str(time_sep);
                }
                buf.push_str(&sc.cur);
                buf.push_str(&separator);
                buf.push('\n');
            }

            buf
        }
    };

    println!("{}", &output);
    ExitCode::SUCCESS
}
