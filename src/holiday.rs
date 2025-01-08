use chrono::{Days, NaiveDate};

/// Calculates the hollidays recognized by the EU
/// ECB recognizes the following hollidays https://www.ecb.europa.eu/ecb/contacts/working-hours/html/index.en.html
#[derive(Debug, Clone)]
pub struct Hollidays {
    hollidays: [NaiveDate; 15],
}

impl Hollidays {
    pub fn is_holliday(&self, date: &NaiveDate) -> bool {
        self.hollidays.contains(date)
    }

    pub fn new(year: i32) -> Self {
        assert!((1583..=4099).contains(&year));

        let easter_sunday = Self::calc_easter_sunday(year);
        let easter_monday = easter_sunday + Days::new(1);
        let good_friday = easter_sunday - Days::new(2);
        let ascension_day = easter_sunday + Days::new(39);
        let whit_monday = easter_sunday + Days::new(50);
        let corpus_christi = easter_sunday + Days::new(60);
        let year_years_day = unsafe { NaiveDate::from_ymd_opt(year, 1, 1).unwrap_unchecked() };
        let labour_day = unsafe { NaiveDate::from_ymd_opt(year, 5, 1).unwrap_unchecked() };
        let robert_schuman_declaration =
            unsafe { NaiveDate::from_ymd_opt(year, 5, 9).unwrap_unchecked() };
        let german_unity_day = unsafe { NaiveDate::from_ymd_opt(year, 10, 3).unwrap_unchecked() };
        let all_saints_day = unsafe { NaiveDate::from_ymd_opt(year, 11, 1).unwrap_unchecked() };
        let christmas_eve = unsafe { NaiveDate::from_ymd_opt(year, 12, 24).unwrap_unchecked() };
        let christmas_day = unsafe { NaiveDate::from_ymd_opt(year, 12, 25).unwrap_unchecked() };
        let christmas_holiday = unsafe { NaiveDate::from_ymd_opt(year, 12, 26).unwrap_unchecked() };
        let new_years_eve = unsafe { NaiveDate::from_ymd_opt(year, 12, 31).unwrap_unchecked() };

        let hollidays = [
            easter_sunday,
            easter_monday,
            good_friday,
            ascension_day,
            whit_monday,
            corpus_christi,
            year_years_day,
            labour_day,
            robert_schuman_declaration,
            german_unity_day,
            all_saints_day,
            christmas_eve,
            christmas_day,
            christmas_holiday,
            new_years_eve,
        ];
        Self { hollidays }
    }

    /// Returns Easter Sunday for a given year (Gregorian calendar).
    /// This uses a variation of the Butcher's algorithm.
    /// Valid for years 1583..=4099 in the Gregorian calendar.
    fn calc_easter_sunday(year: i32) -> NaiveDate {
        // For reference: https://en.wikipedia.org/wiki/Computus#Butcher's_algorithm
        let a = year % 19;
        let b = year / 100;
        let c = year % 100;
        let d = b / 4;
        let e = b % 4;
        let f = (b + 8) / 25;
        let g = (b - f + 1) / 3;
        let h = (19 * a + b - d - g + 15) % 30;
        let i = c / 4;
        let k = c % 4;
        let l = (32 + 2 * e + 2 * i - h - k) % 7;
        let m = (a + 11 * h + 22 * l) / 451;
        let month = (h + l - 7 * m + 114) / 31;
        let day = (h + l - 7 * m + 114) % 31 + 1;

        NaiveDate::from_ymd_opt(year, month as u32, day as u32)
            .expect("Invalid date calculation for Easter Sunday")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_holidays_2025() {
        let year = 2025;
        let holliday = Hollidays::new(year);

        let easter_sunday_2025 = NaiveDate::from_ymd_opt(2025, 4, 20).unwrap();
        assert!(
            holliday.is_holliday(&easter_sunday_2025),
            "Easter Sunday 2025"
        );

        let new_years_2025 = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        assert!(holliday.is_holliday(&new_years_2025), "New Year's Day 2025");

        let labour_day_2025 = NaiveDate::from_ymd_opt(2025, 5, 1).unwrap();
        assert!(holliday.is_holliday(&labour_day_2025), "Labour Day 2025");

        let random_workday_2025 = NaiveDate::from_ymd_opt(2025, 2, 10).unwrap();
        assert!(
            !holliday.is_holliday(&random_workday_2025),
            "Random weekday 2025"
        );
    }

    #[test]
    fn test_holidays_2026() {
        let year = 2026;
        let holliday = Hollidays::new(year);

        let easter_sunday_2026 = NaiveDate::from_ymd_opt(2026, 4, 5).unwrap();
        assert!(
            holliday.is_holliday(&easter_sunday_2026),
            "Easter Sunday 2026"
        );

        let german_unity_day_2026 = NaiveDate::from_ymd_opt(2026, 10, 3).unwrap();
        assert!(
            holliday.is_holliday(&german_unity_day_2026),
            "Day of German Unity 2026"
        );

        let random_workday_2026 = NaiveDate::from_ymd_opt(2026, 2, 10).unwrap();
        assert!(
            !holliday.is_holliday(&random_workday_2026),
            "Random weekday 2026"
        );
    }

    #[test]
    #[should_panic]
    fn test_year_too_low() {
        disable_panic_stack_trace();
        let _ = Hollidays::new(1000); 
    }

    #[test]
    #[should_panic]
    fn test_year_too_high() {
        disable_panic_stack_trace();
        let _ = Hollidays::new(9999); 
    }

    fn disable_panic_stack_trace() {
        std::panic::set_hook(Box::new(|x| {
            let _ = x;
        }));
    }
}
