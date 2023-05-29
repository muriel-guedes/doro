/// Foreground
/// Bright foreground
/// Background
/// Bright background
#[derive(Copy, Clone)]
pub struct Color {
    pub fg: u8,
    pub bfg: u8,
    pub bg: u8,
    pub bbg: u8,
}
#[allow(dead_code)]
impl Color {
    pub const BLACK: Self = Self::new(30, 90, 40, 100);
    pub const RED: Self = Self::new(31, 91, 41, 101);
    pub const GREEN: Self = Self::new(32, 92, 42, 102);
    pub const YELLOW: Self = Self::new(33, 93, 43, 103);
    pub const BLUE: Self = Self::new(34, 94, 44, 104);
    pub const MAGENTA: Self = Self::new(35, 95, 45, 105);
    pub const CYAN: Self = Self::new(36, 96, 46, 106);
    pub const WHITE: Self = Self::new(37, 97, 47, 107);

    const fn new(fg: u8, bfg: u8, bg: u8, bbg: u8) -> Self {
        Self { fg, bfg, bg, bbg }
    }
}

#[allow(unused)]
macro_rules! colored {
    ($text: expr, $bg: expr, $fg: expr) => {
        format!("\x1B[{};{}m{}\x1b[0m", $bg, $fg, $text)
    };
}
#[allow(unused)]
pub(crate) use colored;