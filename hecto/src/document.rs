use crate::Row;

#[derive(Debug, Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    #[must_use]
    pub fn open() -> Self {
        Self {
            rows: vec!["Hello, World!".into()],
        }
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
