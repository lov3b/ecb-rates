use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExchangeRateResult {
    pub time: String,
    pub rates: HashMap<String, f64>,
}
