use crate::Terminal;
use crate::Document;
use crate::Row;
use std::cmp;
use std::time::Duration;
use crossterm::terminal::enable_raw_mode;
use crossterm::event::{
    poll, read,
    Event,
    KeyModifiers, KeyCode, KeyEvent,
};
use crossterm::Result;
use std::env;
const VERSION: &str = env!("CARGO_PKG_VERSION");
#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}
pub struct Editor {
    terminal: Terminal,
    quit: bool,
    cursor_position: Position,
    document: Document,
    offset: Position,
}
trait InputType {
    fn is_ctrl(&self, key: char) -> bool;
    fn is_movement(&self) -> bool;
}
impl InputType for KeyEvent {
    fn is_ctrl(&self, key: char) -> bool {
        (self.code == KeyCode::Char(key)) && (self.modifiers == KeyModifiers::CONTROL)
    }
    fn is_movement(&self) -> bool {
        (
            self.code == KeyCode::Up
            || self.code == KeyCode::Down
            || self.code == KeyCode::Left
            || self.code == KeyCode::Right
            || self.code == KeyCode::PageUp
            || self.code == KeyCode::PageDown
            || self.code == KeyCode::Home
            || self.code == KeyCode::End
        ) && (
            self.modifiers == KeyModifiers::NONE
        )
    }
}
impl Editor {
    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        loop {
            if let Result::Err(error) = self.refresh_screen() {
                ded(error);
            }
            if self.quit {
                break;
            }
            if !poll(Duration::from_millis(100))? {
                continue;
            }
            if let Result::Err(error) = self.process_event() {
                ded(error);
            }
        }
        Ok(())
    }
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            Document::open(&args[1]).unwrap_or_default()
        } else {
            Document::default()
        };
        Self {
            terminal: Terminal::default().expect("something went wrong while initializing terminal"),
            quit: false,
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }
    fn refresh_screen(&self) -> std::io::Result<()> {
        Terminal::hide_cursor();
        Terminal::move_cursor(&Position{ x: 0, y: 0 });
        if self.quit {
            Terminal::clear_screen();
            println!("Goodbye \r");
        } else {
            self.draw_rows();
            Terminal::move_cursor(&Position { x: self.cursor_position.x.saturating_sub(self.offset.x).saturating_add(4),
                                              y: self.cursor_position.y.saturating_sub(self.offset.y), });
        }
        Terminal::show_cursor();
        Terminal::flush()
    }
    fn welcome_messages(&self) -> String {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let colums = self.terminal.size().colums as usize;                
        let len = welcome_message.len();
        let padding = colums.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("{}{}", spaces, welcome_message);
        welcome_message.truncate(colums);
        welcome_message
    }
    fn draw_rows(&self) {
        let visible_rows = self.terminal.size().rows;
        for terminal_row in 0..visible_rows {
            Terminal::clear_current_line();
            if self.document.is_empty() && terminal_row == visible_rows/2 {
                println!("{:3}{}\r", terminal_row + 1, self.welcome_messages());
            } else if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                println!("{:3} {}\r", terminal_row as usize + self.offset.y + 1, self.render_row(row));
            } else {
                println!("{:3} \r", terminal_row + 1);
            }
        }
    }
    pub fn render_row(&self, row: &Row) -> String {
        let offset = self.offset.x;
        let start = 0;
        let end = (self.terminal.size().colums as usize).saturating_sub(1);
        row.render(start + offset, end + offset)
    }
    fn process_event(&mut self) -> Result<()> {
        if let Event::Key(keyevent) = read()? {
            if keyevent.is_ctrl('q') {
                self.quit = true;
            }
            if keyevent.is_movement() {
                self.process_cursor_movement(keyevent.code);
            }
        }
        Ok(())
    }
    fn process_cursor_movement(&mut self, key: KeyCode) {
        let terminal_height = self.terminal.size().rows as usize;
        let Position { mut x, mut y } = self.cursor_position;
        let mut colums = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        let rows = self.document.len();
        //PageUp and PageDown will attempt to keep the same terminal cursor height
        //while moving by terminal_height amount of rows up or down
        //cmp::min() was used to limit cursor height range to document height through rows
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => if y < rows {y = y.saturating_add(1)},
            KeyCode::Left => {
                if x == 0 {
                     if y != y.saturating_sub(1) {
                        y = y.saturating_sub(1);
                        if let Some(row) = self.document.row(y) {x = row.len()};
                     }
                    
                } else {
                    x = x.saturating_sub(1);
                }
            },
            KeyCode::Right => {
                if x == colums {
                    if y != cmp::min(terminal_height, y.saturating_add(1)) {
                        y = cmp::min(terminal_height, y.saturating_add(1));
                        x = 0;
                    }
                    
                } else {
                    x = x.saturating_add(1);
                }
            },
            KeyCode::PageUp => if y > 0 {
                y = y.saturating_sub(terminal_height);
                self.offset.y = self.offset.y.saturating_sub(terminal_height);
            },
            KeyCode::PageDown => if y < rows {
                y = cmp::min(
                    rows.saturating_sub(terminal_height).saturating_add(y).saturating_sub(self.offset.y).saturating_add(1),
                     y.saturating_add(terminal_height)
                );
                self.offset.y = cmp::min(
                    rows.saturating_sub(terminal_height).saturating_add(1),
                    self.offset.y.saturating_add(terminal_height)
                );
            },
            KeyCode::Home => x = 0,
            KeyCode::End => x = colums,
            _ => (),
        }
        colums = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > colums {
            x = colums;
        }
        self.cursor_position = Position { x, y };
        self.scroll();
    }
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let colum = self.terminal.size().colums as usize;
        let row = self.terminal.size().rows as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(row) {
            offset.y = y.saturating_sub(row).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(colum) {
            offset.x = x.saturating_sub(colum).saturating_add(1);
        }
    }
}
fn ded(error: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", error);
}
