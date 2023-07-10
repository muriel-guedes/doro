use std::io::Write;

use crossterm::{
    terminal::{enable_raw_mode, ClearType, EnterAlternateScreen, Clear, disable_raw_mode, LeaveAlternateScreen, DisableLineWrap},
    event::{EnableMouseCapture, self},
    execute, cursor
};

pub fn init() {
    enable_raw_mode().unwrap();
    execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
        DisableLineWrap,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
        cursor::Hide
    ).unwrap();
    std::io::stdout().flush().unwrap();
}
pub fn finish() {
    disable_raw_mode().unwrap();
    execute!(
        std::io::stdout(),
        LeaveAlternateScreen,
        event::DisableMouseCapture,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
        cursor::Show
    ).unwrap();
    std::io::stdout().flush().unwrap();
}