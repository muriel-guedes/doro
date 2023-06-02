use std::{sync::{Arc, Mutex}, time::Instant};
use crossterm::event::{Event, KeyEvent, self, KeyCode, KeyModifiers, KeyEventKind};

use crate::{buffer, settings::Settings, utils::{clear_screen, show_cursor}};

static mut RUNNING: bool = true;

pub fn run(settings: Settings) {
    let buf = Arc::new(Mutex::new(buffer::Buffer::new()));
    let buf1 = buf.clone();
    let t1 = std::thread::spawn(move || {
        loop {
            if !unsafe{RUNNING} { return }
            let e = event::read().unwrap();
            {
                let mut buf = buf1.lock().unwrap();
                match e {
                    Event::Key(e) => match e {
                        KeyEvent {
                            modifiers: KeyModifiers::SHIFT | KeyModifiers::NONE,
                            kind: KeyEventKind::Press,
                            code, ..
                        } => match code {
                            KeyCode::Esc => unsafe{RUNNING=false},
                            KeyCode::Char(c) => buf.insert_char(c),
                            KeyCode::Tab => buf.insert_char('\t'),
                            KeyCode::Enter => buf.break_line(),
                            KeyCode::Down => buf.move_cursor_down(),
                            KeyCode::Up => buf.move_cursor_up(),
                            KeyCode::Left => buf.move_cursor_left(),
                            KeyCode::Right => buf.move_cursor_right(),
                            KeyCode::Backspace => buf.remove_char(),
                            KeyCode::Delete => {
                                buf.move_cursor_right();
                                buf.remove_char()
                            },
                            _ => {}
                        }
                        _ => {}
                    }
                    _ => {}
                }
            }
        }
    });
    let t2 = std::thread::spawn(move || {
        let mut last_time = Instant::now();
        loop {
            if !unsafe{RUNNING} { return }
            
            let mut buf = buf.lock().unwrap();
            let res = buf.update(&settings);
            drop(buf);
            if let Some(draw_buf) = res {
                clear_screen();
                print!("{}", draw_buf.data);
                show_cursor(draw_buf.cursor.0 as u16, draw_buf.cursor.1 as u16)
            }

            let now = Instant::now();
            let elapsed_time = now - last_time;
            if settings.update_time > elapsed_time {
                std::thread::sleep(settings.update_time - elapsed_time);
            }
            last_time = now;
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
}