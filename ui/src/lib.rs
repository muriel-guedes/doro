use std::{cell::RefCell, rc::Rc};

mod style;    pub use style::*;
mod cell;     pub use cell::*;
mod element;  pub use element::*;
pub mod utils;

pub struct Ui {
    pub elements: Vec<Rc<RefCell<dyn Element>>>
}
impl Ui {
    pub fn new() -> Self {
        utils::init();
        Self {
            elements: Vec::new()
        }
    }
    pub fn render(&mut self) {
        let mut stdout = std::io::stdout();
        let (w, h) = crossterm::terminal::size().unwrap();

        let mut lines = vec![vec![Cell::default();w as usize];h as usize];
        for (y, line) in lines.iter_mut().enumerate() {
            for (x, cell) in line.iter_mut().enumerate() {
                for e in self.elements.iter() {
                    let e = e.borrow();
                    e.draw(cell, x, y);
                    for c in e.children() {
                        c.borrow().draw(cell, x, y)
                    }
                }
            }
        }
        
        let content = lines.into_iter().map(
            |line|line.into_iter().map(|cell|cell.to_string()).collect::<String>()
        ).collect::<String>();
        crossterm::execute!(stdout, crossterm::cursor::MoveTo(0, 0)).unwrap();
        use std::io::Write;
        stdout.write_all(content.as_bytes()).unwrap();
        stdout.flush().unwrap()
    }
    pub fn add(&mut self, element: impl Element + 'static) -> Rc<RefCell<dyn Element>>{
        let e = Rc::new(RefCell::new(element));
        self.elements.push(e.clone());
        e
    }
}
impl Drop for Ui {
    fn drop(&mut self) {
        utils::finish()
    }
}