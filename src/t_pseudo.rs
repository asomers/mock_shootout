use pseudo::Mock;

/// ```
/// use pseudo::Mock;
/// 
/// pub trait A {
///     fn foo(&self, key: i16) -> i32;
/// }
///
/// struct MockA {
///     pub foo: Mock<(i16), i32>
/// }
///
/// impl A for MockA {
///     fn foo(&self, key: i16) -> i32 {
///         self.foo.call((key))
///     }
/// }
///
/// let mock = MockA{ foo: Mock::default()};
/// mock.foo.return_value(42);
/// assert_eq!(42, mock.foo(-1));
/// assert!(mock.foo.called_with((-1i16)));
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
    pub fn eat(&self) {
        self.eat.call(())
    }
}

#[cfg(test)]
mod t {

use lazy_static::lazy_static;
use pseudo::Mock;
use std::sync::Mutex;
use crate::TestSuite;
use test_double::*;
#[test_double] use super::Bean;

lazy_static! {
    static ref MOCK_A_BAR: Mutex<Mock<(), u32>> = Mutex::new(Mock::default());
}

pub struct Pseudo;
impl TestSuite for Pseudo{
    const NAME: &'static str = "pseudo";
    fn associated_types() {
        pub trait A {
            type Key;
            type Value;
            fn foo(&self, key: Self::Key) -> Self::Value;
        }

        struct MockA {
            foo: Mock<i16, u32>
        }
        impl A for MockA {
            type Key = i16;
            type Value = u32;
            fn foo(&self, key: i16) -> u32 {
                self.foo.call(key)
            }
        }

        let mock = MockA{ foo: Mock::default() };
        mock.foo.return_value(42u32);
        assert_eq!(42, mock.foo(-1));
        assert!(mock.foo.called_with(-1i16));
    }

    fn checkpoint() {
        pub trait A {
            fn foo(&self, x: i32);
        }

        struct MockA {
            foo: Mock<(i32), ()>
        }
        impl A for MockA {
            fn foo(&self, x: i32) {
                self.foo.call(x)
            }
        }

        let mock = MockA{foo: Mock::default()};
        mock.foo(1);
        mock.foo(2);
        assert!(mock.foo.called_with(1));
        assert!(mock.foo.called_with(2));
        mock.foo.reset_calls();
        mock.foo(3);
        mock.foo(4);
        assert!(mock.foo.called_with(4));
        assert!(mock.foo.called_with(3));
        assert!(!mock.foo.called_with(1));
        assert!(!mock.foo.called_with(2));
    }

    fn reference_parameters() {
        // Pseudo can't implement this, because Pseudo requires that arguments
        // types be 'static .
        unimplemented!();
        //pub trait A {
            //fn foo(&self, x: &u32);
        //}

        //struct MockA {
            //foo: Mock<(&u32), ()>
        //}
        //impl A for MockA {
            //fn foo(&self, x: &u32) {
                //self.foo.call(x)
            //}
        //}
        //let mock = MockA{ foo: Mock::default() };
        //mock.foo(&1);
        //assert!(mock.foo.called_with(&1));
    }

    fn consume_parameters() {
        // Pseudo requires parameters to be Clone
        unimplemented!()
    }

    fn consume_self() {
        trait A {
            fn into_u32(self) -> u32;
        }

        struct MockA {
            into_u32: Mock<(), u32>
        }
        impl A for MockA {
            fn into_u32(self) -> u32 {
                self.into_u32.call(())
            }
        }

        let mock = MockA{ into_u32: Mock::default() };
        mock.into_u32.return_value(42u32);
        assert_eq!(42, mock.into_u32());
    }

    fn derive() { unimplemented!() }
    fn external_trait() {
        pub trait A {
            fn foo(&self);
        }

        struct MockA {
            foo: Mock<(), ()>
        }
        impl A for MockA {
            fn foo(&self) {
                self.foo.call(())
            }
        }

        let _mock = MockA{foo: Mock::default()};
    }

    fn fallback() { unimplemented!() }
    fn foreign() { unimplemented!() }
    fn generic_method() { unimplemented!() }
    fn generic_return() { unimplemented!() }
    fn generic_struct() {
        struct A<T: Clone>(T);
        impl<T: Clone> A<T> {
            fn foo(&self, _t: T) -> u32 {
                unimplemented!()
            }
        }
        struct MockA<T: Clone> {
            foo: Mock<T, u32>
        }
        impl<T: Clone> MockA<T> {
            fn foo(&self, t: T) -> u32 {
                self.foo.call(t)
            }
        }

        let mock: MockA<i16> = MockA{ foo: Mock::default() };
        mock.foo.return_value(42u32);
        assert_eq!(42, mock.foo(-1));
        assert!(mock.foo.called_with(-1i16));
    }

