use std::any::Any;

use crate::Mock;

pub trait MockTimes: Mock {
    fn times<T: Any, U: Any>(&'static self, times: u8, expect: fn(T) -> U) -> &'static Self {
        for _ in 0..times {
            self.expect(expect);
        }
        self
    }
}
