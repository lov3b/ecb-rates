use clap::{Parser, Subcommand};
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, Serialize)]
pub struct ExchangeRateResult {
    pub time: String,
    pub rates: HashMap<String, f64>,
}

const ECB_DAILY: &'static str = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";
const ECB_HIST_DAILY: &'static str = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.xml";
const ECB_HIST_90: &'static str =
    "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist-90d.xml";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let xml_content = client.get(ECB_DAILY).send().await?.text().await?;
    let parsed = parse(&xml_content).unwrap();
    println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
    Ok(())
}

fn parse(xml: &str) -> Result<Vec<ExchangeRateResult>, Box<dyn Error>> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut results = Vec::new();
    let mut current_time: Option<String> = None;
    let mut inside_cube_time = false;
    let mut current_rates = HashMap::new();

    fn handle_cube_element(
        e: &quick_xml::events::BytesStart,
        current_time: &mut Option<String>,
        inside_cube_time: &mut bool,
        current_rates: &mut HashMap<String, f64>,
        results: &mut Vec<ExchangeRateResult>,
    ) -> Result<(), Box<dyn Error>> {
        if e.name().local_name().as_ref() != b"Cube" {
            return Ok(());
        }

        let mut time_attr: Option<String> = None;
        let mut currency_attr: Option<String> = None;
        let mut rate_attr: Option<String> = None;

        // Check attributes to see if it's a time-labeled Cube or a currency-labeled Cube
        for attr_result in e.attributes() {
            let attr = attr_result?;
            let key = attr.key.as_ref();
            let val = String::from_utf8_lossy(attr.value.as_ref()).to_string();

            match key {
                b"time" => {
                    time_attr = Some(val);
                }
                b"currency" => {
                    currency_attr = Some(val);
                }
                b"rate" => {
                    rate_attr = Some(val);
                }
                _ => {}
            }
        }

        // If we found a time attribute, it means we're at "Cube time='...'"
        if let Some(t) = time_attr {
            // If we already had a current_time, that means we finished one block
            if current_time.is_some() {
                let previous_time = current_time.take().unwrap();
                results.push(ExchangeRateResult {
                    time: previous_time,
                    rates: current_rates.clone(),
                });
                current_rates.clear();
            }
            // Now set the new time
            *current_time = Some(t);
            *inside_cube_time = true;
        }

        // If we're inside an existing time block and we see currency/rate, store it
        if *inside_cube_time {
            if let (Some(c), Some(r_str)) = (currency_attr, rate_attr) {
                let r = r_str.parse::<f64>()?;
                current_rates.insert(c, r);
            }
        }

        Ok(())
    }

    // Main parsing loop
    while let Ok(event) = reader.read_event() {
        match event {
            // For normal (non-self-closing) <Cube> tags
            Event::Start(e) => {
                handle_cube_element(
                    &e,
                    &mut current_time,
                    &mut inside_cube_time,
                    &mut current_rates,
                    &mut results,
                )?;
            }

            // For self-closing <Cube .../> tags (like currency lines!)
            Event::Empty(e) => {
                handle_cube_element(
                    &e,
                    &mut current_time,
                    &mut inside_cube_time,
                    &mut current_rates,
                    &mut results,
                )?;
            }
            Event::Eof => break,
            _ => {} // Event::End is here aswell
        }
    }

    // If the document ended and we still have one block in memory
    if let Some(last_time) = current_time {
        results.push(ExchangeRateResult {
            time: last_time,
            rates: current_rates,
        });
    }

    Ok(results)
}
