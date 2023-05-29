use std::io::{stdout, Write};
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
pub fn get_cursor_position() -> (usize, usize) {
    let pos = crossterm::cursor::position().unwrap();
    (pos.0 as usize, pos.1 as usize)
}
pub fn get_tab_size() -> usize {
    clear_screen_all();
    let (ix, iy) = get_cursor_position();
    print!("\t");
    stdout().flush().unwrap();
    let (fx, fy) = get_cursor_position();
    clear_screen_all();
    assert!(iy == fy && fx >= ix);
    fx - ix
}