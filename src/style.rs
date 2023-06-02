#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite
}
impl Color {
    #[inline(always)]
    const fn is_bright(self) -> bool {
        match self {
            Self::BrightBlack | Self::BrightRed | Self::BrightGreen | Self::BrightYellow |
            Self::BrightBlue | Self::BrightMagenta | Self::BrightCyan | Self::BrightWhite
                => true,
            _ => false
        }
    }
    #[inline(always)]
    pub const fn fg(self) -> u8 {
        if self.is_bright() {
            self as u8 - Self::BrightBlack as u8 + 90
        } else {
            self as u8 + 30
        }
    }
    #[inline(always)]
    pub const fn bg(self) -> u8 {
        if self.is_bright() {
            self as u8 - Self::BrightBlack as u8 + 100
        } else {
            self as u8 + 40
        }
    }
}
#[derive(Copy, Clone)]
pub struct Style {
    pub bg: Color,
    pub fg: Color
}
impl Style {
    pub const fn new(bg: Color, fg: Color) -> Self {
        Self { bg, fg }
    }
    pub fn to_string(self) -> String {
        format!("\x1B[{};{}m", self.bg.bg(), self.fg.fg())
    }
}

#[allow(unused)]
macro_rules! write_with_style {
    ($f: expr, $style: expr, $text: expr) => {
        write!(&mut $f, "\x1B[{};{}m{}\x1b[0m", $style.bg.bg(), $style.fg.fg(), $text).unwrap()
    };
    ($f: expr, $style: expr, $text: literal) => {
        write!(&mut $f, "\x1B[{};{}m{}\x1b[0m", $style.bg.bg(), $style.fg.fg(), format!($text)).unwrap()
    };
    ($f: expr, $style: expr, $text: literal, $($arg: tt)*) => {
        write!(&mut f, "\x1B[{};{}m{}\x1b[0m", $style.bg.bg(), $style.fg.fg(), format!($text, $($arg)*))
    };
}

#[allow(unused)]
pub(crate) use write_with_style;


// bold = 1
// italic = 3