    fn generic_trait() {
        // Pseudo can do generic traits, but any type parameters must be Clone
        pub trait A<T: Clone> {
            fn foo(&self, t: T) -> u32;
        }

        struct MockA<T: Clone> {
            foo: Mock<T, u32>
        }
        impl<T: Clone> A<T> for MockA<T> {
            fn foo(&self, t: T) -> u32 {
                self.foo.call(t)
            }
        }

        let mock: MockA<i16> = MockA{ foo: Mock::default() };
        mock.foo.return_value(42u32);
        assert_eq!(42, mock.foo(-1));
        assert!(mock.foo.called_with(-1i16));
    }

    // Can't derive mocks for structs
    fn impl_trait() {unimplemented!() }

    fn inherited_trait() {
        pub trait A {
            fn foo(&self) -> u32;
        }
        pub trait B: A {
            fn bar(&self) -> u32;
        }

        struct MockB {
            foo: Mock<(), u32>,
            bar: Mock<(), u32>,
        }
        impl A for MockB {
            fn foo(&self) -> u32 {
                self.foo.call(())
            }
        }
        impl B for MockB {
            fn bar(&self) -> u32 {
                self.bar.call(())
            }
        }

        let mock = MockB{foo: Mock::default(), bar: Mock::default()};
        mock.foo.return_value(1u32);
        mock.bar.return_value(2u32);

        assert_eq!(1, mock.foo());
        assert_eq!(2, mock.bar());
    }

    fn many_args() {
        pub trait A {
            fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8,
                h: i8, i: i8, j: i8, k: i8, l: i8);
        }

        struct MockA {
            foo: Mock<(i8, i8, i8, i8, i8, i8, i8, i8,
                       i8, i8, i8, i8), ()>,
        }
        impl A for MockA {
            fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8,
                h: i8, i: i8, j: i8, k: i8, l: i8) {
                self.foo.call((a, b, c, d, e, f, g, h, i, j, k, l))
            }
        }

        let mock = MockA{foo: Mock::default()};
        mock.foo(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
        assert!(mock.foo.called_with((0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11)));
        print!("12 ");
    }

    fn match_combo() { unimplemented!() }
    fn match_constant() {
        pub trait A {
            fn foo(&self, key: i16);
        }
        
        struct MockA {
            foo: Mock<(i16), ()>
        }
        impl A for MockA {
            fn foo(&self, key: i16) {
                self.foo.call(key)
            }
        }
        
        let mock = MockA{ foo: Mock::default() };
        mock.foo(-1);
        assert!(mock.foo.called_with(-1i16));
    }

    fn match_method() {
        // This is awkward, but doable
        pub trait A {
            fn foo(&self, x: u32);
        }

        struct MockA {
            foo: Mock<(u32), ()>
        }
        impl A for MockA {
            fn foo(&self, x: u32) {
                self.foo.call(x)
            }
        }

        let mock = MockA{foo: Mock::default()};

        mock.foo(42);
        let matcher = |x: u32| x == 42;
        assert!(matcher(mock.foo.calls()[0]));
    }

    fn match_operator() { unimplemented!() }
    fn match_pattern() { unimplemented!() }
    fn match_range() { unimplemented!() }
    fn match_wildcard() {
        // This is the default behavior
        pub trait A {
            fn foo(&self, key: i16);
        }
        
        struct MockA {
            foo: Mock<(i16), ()>
        }
        impl A for MockA {
            fn foo(&self, key: i16) {
                self.foo.call(key)
            }
        }
        
        let mock = MockA{ foo: Mock::default()};
        mock.foo(-12352);
    }

    fn mock_struct() {
        let mock = Bean{eat: Mock::default()};
        mock.eat();
    }

    fn modules() { unimplemented!() }
    fn mock_trait() {
        pub trait A {
            fn foo(&self) -> u32;
        }

        struct MockA {
            foo: Mock<(), u32>,
        }
        impl A for MockA {
            fn foo(&self) -> u32 {
                self.foo.call(())
            }
        }

        let mock = MockA{foo: Mock::default()};
        mock.foo.return_value(1u32);

        assert_eq!(1, mock.foo());
    }

    fn multi_trait() {
        pub trait A {
            fn foo(&self) -> u32;
        }
        pub trait B {
            fn bar(&self) -> u32;
        }

        struct MockB {
            foo: Mock<(), u32>,
            bar: Mock<(), u32>,
        }
        impl A for MockB {
            fn foo(&self) -> u32 {
                self.foo.call(())
            }
        }
        impl B for MockB {
            fn bar(&self) -> u32 {
                self.bar.call(())
            }
        }

        let mock = MockB{foo: Mock::default(), bar: Mock::default()};
        mock.foo.return_value(1u32);
        mock.bar.return_value(2u32);

        assert_eq!(1, mock.foo());
        assert_eq!(2, mock.bar());
    }

