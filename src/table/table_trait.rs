pub trait TableTrait<'a> {
    type Header;
    type ColumnLeft;
    type ColumnRight;
    type RowLeft;

    fn new(header: Option<Self::Header>, column_left: Self::ColumnLeft, column_right: Self::ColumnRight) -> Self;
    fn disable_header(&mut self);
    fn set_header(&mut self, header: Self::Header);
    fn add_row(&mut self, row_left: Self::RowLeft, row_right: f64);
    fn sort(&mut self);
}
