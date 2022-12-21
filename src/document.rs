use std::fs;
use crate::Row;
use crate::Position;
use std::io::{Error, Write};
#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file: Option<String>,
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
            file: Some(file.to_string()),
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
        if c == '\n' {
            self.insert_line_break(pos);
            return;
        }
        if pos.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else if pos.y < self.len() {
            let row = self.rows.get_mut(pos.y).unwrap();
            row.insert(pos.x, c);
        }
    }
    pub fn insert_line_break(&mut self, pos: &Position) {
        if pos.y > self.len() {
            return;
        }
        if pos.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        let new_row = self.rows.get_mut(pos.y).unwrap().split(pos.x);
        self.rows.insert(pos.y + 1, new_row);
    }
    pub fn delete(&mut self, pos: &Position) {
        let len = self.len();
        if pos.y >= len {
            return;
        }
        if pos.x == self.rows.get_mut(pos.y).unwrap().len() && pos.y < len - 1 {
            let next_row = self.rows.remove(pos.y + 1);
            let row = self.rows.get_mut(pos.y).unwrap();
            row.append(next_row);
        } else {
            let row = self.rows.get_mut(pos.y).unwrap();
            row.delete(pos.x);
        }
    }
    pub fn save(&self) -> Result<(), Error> {
        if let Some(file) = &self.file {
            let mut file = fs::File::create(file)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        Ok(())
    }
}