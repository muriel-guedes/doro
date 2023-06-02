use std::{io::{stdout, Write}, panic};
use crossterm::{execute, terminal::{self, ClearType}, cursor};

pub fn clear_screen() {
    execute!(stdout(), cursor::Hide).unwrap();
    for y in 0..terminal::size().unwrap().1 {
        execute!(stdout(), cursor::MoveTo(0, y)).unwrap();
        execute!(stdout(), terminal::Clear(ClearType::CurrentLine)).unwrap();
    }
    execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    stdout().flush().unwrap();
}
pub fn clear_screen_all() {
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        terminal::Clear(ClearType::All),
        cursor::Hide
    ).unwrap();
    stdout().flush().unwrap();
}
pub fn show_cursor(x: u16, y: u16) {
    execute!(stdout(), cursor::MoveTo(x, y), cursor::Show).unwrap()
}

pub fn init() {
	panic::set_hook(Box::new(|panic_info| {
        exit();
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open("./doro_logs.log").unwrap()
            .write_all(panic_info.to_string().as_bytes()).unwrap();
        eprintln!("{panic_info}")
	}));
    terminal::enable_raw_mode().unwrap();
    execute!(stdout(),
        terminal::SetTitle("DORO"),
        terminal::DisableLineWrap,
        cursor::SetCursorStyle::SteadyBar
    ).unwrap();
    clear_screen_all();
}

pub fn exit() {
    clear_screen_all();
    terminal::disable_raw_mode().unwrap();
}