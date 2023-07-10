use std::{borrow::Borrow, cell::RefCell, rc::Rc};
use ui::{Background, Cell, Element};

#[ui_macros::element]
#[derive(Default)]
pub struct BackgroundSquare {
    #[set]
    pub color: Background
}
impl Element for BackgroundSquare {
    fn draw(&self, cell: &mut Cell, _x: usize, _y: usize) {
        cell.bg = self.color
    }
    fn children(&self) -> &Vec<Rc<RefCell<dyn Element>>> {
        self.children.borrow()
    }
}