use std::fs;
use crate::Row;
use crate::Position;
#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
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
            file_name: Some(file.to_string()),
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
    pub fn insert(&mut self, pos: &Position, c: char) {
        if pos.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if pos.y < self.len() {
            let row = self.rows.get_mut(pos.y).unwrap();
            row.insert(pos.x, c);
        }
    }
    pub fn delete(&mut self, pos: &Position) {
        if pos.y >= self.len() {
            return;
        }
        let row = self.rows.get_mut(pos.y).unwrap();
        row.delete(pos.x);
    }
    pub fn backspace(&mut self, pos: &Position) {
        if pos.y >= self.len() {
            return;
        }
        let row = self.rows.get_mut(pos.y).unwrap();
        row.backspace(pos.x);
    }
}