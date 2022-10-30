use std::io;

use crate::Row;

#[derive(Debug, Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    /// open document in editor
    ///
    /// # Errors
    /// [`std::io::Error`]s if file is not found.
    pub fn open(file_name: &str) -> io::Result<Self> {
        let file_contents = std::fs::read_to_string(file_name)?;
        let rows: Vec<Row> = file_contents.lines().map(Into::into).collect();

        Ok(Self {
            rows,
            file_name: Some(file_name.to_string()),
        })
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
