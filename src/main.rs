use crossterm::{terminal, event::{Event, KeyEvent, self, KeyCode, KeyModifiers, KeyEventKind}};

mod buffer;
mod style;
mod settings;
mod utils;

fn main() {
    terminal::SetTitle("DORO");
    unsafe { settings::TAB_SIZE = utils::get_tab_size() }
    
    let mut buf = buffer::Buffer::new();
    loop {
        if let Event::Key(event) = event::read().unwrap() {
            match event {
                KeyEvent { modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, code, ..} => match code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => buf.insert_char(c),
                    KeyCode::Tab => buf.insert_char('\t'),
                    _ => {}
                }
                _ => {}
            }
        };
        buf.update()
    }
    
    utils::clear_screen_all();
}