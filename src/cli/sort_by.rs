use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone)]
pub enum SortBy {
    Currency,
    Rate,
}

impl SortBy {
    pub fn get_comparer(&self) -> fn(&(&str, f64), &(&str, f64)) -> std::cmp::Ordering {
        match self {
            Self::Currency => |a, b| a.0.cmp(b.0),
            Self::Rate => |a, b| a.1.total_cmp(&b.1),
        }
    }
}
