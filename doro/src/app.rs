use std::time::Duration;
use crossterm::event::{Event, self, KeyEvent, KeyCode, KeyModifiers};
use ui::Ui;

pub struct App {
    pub ui: Ui
}
impl App {
    pub fn new() -> Self {
        std::panic::set_hook(Box::new(|e| {
            println!("{e}");
            std::fs::write("doro_panic.log", e.to_string()).unwrap()
        }));
        Self {
            ui: Ui::new()
        }
    }
    pub fn update(&mut self, _event: Option<Event>) {
        self.ui.render()
    }
    pub fn start(mut self) {
        loop {
            self.update(
                if event::poll(Duration::from_secs_f32(1./10.)).unwrap() {
                    match event::read().unwrap() {
                        Event::Key(KeyEvent { code: KeyCode::Char('\''), modifiers: KeyModifiers::ALT, .. }) => break,
                        event => Some(event)
                    }
                } else { None }
            )
        }
    }
}