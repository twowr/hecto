use crate::Terminal;
use crate::Document;
use crate::Row;
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
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Default for Position {
    fn default() -> Self {
        Position { x: 4, y: 0 }
    }
}
pub struct Editor {
    terminal: Terminal,
    quit: bool,
    cursor_position: Position,
    document: Document,
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
            cursor_position: Position::default(),
            document,
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
            Terminal::move_cursor(&self.cursor_position);
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
        for terminal_row in 0..visible_rows - 1 {
            Terminal::clear_current_line();
            if self.document.is_empty() && terminal_row == visible_rows/3 {
                println!("{:3}{}\r", terminal_row, self.welcome_messages());
           }
            if let Some(row) = self.document.row(terminal_row as usize)  {
                println!("{:3} {}", terminal_row, self.render_row(row));
            } else  {
               println!("{:3} \r", terminal_row);
           }
        }
    }
    pub fn render_row(&self, row: &Row) -> String {
        let start = 0;
        let end = self.terminal.size().colums as usize;
        row.render(start, end)
    }
    fn process_event(&mut self) -> Result<()> {
        if let Event::Key(keyevent) = read()? {
            if keyevent.is_ctrl('d') {
                self.quit = true;
            }
            if keyevent.is_movement() {
                self.process_cursor_movement(keyevent.code);
            }
        }
        Ok(())
    }
    fn process_cursor_movement(&mut self, key: KeyCode) {
        let Position { mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => if y < size.rows as usize {y = y.saturating_add(1);},
            KeyCode::Left => if x > 4 as usize {x = x.saturating_sub(1)},
            KeyCode::Right => if x < size.colums as usize {x = x.saturating_add(1);},
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = size.rows as usize,
            KeyCode::Home => x = 4,
            KeyCode::End => x = size.colums as usize,
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }
}
fn ded(error: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", error);
}