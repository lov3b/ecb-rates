pub mod cache;
pub mod cli;
mod header_description;
mod holiday;
pub mod models;
pub mod os;
pub mod parsing;
pub mod table;
pub mod utils_calc;
mod view;

pub use header_description::HeaderDescription;
pub use holiday::Hollidays;
pub use view::View;

const APP_NAME: &'static str = "ECB-rates";
const DEFAULT_WIDTH: usize = 20;

pub mod ecb_url {
    pub const TODAY: &'static str = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";

    pub mod hist {
        pub const DAYS_ALL: &'static str =
            "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist.xml";
        pub const DAYS_90: &'static str =
            "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-hist-90d.xml";
    }
}
