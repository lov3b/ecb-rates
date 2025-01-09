use std::fmt::Display;

use crate::cli::SortBy;
use crate::models::ExchangeRateResult;
use crate::DEFAULT_WIDTH;

use super::table_display::helper_table_print;
use super::{TableGet, TableTrait};

pub struct Table {
    pub(super) header: Option<String>,
    pub(super) column_left: String,
    pub(super) column_right: String,
    pub(super) rows: Vec<(String, f64)>,
    pub color: bool,
    pub width: usize,
    pub left_offset: usize,
}

impl<'a> TableTrait<'a> for Table {
    type Header = String;
    type ColumnLeft = String;
    type ColumnRight = String;
    type RowLeft = String;

    fn new(
        header: Option<Self::Header>,
        column_left: Self::ColumnLeft,
        column_right: Self::ColumnRight,
    ) -> Self {
        Self {
            header,
            column_left,
            column_right,
            rows: Vec::new(),
            color: false,
            width: DEFAULT_WIDTH,
            left_offset: 1,
        }
    }

    fn disable_header(&mut self) {
        self.header = None;
    }

    fn set_header(&mut self, header: Self::Header) {
        self.header = Some(header);
    }

    fn add_row(&mut self, row_left: Self::RowLeft, row_right: f64) {
        self.rows.push((row_left, row_right));
    }

    fn sort(&mut self, sort_by: &SortBy) {
        let comparer = sort_by.get_comparer();
        self.rows
            .sort_by(|a, b| comparer(&(&a.0, a.1), &(&b.0, b.1)));
    }
}

impl TableGet for Table {
    type RowLeftRef = String;
    type RowRightRef = String;

    fn get_header(&self) -> Option<&str> {
        self.header.as_deref()
    }
    fn get_column_left(&self) -> &str {
        &self.column_left
    }
    fn get_column_right(&self) -> &str {
        &self.column_right
    }
    fn get_rows(&self) -> &Vec<(Self::RowLeftRef, f64)> {
        &self.rows
    }
    fn get_width(&self) -> usize {
        self.width
    }
    
    fn get_left_offset(&self) -> usize {
        self.left_offset
    }
}

impl From<ExchangeRateResult> for Table {
    fn from(value: ExchangeRateResult) -> Self {
        let mut table = Table::new(Some(value.time), "Currency".to_string(), "Rate".to_string());
        for (key, val) in value.rates.into_iter() {
            table.add_row(key, val);
        }

        table
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        helper_table_print(f, self)
    }
}
