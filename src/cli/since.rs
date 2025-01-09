use crate::View;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub enum ShowDays {
    Days(usize),
    All,
}

impl FromStr for ShowDays {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("all") {
            return Ok(ShowDays::All);
        }
        s.parse::<usize>().map(ShowDays::Days).map_err(|_| {
            format!(
                "Invalid value for since: '{}'. Use a positive integer or 'start'.",
                s
            )
        })
    }
}
impl fmt::Display for ShowDays {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShowDays::Days(days) => write!(f, "{}", days),
            ShowDays::All => write!(f, "all"),
        }
    }
}

impl ShowDays {
    /// None represents infinity
    pub fn to_option(&self) -> Option<usize> {
        match self {
            ShowDays::Days(d) => Some(*d),
            ShowDays::All => None,
        }
    }

    pub fn to_view(&self) -> Option<View> {
        match self {
            ShowDays::Days(d) => match d {
                0 => None,
                1 => Some(View::TODAY),
                2..=90 => Some(View::HistDays90),
                91.. => Some(View::HistDaysAll),
            },
            ShowDays::All => Some(View::HistDaysAll),
        }
    }
}
