pub mod cli;
pub mod parsing;
pub mod models;

pub mod ecb_url {
    pub const TODAY: &'static str = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";

    pub mod hist {
        pub const DAILY: &'static str =
            "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.xml";
        pub const DAYS_90: &'static str =
            "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist-90d.xml";
    }
}
