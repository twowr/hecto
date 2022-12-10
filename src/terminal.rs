use std::io::{self, stdout, Write};
use crossterm::terminal::{
    Clear, ClearType,
};
use crossterm::execute;
use crate::Position;
#[derive(Debug)]
pub struct Size {
    pub colums: u16,
    pub rows: u16,    
}
pub struct Terminal {
    size: Size,
}
impl Terminal {
    pub fn default() -> io::Result<Self> {
        let size = crossterm::terminal::size()?;
        Ok(Self {
            size: Size { 
                colums: size.0,
                rows: size.1,
            },
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }
    pub fn clear_current_line() {
        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
    }
    #[allow(clippy::cast_possible_truncation)]
    pub fn move_cursor(position: &Position) {
        execute!(stdout(), crossterm::cursor::MoveTo(position.x as u16, position.y as u16)).unwrap();
    }
    pub fn flush() -> io::Result<()> {
        stdout().flush()
    }
    pub fn show_cursor() {
        execute!(stdout(), crossterm::cursor::Show).unwrap();
    }
    pub fn hide_cursor() {
        execute!(stdout(), crossterm::cursor::Hide).unwrap();
    }
}