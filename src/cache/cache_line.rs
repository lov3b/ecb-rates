use std::rc::Rc;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDate, TimeDelta, Utc, Weekday};
use serde::{Deserialize, Serialize};

use crate::models::ExchangeRateResult;
use crate::Hollidays;

const CET: FixedOffset = unsafe { FixedOffset::east_opt(3600).unwrap_unchecked() };

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CacheLine {
    #[serde(with = "ts_seconds")]
    date: DateTime<Utc>,

    #[serde(rename = "camelCase")]
    pub exchange_rate_results: Vec<ExchangeRateResult>,
}

impl CacheLine {
    pub fn is_valid(&self) -> bool {
        self.is_valid_at(Local::now().with_timezone(&CET))
    }

    pub fn is_valid_at(&self, now_cet: DateTime<FixedOffset>) -> bool {
        let saved_cet = self.date.with_timezone(&CET);

        // Shortcut: if the saved time is somehow *in the future* vs. 'now', treat as invalid.
        if saved_cet > now_cet {
            return false;
        }

        // This can be optimized, but it won't make a difference for the application
        let hollidays_opt = if now_cet.year() == saved_cet.year() {
            Some(Rc::new(Hollidays::new(now_cet.year())))
        } else {
            None
        };

        let mut day_iter = saved_cet.date_naive();
        let end_day = now_cet.date_naive();

        // Helper: checks if a day is open (ECB publishes).
        // weekend (Sat/Sun) or holiday is "closed".
        let is_open_day = |date: NaiveDate| {
            let wd = date.weekday();
            let is_weekend = wd == Weekday::Sat || wd == Weekday::Sun;

            let hollidays = hollidays_opt
                .clone()
                .unwrap_or_else(|| Rc::new(Hollidays::new(date.year())));

            let is_holiday = hollidays.is_holliday(&date);

            !(is_weekend || is_holiday)
        };

        while day_iter <= end_day {
            if is_open_day(day_iter) {
                // Potential publish time is day_iter at 16:00 CET
                let publish_time_cet = unsafe {
                    day_iter
                        .and_hms_opt(16, 0, 0)
                        .unwrap_unchecked()
                        .and_local_timezone(CET)
                        .unwrap()
                };

                if publish_time_cet > saved_cet && publish_time_cet <= now_cet {
                    return false;
                }
            }
            day_iter += TimeDelta::days(1);
        }

        // If we never found an open day’s 16:00 that invalidates the cache, we're good.
        true
    }

    pub fn new(exchange_rate_results: Vec<ExchangeRateResult>) -> Self {
        let date = Local::now().to_utc();
        Self {
            exchange_rate_results,
            date,
        }
    }
}

impl PartialEq<Vec<ExchangeRateResult>> for CacheLine {
    fn eq(&self, other: &Vec<ExchangeRateResult>) -> bool {
        &self.exchange_rate_results == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn cl(date_utc: DateTime<Utc>) -> CacheLine {
        CacheLine {
            date: date_utc,
            exchange_rate_results: vec![],
        }
    }

    #[test]
    fn test_cache_in_future() {
        let now_cet = Utc
            .with_ymd_and_hms(2025, 1, 1, 9, 0, 0)
            .unwrap()
            .with_timezone(&CET);
        let future_utc = Utc.with_ymd_and_hms(2025, 1, 2, 9, 0, 0).unwrap();
        assert!(!cl(future_utc).is_valid_at(now_cet));
    }

    #[test]
    fn test_same_open_day_before_16() {
        let now_cet = Utc
            .with_ymd_and_hms(2025, 1, 8, 12, 0, 0)
            .unwrap()
            .with_timezone(&CET);
        let cache_utc = Utc.with_ymd_and_hms(2025, 1, 8, 10, 0, 0).unwrap();
        assert!(cl(cache_utc).is_valid_at(now_cet));
    }

    #[test]
    fn test_same_day_after_16() {
        let now_cet = Utc
            .with_ymd_and_hms(2025, 1, 8, 17, 0, 0)
            .unwrap()
            .with_timezone(&CET);
        let cache_utc = Utc.with_ymd_and_hms(2025, 1, 8, 14, 0, 0).unwrap();
        assert!(!cl(cache_utc).is_valid_at(now_cet));
    }

    #[test]
    fn test_saved_after_16_same_day() {
        let now_cet = Utc
            .with_ymd_and_hms(2025, 1, 8, 18, 0, 0)
            .unwrap()
            .with_timezone(&CET);
        let cache_utc = Utc.with_ymd_and_hms(2025, 1, 8, 17, 0, 0).unwrap();
        assert!(cl(cache_utc).is_valid_at(now_cet));
    }

    #[test]
    fn test_multi_day_old_cache_should_invalidate_if_open_day_passed() {
        let now_cet = Utc
            .with_ymd_and_hms(2025, 1, 10, 18, 0, 0)
            .unwrap()
            .with_timezone(&CET);
        let cache_utc = Utc.with_ymd_and_hms(2025, 1, 5, 10, 0, 0).unwrap();
        assert!(!cl(cache_utc).is_valid_at(now_cet));
    }

    #[test]
    fn test_multi_day_holiday_scenario() {
        let now_cet = Utc
            .with_ymd_and_hms(2025, 12, 26, 19, 0, 0)
            .unwrap()
            .with_timezone(&CET);
        let cache_utc = Utc.with_ymd_and_hms(2025, 12, 24, 10, 0, 0).unwrap();
        assert!(cl(cache_utc).is_valid_at(now_cet));
    }
}
