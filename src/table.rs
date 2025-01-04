use colored::*;
use std::fmt::Display;

use crate::models::ExchangeRateResult;

pub struct Table {
    header: Option<String>,
    column_left: String,
    column_right: String,
    rows: Vec<(String, String)>,
    pub color: bool,
    pub width: usize,
}

impl Table {
    fn new(header: Option<String>, column_left: String, column_right: String) -> Self {
        Self {
            header,
            column_left,
            column_right,
            rows: Vec::new(),
            color: false,
            width: 21,
        }
    }

    fn disable_header(&mut self) {
        self.header = None
    }

    fn set_header(&mut self, header: String) {
        self.header = Some(header);
    }

    fn add_row(&mut self, row_left: String, row_right: String) {
        self.rows.push((row_left, row_right));
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(header) = self.header.as_ref() {
            let middle_padding_amount = (self.width - header.len()) / 2;
            assert!(middle_padding_amount > 0);
            let middle_padding = " ".repeat(middle_padding_amount);
            writeln!(
                f,
                "{}{}{}",
                middle_padding,
                header.bold().cyan(),
                middle_padding
            )?;
        }

        let right_padding_amount = self.width - self.column_left.len() - self.column_right.len();
        let right_padding = " ".repeat(right_padding_amount);
        writeln!(
            f,
            "{}{}{}",
            self.column_left.bold().yellow(),
            right_padding,
            self.column_right.bold().yellow()
        )?;
        writeln!(f, "{}", "-".repeat(self.width))?;

        for (left, right) in self.rows.iter() {
            let padding_amount = self.width - left.len() - right.len();
            let padding = " ".repeat(padding_amount);
            writeln!(f, "{}{}{}", left.bold().green(), padding, right)?;
        }

        Ok(())
    }
}

impl From<ExchangeRateResult> for Table {
    fn from(value: ExchangeRateResult) -> Self {
        let mut table = Table::new(Some(value.time), "Currency".to_string(), "Rate".to_string());
        for (key, val) in value.rates.into_iter() {
            table.add_row(key, val.to_string());
        }

        table
    }
}
