use std::collections::HashMap;

use quick_xml::Reader;
use quick_xml::events::Event;
use smol_str::SmolStr;

use crate::models::ExchangeRateResult;

fn smol_from_utf8(bytes: &[u8]) -> SmolStr {
    str::from_utf8(bytes)
        .map(SmolStr::new)
        .unwrap_or_else(|_| SmolStr::new(String::from_utf8_lossy(bytes)))
}

pub fn parse(xml: &str) -> anyhow::Result<Vec<ExchangeRateResult>> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut results = Vec::new();
    let mut current_time: Option<SmolStr> = None;
    let mut inside_cube_time = false;
    let mut current_rates = HashMap::new();

    fn handle_cube_element(
        e: &quick_xml::events::BytesStart,
        current_time: &mut Option<SmolStr>,
        inside_cube_time: &mut bool,
        current_rates: &mut HashMap<SmolStr, f64>,
        results: &mut Vec<ExchangeRateResult>,
    ) -> anyhow::Result<()> {
        if e.name().local_name().as_ref() != b"Cube" {
            return Ok(());
        }

        let mut time_attr: Option<SmolStr> = None;
        let mut currency_attr: Option<SmolStr> = None;
        let mut rate_attr: Option<SmolStr> = None;

        for attr_result in e.attributes() {
            let attr = attr_result?;
            let key = attr.key.as_ref();
            let val = smol_from_utf8(attr.value.as_ref());

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

        if *inside_cube_time && let (Some(c), Some(r_str)) = (currency_attr, rate_attr) {
            let r = r_str.parse::<f64>()?;
            current_rates.insert(c, r);
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
