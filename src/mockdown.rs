// https://github.com/fmierlo/mockdown

use std::any::{type_name, Any};
use std::cell::RefCell;
use std::default::Default;
use std::error::Error;
use std::fmt::Debug;
use std::thread::LocalKey;

use expect::ExpectList;

thread_local! {
    static MOCKDOWN: RefCell<Mockdown> = Default::default();
}

pub fn mockdown() -> &'static LocalKey<RefCell<Mockdown>> {
    &MOCKDOWN
}

pub trait Mock {
    fn clear(&'static self) -> &'static Self;
    fn expect<T: Any, U: Any>(&'static self, expect: fn(T) -> U) -> &'static Self;
    fn mock<T: Any + Debug, U: Any>(&'static self, args: T) -> Result<U, Box<dyn Error>>;
}

impl Mock for LocalKey<RefCell<Mockdown>> {
    fn clear(&'static self) -> &'static Self {
        self.with_borrow_mut(|mock| mock.clear());
        self
    }

    fn expect<T: Any, U: Any>(&'static self, expect: fn(T) -> U) -> &'static Self {
        self.with_borrow_mut(|mock| mock.add(expect));
        self
    }

    fn mock<T: Any + Debug, U: Any>(&'static self, args: T) -> Result<U, Box<dyn Error>> {
        self.with_borrow_mut(|mock| mock.mock::<T, U>(args))
    }
}

#[derive(Default)]
pub struct Mockdown {
    expects: ExpectList,
}

impl Mockdown {
    pub fn clear(&mut self) {
        self.expects.clear();
    }

    pub fn add<T: Any, U: Any>(&mut self, expect: fn(T) -> U) {
        self.expects.add(expect);
    }

    pub fn mock<T: Any + Debug, U: Any>(&mut self, args: T) -> Result<U, Box<dyn Error>> {
        let expect = self.expects.next().ok_or_else(|| {
            self.expects.clear();
            Self::type_error::<T, U>("nothing")
        })?;

        let result = expect.mock(args).map_err(|expect| {
            self.expects.clear();
            Self::type_error::<T, U>(expect)
        })?;

        Ok(result)
    }

    pub fn type_error<T: Any + Debug, U: Any>(expect: &str) -> String {
        let received = type_name::<fn(T) -> U>();
        format!("Mockdown error, expect type mismatch: expecting {expect:?}, received {received:?}")
    }
}

pub mod expect {
    use std::any::Any;
    use std::fmt::Debug;

    pub trait IntoAny {
        fn into_any(self) -> Box<dyn Any>;
    }

    impl<T: Any> IntoAny for T {
        fn into_any(self) -> Box<dyn Any> {
            Box::new(self)
        }
    }

    pub trait IntoType {
        fn into_type<T: Any>(self, expect: &dyn Expect) -> Result<T, &'static str>;
    }

    impl IntoType for Box<dyn Any> {
        fn into_type<T: Any>(self, expect: &dyn Expect) -> Result<T, &'static str> {
            self.downcast::<T>()
                .map_err(|_| expect.type_name())
                .map(|value| *value)
        }
    }

    pub trait Expect: Send {
        fn call_mock(&self, when: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str>;
        fn type_name(&self) -> &'static str;
    }

    impl Debug for dyn Expect {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.type_name())
        }
    }

    impl dyn Expect {
        pub fn mock<T: Any, U: Any>(&self, when: T) -> Result<U, &'static str> {
            let then = self.call_mock(when.into_any())?;
            then.into_type(self)
        }
    }

    impl<T: Any, U: Any> Expect for fn(T) -> U {
        fn call_mock(&self, when: Box<dyn Any>) -> Result<Box<dyn Any>, &'static str> {
            let then = self(when.into_type(self)?);
            Ok(then.into_any())
        }

        fn type_name(&self) -> &'static str {
            std::any::type_name::<fn(T) -> U>()
        }
    }

    #[derive(Debug, Default)]
    pub struct ExpectList {
        list: Vec<Box<dyn Expect>>,
    }

    impl std::iter::Iterator for ExpectList {
        type Item = Box<dyn Expect>;

        fn next(&mut self) -> Option<Self::Item> {
            self.list.pop()
        }
    }

    impl ExpectList {
        pub fn clear(&mut self) {
            self.list.clear();
        }

        pub fn add<T: Any, U: Any>(&mut self, expect: fn(T) -> U) {
            self.list.insert(0, Box::new(expect));
        }

        pub fn is_empty(&self) -> bool {
            self.list.is_empty()
        }
    }

    impl Drop for ExpectList {
        fn drop(&mut self) {
            if !self.is_empty() {
                panic!("Mockdown error, pending expects: {:?}", self.list)
            }
        }
    }
}
