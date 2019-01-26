use mock_it::Mock;

/// ```
/// use mock_it::Mock;
/// 
/// pub trait A {
///     fn foo(&self, key: i16) -> i32;
/// }
///
/// struct AMock {
///     foo: Mock<(i16), i32>
/// }
/// impl A for AMock {
///     fn foo(&self, key: i16) -> i32 {
///         self.foo.called((key))
///     }
/// }
/// impl AMock {
///     fn new() -> AMock {
///         AMock {
///             foo: Mock::new(0)
///         }
///     }
/// }
///
/// let mock = AMock::new();
/// mock.foo.given(-1).will_return(42);
/// assert_eq!(42, mock.foo(-1));
/// ```
fn doctest() {}

struct Bean();
impl Bean {
    pub fn eat(&self) {}
}
struct BeanMock{
    eat: Mock<(), ()>
}
impl BeanMock {
    pub fn new() -> Self {
        Self {
            eat: Mock::new(())
        }
    }
    pub fn eat(&self) {
        self.eat.called(())
    }
}

#[cfg(test)]
mod t {

use crate::TestSuite;
use lazy_static::lazy_static;
use mock_it::Mock;
use mock_it::Matcher;
use mock_it::Matcher::*;
use std::sync::Mutex;
use test_double::*;
#[test_double] use super::Bean;

lazy_static! {
    static ref MOCK_A_BAR: Mutex<Mock<(), u32>> = Mutex::new(Mock::new(0));
}

struct MockIt {}
impl TestSuite for MockIt {
    const NAME: &'static str = "mock-it";
    fn associated_types() {
        pub trait A {
            type Key;
            type Value;
            fn foo(&self, key: Self::Key) -> Self::Value;
        }

        struct AMock {
            foo: Mock<i16, u32>
        }
        impl A for AMock {
            type Key = i16;
            type Value = u32;
            fn foo(&self, key: i16) -> u32 {
                self.foo.called(key)
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    foo: Mock::new(0)
                }
            }
        }

        let mock = AMock::new();
        mock.foo.given(-1).will_return(42);
        assert_eq!(42, mock.foo(-1));
    }

    fn checkpoint() { unimplemented!() }
    fn consume_parameters() {
        // Mock-it can't even match parameters, much less consume them.
        unimplemented!()
    }

    fn consume_self() {
        trait A {
            fn into_u32(self) -> u32;
        }

        struct AMock {
            into_u32: Mock<(), u32>
        }
        impl A for AMock {
            fn into_u32(self) -> u32 {
                self.into_u32.called(())
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    into_u32: Mock::new(0)
                }
            }
        }

