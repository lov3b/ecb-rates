use std::{borrow::BorrowMut, collections::HashMap, ops::Deref};

use crate::models::ExchangeRateResult;

pub fn filter_currencies(exchange_rate_results: &mut [ExchangeRateResult], currencies: &[String]) {
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

pub fn change_perspective(
    exchange_rate_results: &mut [ExchangeRateResult],
    currency: &str,
) -> Option<()> {
    for rate_res in exchange_rate_results {
        let currency_rate = rate_res.rates.remove(currency)?;
        let eur_rate = 1.0 / currency_rate;

        for (_, iter_rate) in rate_res.rates.iter_mut() {
            *iter_rate = eur_rate * iter_rate.deref();
        }

        rate_res.rates.insert("EUR".to_string(), eur_rate);
    }
    Some(())
}

pub fn invert_rates(exchange_rate_results: &mut [ExchangeRateResult]) {
    for rate_res in exchange_rate_results {
        for (_, iter_rate) in rate_res.rates.iter_mut() {
            *iter_rate = 1.0 / *iter_rate;
        }
    }
}

pub fn round(exchange_rate_results: &mut [ExchangeRateResult], max_decimals: u8) {
    let power = 10.0_f64.powf(max_decimals as f64);
    for rate_res in exchange_rate_results {
        for (_, iter_rate) in rate_res.rates.iter_mut() {
            let more = iter_rate.deref() * power;
            *iter_rate = more.round() / power;
        }
    }
}
