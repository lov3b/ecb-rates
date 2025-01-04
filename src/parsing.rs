use std::collections::HashMap;
use std::error::Error;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::models::ExchangeRateResult;

pub fn parse(xml: &str) -> Result<Vec<ExchangeRateResult>, Box<dyn Error>> {
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

        if let Some(t) = time_attr {
            if current_time.is_some() {
                let previous_time = current_time.take().unwrap();
                results.push(ExchangeRateResult {
                    time: previous_time,
                    rates: current_rates.clone(),
                });
                current_rates.clear();
            }
            *current_time = Some(t);
            *inside_cube_time = true;
        }

        if *inside_cube_time {
            if let (Some(c), Some(r_str)) = (currency_attr, rate_attr) {
                let r = r_str.parse::<f64>()?;
                current_rates.insert(c, r);
            }
        }

        Ok(())
    }

    while let Ok(event) = reader.read_event() {
        match event {
            Event::Start(e) => {
                handle_cube_element(
                    &e,
                    &mut current_time,
                    &mut inside_cube_time,
                    &mut current_rates,
                    &mut results,
                )?;
            }

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

    if let Some(last_time) = current_time {
        results.push(ExchangeRateResult {
            time: last_time,
            rates: current_rates,
        });
    }

    Ok(results)
}
