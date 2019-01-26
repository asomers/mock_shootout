/// ```
/// #![feature(proc_macro_hygiene)]
/// extern crate mocktopus;
/// #[macro_use] use mocktopus::macros::*;
/// use mocktopus::mocking::*;
/// 
/// #[mockable]
/// fn foo(key: i16) -> i32 {
///     0
/// }
///
/// foo.mock_safe(|key| MockResult::Return(42));
/// assert_eq!(42, foo(1));
/// ```
fn doctest() {}

#[cfg(test)]
mod t {

#[macro_use]
use mocktopus::macros::*;
use mocktopus::mocking::*;
use std::{
    cell::RefCell,
    sync::Arc
};

use {TestSuite, UniquelyOwned};

struct Mocktopus {}
#[allow(unused_parens)]
impl TestSuite for Mocktopus{
    const NAME: &'static str = "double";
    fn associated_types() { 
        // mocktopus can't mock traits
        unimplemented!()
    }

    fn checkpoint() {
        // mocktopus does not have expectations
        unimplemented!()
    }

    fn consume_parameters() {
        #[mockable]
        fn foo(_x: UniquelyOwned) {}

        let dest: Arc<RefCell<Option<UniquelyOwned>>> =
            Arc::new(RefCell::new(None));
        let dest2 = dest.clone();
        foo.mock_safe(move |uo| {
            dest2.replace(Some(uo));
            MockResult::Return(())
        });
        foo(UniquelyOwned(42));
        assert!(dest.borrow().is_some());
    }

    fn consume_self() {
        #[derive(Default)]
        struct AS {}
        #[mockable]
        impl AS {
            pub fn into_nothing(self) {}
        }

        AS::into_nothing.mock_safe(|_s| MockResult::Return(()));
        AS::default().into_nothing();
    }

    fn derive() {
        #[mockable]
        fn foo(key: i16)  -> i16 {
            key
        }

        foo.mock_safe(|_key| MockResult::Return(42));
        assert_eq!(42, foo(0));
    }

    fn external_trait() {
        // mocktopus can't mock traits, nor can it mock external structs
        unimplemented!();
    }

    fn fallback() {
        #[derive(Default)]
        struct A {};
        #[mockable]
        impl A {
            pub fn foo(&self, i: u32) -> u32 { i + 1 }
        }

        A::foo.mock_safe(|s, i| MockResult::Continue((s, 2 * i)));
        let a = A::default();
        assert_eq!(21, a.foo(10));
    }

    fn foreign() {
        // mock_safe is not implemented for extern functions
        //#[mockable]
        //extern "C" {
            //fn foo() -> u32;
        //}
        //#[mockable]
        //extern "Rust" {
            //fn bar() -> u32;
        //}

        //foo.mock_safe(|| 42);
        //bar.mock_safe(|| 43);

        //assert_eq!(42, unsafe { foo() });
        //assert_eq!(43, unsafe { bar() });
        unimplemented!();
    }

    fn generic_method() {
        #[mockable]
        fn foo<T: Clone + Default>(_t:T) -> T {T::default()}

        foo.mock_safe(|t: u32| MockResult::Return(t.clone()));
        foo.mock_safe(|t: i16| MockResult::Return(t.clone()));
        assert_eq!(42u32, foo(42u32));
        assert_eq!(-1i16, foo(-1i16));
    }

    fn generic_return() {
        #[mockable]
        fn foo<T: Default>() -> T {T::default()}

        foo::<u32>.mock_safe(|| MockResult::Return(42u32));
        assert_eq!(42u32, foo());
    }

    fn generic_struct() {
        #[derive(Default)]
        struct A<T: Clone + Default> {
            _t: T
        }
        #[mockable]
        impl<T: Clone + Default> A<T> {
            pub fn foo(_t: T) -> T { T::default() }
        }

        A::<u32>::foo.mock_safe(|t: u32| MockResult::Return(t));
        assert_eq!(42, A::<u32>::foo(42u32))
    }

    fn generic_trait() {
        // Mocktopus can't mock traits
    }

    fn inherited_trait() {
        // mocktopus can't mock traits
        unimplemented!()
    }

    fn many_args() {
        #[mockable]
        fn foo(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8,
            i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8, p: i8) -> u32 {0}

        foo.mock_safe(|_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _|
                      MockResult::Return(42));
        assert_eq!(42,
                   foo(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15));
        print!(">= 16 ");
    }

    fn match_combo() {
        // mocktopus has no expectations
        unimplemented!()
    }

    fn match_constant() { 
        // mocktopus has no expectations
        unimplemented!()
    }

