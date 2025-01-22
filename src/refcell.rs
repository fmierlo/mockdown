use std::any::Any;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Debug;

use crate::{Mock, Mockdown};

impl Mock for RefCell<Mockdown> {
    fn clear(&'static self) -> &'static Self {
        self.borrow_mut().clear();
        self
    }

    fn expect<T: Any, U: Any>(&'static self, expect: fn(T) -> U) -> &'static Self {
        self.borrow_mut().add(expect);
        self
    }

    fn mock<T: Any + Debug, U: Any>(&'static self, args: T) -> Result<U, Box<dyn Error>> {
        self.borrow_mut().mock(args)
    }
}
