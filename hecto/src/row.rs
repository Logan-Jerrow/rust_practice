use std::cmp;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Row {
    string: String,
    length: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            length: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    #[must_use]
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        // self.string.get(start..end).unwrap_or_default().to_string()

        self.string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .map(|g| if g == "\t" { " " } else { g })
            .collect()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.length
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn update_len(&mut self) {
        self.length = self.string.graphemes(true).count();
    }
}

#[cfg(test)]
mod tests {
    use super::Row;

    #[test]
    fn grapheme_len() {
        let s = "ä";
        let row = Row::from(s);
        assert_eq!(s.len(), 2);
        assert_eq!(row.len(), 1);
    }
}
