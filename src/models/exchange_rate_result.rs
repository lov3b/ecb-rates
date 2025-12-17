use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Currency;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExchangeRateResult {
    pub time: String,
    pub rates: HashMap<Currency, f64>,
}
