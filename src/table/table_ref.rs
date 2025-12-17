use std::fmt::Display;

use crate::cli::SortBy;
use crate::models::{Currency, ExchangeRateResult};
use crate::DEFAULT_WIDTH;

use super::table_display::helper_table_print;
use super::table_getter::TableGet;
use super::table_trait::TableTrait;
use super::Table;

pub struct TableRef<'a> {
    header: Option<&'a str>,
    column_left: &'a str,
    column_right: &'a str,
    rows: Vec<(&'a Currency, f64)>,
    pub color: bool,
    pub width: usize,
    pub left_offset: usize,
}

impl<'a> TableTrait<'a> for TableRef<'a> {
    type Header = &'a str;
    type ColumnLeft = &'a str;
    type ColumnRight = &'a str;
    type RowLeft = &'a Currency;

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
        self.rows.sort_by(comparer);
    }
}

impl<'a> TableGet for TableRef<'a> {
    type RowLeftRef = &'a Currency;
    type RowRightRef = &'a str;

    fn get_header(&self) -> Option<&str> {
        self.header
    }
    fn get_column_left(&self) -> &str {
        self.column_left
    }
    fn get_column_right(&self) -> &str {
        self.column_right
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

impl<'a> From<&'a ExchangeRateResult> for TableRef<'a> {
    fn from(value: &'a ExchangeRateResult) -> Self {
        let mut table = TableRef::new(Some(&value.time), "Currency", "Rate");
        for (key, val) in value.rates.iter() {
            table.add_row(key, *val);
        }

        table
    }
}

impl<'a> Display for TableRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        helper_table_print(f, self)
    }
}

impl<'a> From<&'a Table> for TableRef<'a> {
    fn from(table: &'a Table) -> Self {
        let rows = table
            .rows
            .iter()
            .map(|(left, right)| (left, *right))
            .collect();

        TableRef {
            header: table.header.as_deref(),
            column_left: table.column_left.as_str(),
            column_right: table.column_right.as_str(),
            rows,
            color: table.color,
            width: table.width,
            left_offset: table.left_offset,
        }
    }
}
