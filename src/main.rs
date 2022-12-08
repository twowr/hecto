#![warn(clippy::all, clippy::pedantic)]
mod terminal;
mod editor;
mod document;
mod row;
pub use terminal::Terminal;
use editor::Editor;
pub use editor::Position;
pub use document::Document;
pub use row::Row;

fn main() {
    Editor::default().run().unwrap();
}