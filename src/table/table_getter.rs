pub trait TableGet {
    type RowLeftRef: AsRef<str>;
    type RowRightRef: AsRef<str>;

    fn get_header(&self) -> Option<&str>;
    fn get_column_left(&self) -> &str;
    fn get_column_right(&self) -> &str;
    fn get_rows(&self) -> &Vec<(Self::RowLeftRef, f64)>;
    fn get_width(&self) -> usize;
    fn get_left_offset(&self) -> usize;
}
