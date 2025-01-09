use crate::ecb_url;

pub enum View {
    TODAY,
    HistDays90,
    HistDaysAll,
}

impl View {
    pub fn to_ecb_url(&self) -> &'static str {
        match self {
            Self::TODAY => ecb_url::TODAY,
            Self::HistDays90 => ecb_url::hist::DAYS_90,
            Self::HistDaysAll => ecb_url::hist::DAYS_ALL,
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Self::TODAY => "today",
            Self::HistDays90 => "last-90-days",
            Self::HistDaysAll => "all-days",
        }
    }
}
