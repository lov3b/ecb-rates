use colored::Colorize;

use super::TableGet;

pub fn helper_table_print<T: TableGet>(
    f: &mut std::fmt::Formatter<'_>,
    table: &T,
) -> std::fmt::Result {
    let width = table.get_width();

    if let Some(header) = table.get_header() {
        let middle_padding_amount = (width - header.len()) / 2;
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

    let column_left = table.get_column_left();
    let column_right = table.get_column_right();
    let right_padding_amount = width - column_left.len() - column_right.len();
    let right_padding = " ".repeat(right_padding_amount);
    writeln!(
        f,
        "{}{}{}",
        column_left.bold().yellow(),
        right_padding,
        column_right.bold().yellow()
    )?;
    writeln!(f, "{}", "-".repeat(width))?;

    for (left, right) in table.get_rows().iter() {
        let left_str = left.as_ref();
        let right_str = format!("{:.5}", right);
        let padding_amount = width.saturating_sub(left_str.len() + right_str.len());
        let padding = " ".repeat(padding_amount);
        writeln!(f, "{}{}{}", left_str.bold().green(), padding, right_str)?;
    }

    Ok(())
}
