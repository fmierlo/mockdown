use std::any::Any;
use std::cell::RefCell;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread::LocalKey;

use crate::{Mock, Mockdown};

pub trait MockTimes: Mock {
    fn times<E: Any + Send + Clone>(&'static self, times: u8, expect: E) -> &'static Self {
        for _ in 0..times {
            self.expect(expect.clone());
        }
        self
    }
}

impl MockTimes for RefCell<Mockdown> {}

impl MockTimes for LocalKey<RefCell<Mockdown>> {}

impl MockTimes for LazyLock<Arc<Mutex<Mockdown>>> {}
