use crate::{Style, Background, Foreground};

pub type Line = Vec<Cell>;
pub type Lines = Vec<Line>;

#[derive(Clone)]
pub struct Cell {
    pub style: Style,
    pub bg: Background,
    pub fg: Foreground,
    pub text: String,
    pub z: i16
}
impl Default for Cell {
    fn default() -> Self {
        Self {
            style: Style::None,
            bg: Background::Black,
            fg: Foreground::White,
            text: " ".to_string(),
            z: 0
        }
    }
}
impl ToString for Cell {
    fn to_string(&self) -> String {
        vec![
            self.style.get(),
            self.bg.get(),
            self.fg.get(),
            &self.text
        ].join("")
    }
}