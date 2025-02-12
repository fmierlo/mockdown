use std::any::Any;
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

    fn expect<E: Any + Send>(&'static self, expect: E) -> &'static Self {
        self.lock().unwrap().expect(expect);
        self
    }

    fn next<E, R, W>(&'static self, with: W) -> Result<R, String>
    where
        E: Any + Send,
        W: FnOnce(E) -> R,
    {
        self.lock().unwrap().next(with)
    }
}
