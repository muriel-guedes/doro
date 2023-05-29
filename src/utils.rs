use std::io::stdout;
use crossterm::{execute, terminal::{self, ClearType}};

pub fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::UntilNewLine)).unwrap();
    execute!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
}
pub fn clear_screen_all() {
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(ClearType::All)).unwrap();
    execute!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
}