    fn match_method() {
        // Mocktopus technically can match the result of a generic function, but
        // it's cumbersome.
        #[mockable]
        fn foo(key: i16) {}

        foo.mock_safe(|key| {
            assert_eq!(key, 5);
            MockResult::Return(())
        });
        foo(5)
    }

    fn match_operator() {
        // mocktopus has no expectations
        unimplemented!()
    }

    fn match_pattern() {
        // mocktopus has no expectations
        unimplemented!()
    }

    fn match_range() {
        // mocktopus has no expectations
        unimplemented!()
    }

    fn match_wildcard() {
        // mocktopus has no expectations
        unimplemented!()
    }

    fn mock_struct() {
        #[derive(Default)]
        struct Bean();
        #[mockable]
        impl Bean {
            pub fn eat(&self) -> u32 {0}
        }
        let mock = Bean::default();
        Bean::eat.mock_safe(|_s| MockResult::Return(42));
        assert_eq!(42, mock.eat());
    }

    fn mock_trait() {
        // mocktopus can only mock concrete functions
        unimplemented!()
    }

    fn modules() {
        #[mockable]
        mod a {
            pub fn foo() -> u32 {0}
            pub fn bar() -> u32 {0}
        }
        a::foo.mock_safe(|| MockResult::Return(42));
        a::bar.mock_safe(|| MockResult::Return(69));
        assert_eq!(42, a::foo());
        assert_eq!(69, a::bar());
    }

    fn multi_trait() {
        // mocktopus has no expectations
        unimplemented!()
    }

    fn return_call() { 
        unimplemented!()
    }

    fn return_call_with_args() { 
        #[mockable]
        fn foo(x: i16) -> i16 {0}

        foo.mock_safe(|x| MockResult::Return(x + 1));
        assert_eq!(foo(2), 3);
    }

    fn return_constant() { 
        unimplemented!()
    }

    fn return_default() { 
        unimplemented!()
    }

    fn return_lifetime() {
        #[derive(Default)]
        struct A {
            x: u32
        }
        #[mockable]
        impl<'a> A {
            pub fn foo(&'a self) -> &'a u32 {&self.x}
        }
        A::foo.mock_safe(|_s| MockResult::Return(&5u32));
        let a = A::default();
        assert_eq!(5, *a.foo());
    }

    // https://github.com/CodeSandwich/Mocktopus/issues/34
    fn return_owned() { 
        // mock_safe and mock_raw both take FnMut arguments
        unimplemented!()
        //#[mockable]
        //fn foo() -> UniquelyOwned {UniquelyOwned(0)}

        //let uo = UniquelyOwned(42);
        //foo.mock_safe(move || {
            //MockResult::Return(uo)
        //});
        //assert_eq!(UniquelyOwned(42), foo());
    }

    fn return_panic() {
        unimplemented!()
    }

    fn return_parameters() {
        #[mockable]
        fn foo(x: &mut u32) {*x = 0;}

        foo.mock_safe(|x| {
            *x = 42;
            MockResult::Return(())
        });

        let mut value = 1;
        foo(&mut value);
        assert_eq!(value, 42);
    }

    fn send() {
        // Mocktopus can't create mock objects at all
        unimplemented!()
    }

    fn static_method() {
        pub trait A {
            fn foo(&self, key: i16) -> i16;
            fn bar() -> u32;
        }
        #[derive(Default)]
        struct AS {}
        #[mockable]
        impl A for AS {
            fn foo(&self, key: i16)  -> i16{
                key
            }
            fn bar() -> u32 {unimplemented!()}
        }

        AS::foo.mock_safe(|_self, _key| MockResult::Return(42));
        AS::bar.mock_safe(|| MockResult::Return(42));
        let mock = AS::default();
        assert_eq!(42, mock.foo(0));
        assert_eq!(42u32, AS::bar());
    }

    fn sequence() {
        // mocktopus does not have any sequence support
        unimplemented!()
    }

    fn times_once() { 
        // mocktopus does not verify call counts
        unimplemented!()
    }

    fn times_any() {
        // mocktopus does not verify call counts
        unimplemented!()
    }

    fn times_n() {
        // mocktopus does not verify call counts
        unimplemented!()
    }

    fn times_never() {
        // mocktopus does not verify call counts
        unimplemented!()
    }

    fn times_range() { 
        // mocktopus does not verify call counts
        unimplemented!()
    }

    fn version() {
        let ver = ::built_info::DEPENDENCIES.iter()
            .find(|(name, _)| *name == "mocktopus")
            .unwrap()
            .1;
        print!("{} ", ver);
    }
}

test!{Mocktopus}

}
