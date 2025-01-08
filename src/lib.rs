pub mod cache;
pub mod cli;
mod holiday;
pub mod models;
pub mod os;
pub mod parsing;
pub mod table;
pub mod utils_calc;

pub use holiday::Hollidays;

const APP_NAME: &'static str = "ECB-rates";

pub mod ecb_url {
    pub const TODAY: &'static str = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";

    pub mod hist {
        pub const DAYS_ALL: &'static str =
            "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.xml";
        pub const DAYS_90: &'static str =
            "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist-90d.xml";
    }
}
