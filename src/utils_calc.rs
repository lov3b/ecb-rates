use std::{borrow::BorrowMut, collections::HashMap};

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
