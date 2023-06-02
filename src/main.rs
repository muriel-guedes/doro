mod buffer;
mod utils;
mod style;
mod settings;
mod run;

use std::time::Duration;

use style::{Style, Color};
use settings::{Settings, LeftBarStyle};

fn main() {
    utils::init();
    run::run(Settings {
        update_time: Duration::from_secs_f32(1. / 60.),
        tab: "    ",
        left_bar: LeftBarStyle {
            style: Style::new(Color::Black, Color::BrightBlack),
            divider: " "
        },
        text: Style::new(Color::Black, Color::White)
    });
    utils::exit();
}