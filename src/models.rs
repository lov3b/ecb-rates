use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExchangeRateResult {
    pub time: SmolStr,
    pub rates: HashMap<SmolStr, f64>,
}
