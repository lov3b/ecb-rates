use colored::Colorize;
use std::fmt::Display;

use crate::DEFAULT_WIDTH;

pub struct HeaderDescription<'a> {
    header_description: [&'a str; 2],
}

impl<'a> HeaderDescription<'a> {
    pub fn new() -> Self {
        Self {
            header_description: ["EUR", /*"\u{2217}"*/ "ALL"], // Unicode is ∗
        }
    }

    pub fn invert(&mut self) {
        self.header_description.swap(0, 1);
    }

    pub fn replace_eur(&mut self, currency: &'a str) {
        self.header_description[0] = currency;
    }
}

impl<'a> Display for HeaderDescription<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = DEFAULT_WIDTH - 2;
        let formatted = format!(
            "{} {} {}",
            self.header_description[0].purple().bold(),
            "to".italic(),
            self.header_description[1].purple().bold()
        );
        let unformatted_len =
            self.header_description[0].len() + self.header_description[1].len() + 4;
        let left_padding = " ".repeat((width - unformatted_len) / 2);

        let vertical = "═".repeat(width);
        writeln!(f, " ╔{}╗", &vertical)?;
        writeln!(
            f,
            "  {}{}{} ",
            &left_padding,
            formatted,
            " ".repeat(width - left_padding.len() - unformatted_len)
        )?;
        writeln!(f, " ╚{}╝\n", &vertical)?;
        Ok(())
    }
}
