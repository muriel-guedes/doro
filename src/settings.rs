use std::time::Duration;

use crate::Style;

pub struct LeftBarStyle {
    pub style: Style,
    pub divider: &'static str
}

pub struct Settings {
    pub update_time: Duration,
    pub tab: &'static str,
    pub left_bar: LeftBarStyle,
    pub text: Style
}