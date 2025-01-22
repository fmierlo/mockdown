use std::any::Any;
use std::error::Error;
use std::fmt::Debug;
use std::sync::{Arc, LazyLock, Mutex};

use crate::{Mock, Mockdown};

pub const fn new() -> LazyLock<Arc<Mutex<Mockdown>>> {
    LazyLock::new(Default::default)
}

pub fn clone(mockdown: &Arc<Mutex<Mockdown>>) -> Arc<Mutex<Mockdown>> {
    Arc::clone(mockdown)
}

impl Mock for LazyLock<Arc<Mutex<Mockdown>>> {
    fn clear(&'static self) -> &'static Self {
        self.lock().unwrap().clear();
        self
    }

    fn expect<T: Any, U: Any>(&'static self, expect: fn(T) -> U) -> &'static Self {
        self.lock().unwrap().add(expect);
        self
    }

    fn mock<T: Any + Debug, U: Any>(&'static self, args: T) -> Result<U, Box<dyn Error>> {
        self.lock().unwrap().mock(args)
    }
}
