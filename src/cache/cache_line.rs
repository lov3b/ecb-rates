use chrono::serde::ts_seconds;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

use crate::models::ExchangeRateResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheLine {
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,

    #[serde(rename = "camelCase")]
    pub exchange_rate_results: Vec<ExchangeRateResult>,
}

impl CacheLine {
    pub fn validate(&self) -> bool {
        let today = Local::now().naive_local().date();
        let saved = self.date.naive_local().date();
        saved == today
    }

    pub fn new(exchange_rate_results: Vec<ExchangeRateResult>) -> Self {
        let date = Local::now().to_utc();
        Self {
            exchange_rate_results,
            date,
        }
    }
}
