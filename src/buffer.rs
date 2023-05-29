use std::io::{stdout, Write};
use crossterm::execute;

use crate::utils::get_cursor_position;

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
        let cursor_pos = get_cursor_position();
        crate::utils::clear_screen();
        let (w, h) = crossterm::terminal::size().unwrap();
        let mut buf = Vec::new();
        if self.data.len() > 2 && h > 2 && w > 2 {
            for cy in 0..self.data.len().max((h-1)as usize) {
                if self.data[cy].len() > 0 {
                    for cx in 0..self.data[cy].len().max((w-1)as usize) {
                        buf.push(self.data[cy][cx] as u8);
                    }
                    buf.push(b'\n')
                }
            }
        }
        stdout().write_all(&buf).unwrap();
        stdout().flush().unwrap();
    }
}