use std::any::Any;
use std::cell::RefCell;

use crate::{Mock, Mockdown};

pub fn new() -> RefCell<Mockdown> {
    Default::default()
}

impl Mock for RefCell<Mockdown> {
    fn clear(&'static self) -> &'static Self {
        self.borrow_mut().clear();
        self
    }

    fn expect<E: Any + Send>(&'static self, expect: E) -> &'static Self {
        self.borrow_mut().expect(expect);
        self
    }

    fn next<E, R, W>(&'static self, with: W) -> Result<R, String>
    where
        E: Any + Send,
        W: FnOnce(E) -> R,
    {
        self.borrow_mut().next(with)
    }
}