    fn return_call_with_args() {
        pub trait A {
            fn foo(&self, x: u32) -> u32;
        }

        struct MockA {
            foo: Mock<(u32), u32>
        }
        impl A for MockA {
            fn foo(&self, x: u32) -> u32 {
                self.foo.call(x)
            }
        }

        let mock = MockA{foo: Mock::default()};
        mock.foo.use_closure(Box::new(|x| x + 1));

        assert_eq!(5, mock.foo(4));
    }

    fn return_constant() {
        trait A {
            fn foo(&self) -> u32;
        }

        struct MockA {
            foo: Mock<(), u32>
        }
        impl A for MockA {
            fn foo(&self) -> u32 {
                self.foo.call(())
            }
        }

        let mock = MockA{ foo: Mock::default() };
        mock.foo.return_value(42u32);
        assert_eq!(42, mock.foo());
    }

    fn return_default() {
        pub trait A {
            fn foo(&self) -> u32;
        }
        
        struct MockA {
            foo: Mock<(), (u32)>
        }
        impl A for MockA {
            fn foo(&self) -> u32{
                self.foo.call(())
            }
        }

        let mock = MockA{foo: Mock::default()};
        assert_eq!(0, mock.foo());
    }

    fn return_reference() {
        // Pseudo can't implement this, because Pseudo requires that return
        // types be Clone
        unimplemented!()
    }

    fn return_mutable_reference() { unimplemented!() }
    fn return_owned() {
        // Mock-it requires return types to be Clone
        unimplemented!()
    }

    fn return_panic() { unimplemented!() }
    fn return_parameters() { unimplemented!() }
    fn sequence() {
        pub trait A {
            fn foo(&self, x: i32);
        }

        struct MockA {
            foo: Mock<(i32), ()>
        }
        impl A for MockA {
            fn foo(&self, x: i32) {
                self.foo.call(x)
            }
        }

        let mock = MockA{foo: Mock::default()};
        mock.foo(1);
        mock.foo(2);
        assert_eq!(mock.foo.calls().as_slice(), [1, 2]);
        print!("single method ");
    }

    // https://github.com/iredelmeier/pseudo/issues/1
    fn send() {
        pub trait A {
            fn foo(&self);
        }

        struct MockA {
            foo: Mock<(), ()>
        }
        impl A for MockA {
            fn foo(&self) {
                self.foo.call(())
            }
        }

        let mock = MockA{foo: Mock::default()};
        let _ = Box::new(mock) as Box<dyn A + Send>;
    }

    // Pseudo can do this, but you must manually create a global Mock object for
    // each static method.
    fn static_method() {
        pub trait A {
            fn bar() -> u32;
            fn foo(&self, x: u32) -> u32;
        }

        #[derive(Default)]
        struct MockA {
            foo: Mock<(u32), (u32)>
        }
        impl A for MockA {
            fn bar() -> u32 {
                MOCK_A_BAR.lock().unwrap().call(())
            }
            fn foo(&self, x: u32) -> u32 {
                self.foo.call(x)
            }
        }

        let mock = MockA::default();
        mock.foo.return_value(2u32);
        MOCK_A_BAR.lock().unwrap().return_value(3u32);
        assert_eq!(2, mock.foo(1));
        assert_eq!(3, MockA::bar());
    }

    fn times_once() { unimplemented!() }
    fn times_any() { unimplemented!() }
    fn times_n() {
        pub trait A {
            fn foo(&self);
        }

        struct MockA {
            foo: Mock<(), ()>
        }
        impl A for MockA {
            fn foo(&self) {
                self.foo.call(())
            }
        }

        let mock = MockA{foo: Mock::default()};
        mock.foo();
        mock.foo();
        assert_eq!(2, mock.foo.num_calls());
    }

    fn times_never() {
        pub trait A {
            fn foo(&self);
        }

        struct MockA {
            foo: Mock<(), ()>
        }
        impl A for MockA {
            fn foo(&self) {
                self.foo.call(())
            }
        }

        let mock = MockA{foo: Mock::default()};
        assert!(!mock.foo.called());
    }

    fn times_range() { unimplemented!() }

    fn version() {
        let ver = crate::built_info::DEPENDENCIES.iter()
            .find(|(name, _)| *name == "pseudo")
            .unwrap()
            .1;
        print!("{} ", ver);
    }

    // Pseudo can't mock generic methods
    fn where_clause() { unimplemented!() }
}

test!{Pseudo}

}
