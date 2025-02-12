// https://github.com/fmierlo/mockdown

use std::any::Any;
use std::cell::RefCell;
use std::default::Default;
use std::marker::Send;
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
    fn expect<E: Any + Send>(&'static self, expect: E) -> &'static Self;
    fn next<E, R, W>(&'static self, with: W) -> Result<R, String>
    where
        E: Any + Send,
        W: FnOnce(E) -> R;
}

impl Mock for LocalKey<RefCell<Mockdown>> {
    fn clear(&'static self) -> &'static Self {
        self.with_borrow_mut(|mock| mock.clear());
        self
    }

    fn expect<E: Any + Send>(&'static self, expect: E) -> &'static Self {
        self.with_borrow_mut(|mock| mock.expect(expect));
        self
    }

    fn next<E, R, W>(&'static self, with: W) -> Result<R, String>
    where
        E: Any + Send,
        W: FnOnce(E) -> R,
    {
        self.with_borrow_mut(|mock| mock.next(with))
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

    pub fn expect<E: Any + Send>(&mut self, expect: E) {
        self.expects.add(expect);
    }

    pub fn next<E, R, W>(&mut self, with: W) -> Result<R, String>
    where
        E: Any + Send,
        W: FnOnce(E) -> R,
    {
        self.expects.next::<E>().map(with)
    }
}

pub mod expect {
    use std::any::Any;
    use std::collections::VecDeque;
    use std::fmt::Debug;
    use std::marker::Send;

    pub trait Expect: 'static {
        fn type_name(&self) -> &'static str;
        fn as_any(&mut self) -> Box<&mut dyn Any>;
    }

    impl Debug for dyn Expect {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.type_name())
        }
    }

    impl dyn Expect {
        pub fn downcast_mut<E: Any>(&mut self) -> Option<E> {
            self.as_any()
                .downcast_mut::<Option<E>>()
                .and_then(|value| value.take())
        }
    }

    impl<E: Any> Expect for Option<E> {
        fn type_name(&self) -> &'static str {
            std::any::type_name::<E>()
        }

        fn as_any(&mut self) -> Box<&mut dyn Any> {
            Box::new(self)
        }
    }

    #[derive(Debug, Default)]
    pub struct ExpectList {
        list: VecDeque<Box<dyn Expect>>,
    }

    impl ExpectList {
        pub fn clear(&mut self) {
            self.list.clear();
        }

        pub fn add<E: Any + Send>(&mut self, expect: E) {
            self.list.push_back(Box::new(Some(expect)));
        }

        #[allow(clippy::should_implement_trait)]
        pub fn next<E: Any>(&mut self) -> Result<E, String> {
            let mut expect = self.list.pop_front().ok_or_else(|| {
                let received = std::any::type_name::<E>();
                format!("Mockdown error, expect type mismatch: expecting nothing, received {received:?}")
            })?;

            expect.downcast_mut::<E>().ok_or_else(|| {
                self.clear();
                let expected = expect.type_name();
                let received = std::any::type_name::<E>();
                format!(
                    "Mockdown error, expect type mismatch: expecting {expected:?}, received {received:?}"
                )
            })
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
