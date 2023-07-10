use std::{rc::Rc, cell::RefCell};

use crate::Cell;

pub trait Element {
    fn draw(&self, cell: &mut Cell, _x: usize, _y: usize);
    fn children(&self) -> &Vec<Rc<RefCell<dyn Element>>>;
}