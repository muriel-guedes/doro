use std::io::{stdout, Write};
use crossterm::execute;

use crate::settings::TAB_SIZE;

pub struct Buffer {
    data: Vec<Vec<char>>
}
impl Buffer {
    pub fn new() -> Self {
        Self {
            data: vec![Vec::new()]
        }
    }
    pub fn insert_char(&mut self, c: char) {
        let (x, y) = get_cursor_position();
        if x >= self.data[y].len() {
            self.data[y].push(c)
        } else {
            self.data[y][x] = c
        }
    }
    pub fn update(&self) {
        let (x, y) = get_cursor_position();
        execute!(stdout(), crossterm::cursor::Hide).unwrap();
        crate::utils::clear_screen();
        let (w, h) = crossterm::terminal::size().unwrap();
        if w == 0 || h == 0 { return }
        let mut buf = Vec::new();
        for cy in 0..self.data.len().max((h-1)as usize) {
            for cx in 0..self.data[cy].len().max((w-1)as usize) {
                let c = self.data[cy][cx];
                if c == '\t' {
                    for _ in 0..TAB_SIZE {
                        buf.push(b' ')
                    }
                } else {
                    buf.push(c as u8)
                }
            }
            buf.push(b'\n')
        }
        let mut cx = 0;
        for i in 0..self.letter.min(self.data[self.line].len()) {
            cx += if self.data[self.line][i] == '\t' { TAB_SIZE } else { 1 }
        }
        stdout().write_all(&buf).unwrap();
        stdout().flush().unwrap();
        execute!(stdout(), crossterm::cursor::Show).unwrap();
        execute!(stdout(), crossterm::cursor::MoveTo(letter as u16, line as u16)).unwrap();
    }
}

pub fn get_cursor_position() -> (usize, usize) {
    let pos = crossterm::cursor::position().unwrap();
    (pos.0 as usize, pos.1 as usize)
}