        let mock = AMock::new();
        mock.into_u32.given(()).will_return(42);
        assert_eq!(42, mock.into_u32());
    }

    fn derive() { unimplemented!() }
    fn external_trait() {
        pub trait A {
            fn foo(&self);
        }

        struct AMock {
            foo: Mock<(), ()>
        }
        impl A for AMock {
            fn foo(&self) {
                self.foo.called(())
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    foo: Mock::new(())
                }
            }
        }

        let _mock = AMock::new();
    }

    fn fallback() {
        // mock_it can't implement fallbacks.  It can do it for entire method
        // calls, but not for method calls with specific arguments, like
        // mock_derive and galvanic_mock can.
        unimplemented!()
    }

    fn foreign() { unimplemented!() }
    fn generic_method() { unimplemented!() }
    fn generic_return() { unimplemented!() }

    fn generic_struct() {
        struct GenericBean<T>(T);
        impl<T: Clone + Default> GenericBean<T> {
            pub fn eat(&self) -> T {
                T::default()
            }
        }
        struct GenericBeanMock<T> {
            eat: Mock<(), T>
        }
        impl<T: Default + Clone> GenericBeanMock<T> {
            pub fn new() -> Self {
                Self {
                    eat: Mock::new(Default::default())
                }
            }
            pub fn eat(&self) -> T {
                self.eat.called(())
            }
        }

        let mock = GenericBeanMock::<u32>::new();
        mock.eat.given(()).will_return(42u32);
        assert_eq!(42, mock.eat());
    }

    fn generic_trait() {
        // Mock-it can do generic Traits, but any type parameters used as return
        // values must support PartialEq and Default
        pub trait A<T: Default + PartialEq> {
            fn foo(&self, t: T) -> u32;
        }

        struct AMock<T: Default + PartialEq> {
            foo: Mock<T, u32>
        }
        impl<T: Default + PartialEq> A<T> for AMock<T> {
            fn foo(&self, t: T) -> u32 {
                self.foo.called(t)
            }
        }
        impl<T: Default + PartialEq> AMock<T> {
            fn new() -> AMock<T> {
                AMock {
                    foo: Mock::new(Default::default())
                }
            }
        }

        let mock: AMock<i16> = AMock::new();
        mock.foo.given(-1).will_return(42);
        assert_eq!(42, mock.foo(-1));
    }

    fn inherited_trait() {
        pub trait A {
            fn foo(&self) -> u32;
        }
        pub trait B: A {
            fn bar(&self) -> u32;
        }

        struct BMock {
            foo: Mock<(), u32>,
            bar: Mock<(), u32>,
        }
        impl A for BMock {
            fn foo(&self) -> u32 {
                self.foo.called(())
            }
        }
        impl B for BMock {
            fn bar(&self) -> u32 {
                self.bar.called(())
            }
        }
        impl BMock {
            fn new() -> BMock {
                BMock {
                    foo: Mock::new(0),
                    bar: Mock::new(0),
                }
            }
        }

        let mock = BMock::new();
        mock.foo.given(()).will_return(1);
        mock.bar.given(()).will_return(2);

        assert_eq!(1, mock.foo());
        assert_eq!(2, mock.bar());
    }

    fn many_args() {
        pub trait A {
            fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8,
                h: i8, i: i8, j: i8, k: i8, l: i8);
        }

        struct AMock {
            foo: Mock<(i8, i8, i8, i8, i8, i8, i8, i8,
                       i8, i8, i8, i8), ()>,
        }
        impl A for AMock {
            fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8,
                h: i8, i: i8, j: i8, k: i8, l: i8) {
                self.foo.called((a, b, c, d, e, f, g, h, i, j, k, l))
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    foo: Mock::new(()),
                }
            }
        }

        let mock = AMock::new();
        mock.foo.given((0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11)).will_return(());
        mock.foo(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
        print!("12 ");

    }

    fn match_combo() { unimplemented!() }
    fn match_constant() {
        pub trait A {
            fn foo(&self, key: i16);
        }
        
        struct AMock {
            foo: Mock<(i16), ()>
        }
        impl A for AMock {
            fn foo(&self, key: i16) {
                self.foo.called(key)
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    foo: Mock::new(())
                }
            }
        }
        
        let mock = AMock::new();
        mock.foo.given(-1).will_return(());
        mock.foo(-1);
    }

    fn match_method() { unimplemented!() }
    fn match_operator() { unimplemented!() }
    fn match_pattern() { unimplemented!() }
    fn match_range() { unimplemented!() }
    fn match_wildcard() {
        pub trait A {
            fn foo(&self, key: i16);
        }
        
        struct AMock {
            foo: Mock<(Matcher<i16>), ()>
        }
        impl A for AMock {
            fn foo(&self, key: i16) {
                self.foo.called(Val(key))
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    foo: Mock::new(())
                }
            }
        }
        
        let mock = AMock::new();
        mock.foo.given(Any).will_return(());
        mock.foo(-1);
    }

    fn mock_trait() {
        pub trait A {
            fn foo(&self, key: i16);
        }

        struct AMock {
            foo: Mock<(i16), ()>
        }
        impl A for AMock {
            fn foo(&self, key: i16) {
                self.foo.called(key)
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    foo: Mock::new(())
                }
            }
        }

        let mock = AMock::new();
        mock.foo.given(-1).will_return(());
        mock.foo(-1);
    }

    fn mock_struct() {
        let mock = Bean::new();
        mock.eat.given(()).will_return(());
        mock.eat();
    }

    fn modules() { unimplemented!() }
    fn multi_trait() {
        pub trait A {
            fn foo(&self) -> u32;
        }
        pub trait B {
            fn bar(&self) -> u32;
        }

        struct BMock {
            foo: Mock<(), u32>,
            bar: Mock<(), u32>,
        }
        impl A for BMock {
            fn foo(&self) -> u32 {
                self.foo.called(())
            }
        }
        impl B for BMock {
            fn bar(&self) -> u32 {
                self.bar.called(())
            }
        }
        impl BMock {
            fn new() -> BMock {
                BMock {
                    foo: Mock::new(0),
                    bar: Mock::new(0),
                }
            }
        }

        let mock = BMock::new();
        mock.foo.given(()).will_return(1);
        mock.bar.given(()).will_return(2);

        assert_eq!(1, mock.foo());
        assert_eq!(2, mock.bar());
    }

    fn return_call_with_args() { unimplemented!() }
    fn return_constant() {
        pub trait A {
            fn foo(&self) -> u32;
        }
        
        struct AMock {
            foo: Mock<(), (u32)>
        }
        impl A for AMock {
            fn foo(&self) -> u32{
                self.foo.called(())
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    foo: Mock::new(0)
                }
            }
        }
        
        let mock = AMock::new();
        mock.foo.given(()).will_return(42);
        assert_eq!(42, mock.foo());
    }

    fn return_default() {
        pub trait A {
            fn foo(&self) -> u32;
        }
        
        struct AMock {
            foo: Mock<(), (u32)>
        }
        impl A for AMock {
            fn foo(&self) -> u32{
                self.foo.called(())
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    foo: Mock::new(Default::default())
                }
            }
        }

        let mock = AMock::new();
        assert_eq!(0, mock.foo());
    }

    fn return_lifetime() {
        // Mock-it can't implement this, because Mock-it requires that return
        // types have a 'static default value, and to be Clone
        unimplemented!()
    }

    fn return_owned() {
        // Mock-it requires return types to be Clone
        unimplemented!()
    }

    fn return_panic() { unimplemented!() }
    fn return_parameters() { unimplemented!() }
    fn sequence() { unimplemented!() }

    // https://github.com/nathanielsimard/mock-it/issues/5
    fn send() {
        pub trait A {
            fn foo(&self);
        }

        struct AMock {
            foo: Mock<(), ()>
        }
        impl A for AMock {
            fn foo(&self) {
                self.foo.called(())
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    foo: Mock::new(())
                }
            }
        }


        let mock = AMock::new();
        let _ = Box::new(mock) as Box<A + Send>;
    }

    fn static_method() {
        pub trait A {
            fn bar() -> u32;
            fn foo(&self, x: u32) -> u32;
        }

        struct AMock {
            foo: Mock<(u32), (u32)>
        }
        impl A for AMock {
            fn bar() -> u32 {
                MOCK_A_BAR.lock().unwrap()
                    .called(())
            }
            fn foo(&self, x: u32) -> u32 {
                self.foo.called(x)
            }
        }
        impl AMock {
            fn new() -> AMock {
                AMock {
                    foo: Mock::new(0)
                }
            }
        }

        let mock = AMock::new();
        mock.foo.given(1).will_return(2);
        MOCK_A_BAR.lock().unwrap()
            .given(()).will_return(42);
        assert_eq!(2, mock.foo(1));
        assert_eq!(42, AMock::bar());
    }

    fn times_once() { unimplemented!() }
    fn times_any() { unimplemented!() }
    fn times_n() { unimplemented!() }
    fn times_never() { unimplemented!() }
    fn times_range() { unimplemented!() }

    fn version() {
        let ver = crate::built_info::DEPENDENCIES.iter()
            .find(|(name, _)| *name == "mock-it")
            .unwrap()
            .1;
        print!("{} ", ver);
    }
}

test!{MockIt}
}
