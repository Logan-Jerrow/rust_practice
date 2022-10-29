use std::{io, path::Path};

use crate::Row;

#[derive(Debug, Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    #[must_use]
    pub fn open<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let file_contents = std::fs::read_to_string(filename)?;
        let rows: Vec<Row> = file_contents.lines().map(|line| line.into()).collect();

        Ok(Self { rows })
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
