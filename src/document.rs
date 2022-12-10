use std::fs;
use crate::Row;
#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}
impl Document {
    pub fn open(file: &str) -> std::io::Result<Self> {
        let contents = fs::read_to_string(file)?;
        let mut rows = Vec::new();
        for line in contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Self {
            rows,
        })
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    } 
}