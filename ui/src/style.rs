#[derive(Default, Copy, Clone)]
pub enum Foreground {
    #[default]
    None,
    Unset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightGray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White
}
impl Foreground {
    pub fn get(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Unset => "\x1b[0m",
            Self::Black => "\x1b[30m",
            Self::Red => "\x1b[31m",
            Self::Green => "\x1b[32m",
            Self::Yellow => "\x1b[33m",
            Self::Blue => "\x1b[34m",
            Self::Magenta => "\x1b[35m",
            Self::Cyan => "\x1b[36m",
            Self::LightGray => "\x1b[37m",
            Self::DarkGray => "\x1b[90m",
            Self::LightRed => "\x1b[91m",
            Self::LightGreen => "\x1b[92m",
            Self::LightYellow => "\x1b[93m",
            Self::LightBlue => "\x1b[94m",
            Self::LightMagenta => "\x1b[95m",
            Self::LightCyan => "\x1b[96m",
            Self::White => "\x1b[97m"
        }
    }
}

#[derive(Default, Copy, Clone)]
pub enum Background {
    #[default]
    None,
    Unset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightGray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White
}
impl Background {
    pub fn get(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Unset => "\x1b[7m",
            Self::Black => "\x1b[40m",
            Self::Red => "\x1b[41m",
            Self::Green => "\x1b[42m",
            Self::Yellow => "\x1b[43m",
            Self::Blue => "\x1b[44m",
            Self::Magenta => "\x1b[45m",
            Self::Cyan => "\x1b[46m",
            Self::LightGray => "\x1b[47m",
            Self::DarkGray => "\x1b[100m",
            Self::LightRed => "\x1b[101m",
            Self::LightGreen => "\x1b[102m",
            Self::LightYellow => "\x1b[103m",
            Self::LightBlue => "\x1b[104m",
            Self::LightMagenta => "\x1b[105m",
            Self::LightCyan => "\x1b[106m",
            Self::White => "\x1b[107m"
        }
    }
}

#[derive(Default, Copy, Clone)]
pub enum Style {
    #[default]
    None,
    Bold,
    Dark,
    Italic,
    Underlined
}
impl Style {
    pub fn get(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Bold => "\x1b[1m",
            Self::Dark => "\x1b[2m",
            Self::Italic => "\x1b[3m",
            Self::Underlined => "\x1b[4m"
        }
    }
}

#[derive(Default, Copy, Clone)]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right
}

#[derive(Default, Copy, Clone)]
pub enum VerticalAlignment {
    #[default]
    Top,
    Middle,
    Bottom
}