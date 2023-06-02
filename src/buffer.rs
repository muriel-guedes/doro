use crate::settings::Settings;

pub struct Buffer {
    cursor: (usize, usize),
    data: Vec<String>,
    needs_update: bool
}
impl Buffer {
    pub fn new() -> Self {
        Self {
            cursor: (0, 0),
            data: vec![String::new()],
            needs_update: true
        }
    }
    pub fn is_cursor_at_line_end(&mut self) -> bool {
        self.needs_update = true;
        self.cursor.0 >= self.data[self.cursor.1].len()
    }
    pub fn insert_char(&mut self, c: char) {
        self.needs_update = true;
        if self.is_cursor_at_line_end() {
            self.data[self.cursor.1].push(c)
        } else {
            self.data[self.cursor.1].insert(self.cursor.0, c)
        }
        self.cursor.0 += 1;
    }
    pub fn remove_char(&mut self) {
        self.needs_update = true;
        if self.cursor.0 > 0 {
            self.data[self.cursor.1].remove(self.cursor.0-1);
            self.move_cursor_left()
        } else if self.cursor.1 > 0 {
            if self.data[self.cursor.1].len() > 0 {
                self.remove_line_start()
            } else {
                self.remove_line()
            }
        }
    }
    pub fn remove_line(&mut self) {
        self.needs_update = true;
        self.data.remove(self.cursor.1);
        self.move_cursor_up()
    }
    pub fn remove_line_start(&mut self) {
        self.needs_update = true;
        let line = self.data[self.cursor.1].clone();
        self.remove_line();
        self.data[self.cursor.1].push_str(&line);
        self.move_cursor_line_end()
    }
    pub fn clamp_cursor_to_line_end(&mut self) {
        self.needs_update = true;
        if self.is_cursor_at_line_end() {
            self.cursor.0 = self.data[self.cursor.1].len()
        }
    }
    pub fn move_cursor_line_end(&mut self) {
        self.needs_update = true;
        self.cursor.0 = self.data[self.cursor.1].len()
    }
    pub fn move_cursor_line_start(&mut self) {
        self.needs_update = true;
        self.cursor.0 = 0;
        for c in self.data[self.cursor.1].chars() {
            if c == '\t' || c == ' ' {
                self.cursor.0 += 1
            } else {
                break
            }
        }
    }
    pub fn move_cursor_down(&mut self) {
        self.needs_update = true;
        self.cursor.1 += 1;
        if self.cursor.1 >= self.data.len() {
            self.data.push(String::new());
            self.cursor.0 = 0
        }
        self.clamp_cursor_to_line_end()
    }
    pub fn move_cursor_up(&mut self) {
        self.needs_update = true;
        if self.cursor.1 > 0 {
            self.cursor.1 -= 1;
        }
        self.clamp_cursor_to_line_end()
    }
    pub fn move_cursor_left(&mut self) {
        self.needs_update = true;
        if self.cursor.0 == 0 {
            self.move_cursor_up();
            self.move_cursor_line_end();
        } else {
            self.cursor.0 -= 1
        }
    }
    pub fn move_cursor_right(&mut self) {
        self.needs_update = true;
        if self.is_cursor_at_line_end() {
            self.move_cursor_down();
            self.move_cursor_line_start()
        } else {
            self.cursor.0 += 1
        }
    }
    pub fn insert_line_bellow(&mut self, line: String) {
        self.needs_update = true;
        self.cursor.1 += 1;
        self.data.insert(self.cursor.1, line);
        self.cursor.0 = 0
    }
    pub fn break_line(&mut self) {
        self.needs_update = true;
        let parts = self.data[self.cursor.1].split_at(self.cursor.0);
        let last_part = parts.1.to_string();
        self.data[self.cursor.1] = parts.0.to_string();
        self.insert_line_bellow(last_part)
    }
    pub fn get_cursor_tab_padding(&self, settings: &Settings) -> usize {
        let mut pad = 0;
        for c in self.data[self.cursor.1][0..self.cursor.0].chars() {
            if c == '\t' {
                pad += settings.tab.chars().count() - 1
            }
        }
        pad
    }
    pub fn update(&mut self, settings: &Settings) -> Option<DrawBuffer> {
        if !self.needs_update { return None }
        self.needs_update = false;
        let (w, h) = crossterm::terminal::size().unwrap();
        let left_bar_digits = self.data.len().to_string().chars().count();
        let left_bar_size = left_bar_digits + settings.left_bar.divider.len();
        let left_bar_style = settings.left_bar.style.to_string();
        let text_style = settings.text.to_string();
        let max_y = self.data.len().min(h as usize);
        let mut lines = Vec::<String>::with_capacity(max_y);
        for y in 0..max_y {
            let mut line = String::with_capacity(w as usize);
            line.push_str(&format!("{left_bar_style}{y: >left_bar_digits$}{}{text_style}", settings.left_bar.divider));
            let max_x = self.data[y].len().min(w as usize - left_bar_size);
            line.push_str(&self.data[y][0..max_x].replace('\t', settings.tab));
            lines.push(line)
        }
        Some(DrawBuffer {
            data: lines.join("\n"),
            cursor: (
                self.get_cursor_tab_padding(settings) + self.cursor.0 + left_bar_size,
                self.cursor.1
            )
        })
    }
}

pub struct DrawBuffer {
    pub data: String,
    pub cursor: (usize, usize)
}