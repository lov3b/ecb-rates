use clap::ValueEnum;

use crate::models::Currency;

#[derive(Debug, ValueEnum, Clone)]
pub enum SortBy {
    Currency,
    Rate,
}

impl SortBy {
    pub fn get_comparer(&self) -> fn(&(&Currency, f64), &(&Currency, f64)) -> std::cmp::Ordering {
        match self {
            Self::Currency => |a, b| a.0.as_ref().cmp(b.0.as_ref()),
            Self::Rate => |a, b| a.1.total_cmp(&b.1),
        }
    }
}
