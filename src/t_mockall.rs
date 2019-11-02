// vim: tw=80
/// ```
/// use mockall::*;
///
/// #[automock]
/// pub trait A {
///     fn foo(&self, key: i16) -> i32;
/// }
///
/// let mut mock = MockA::new();
/// mock.expect_foo().returning(|_| 42);
/// assert_eq!(42, mock.foo(-1));
/// ```
fn doctest() {}

#[cfg(test)]
mod t {

use crate::{TestSuite, UniquelyOwned};
use mockall::{
    PredicateBooleanExt,
    Sequence,
    automock,
    mock,
    predicate::*
};
use std::{
    fmt::Debug,
    sync::{Arc, Mutex}
};

struct Holder<T1: PartialEq<u32>, T2: PartialEq<f32>>((T1, T2));

struct Mockall {}
impl TestSuite for Mockall {
    const NAME: &'static str = "mockall";

    fn associated_types() {
        #[automock(type Key=u16; type Value=i32;)]
        pub trait A {
            type Key;
            type Value;
            fn foo(&self, k: Self::Key) -> Self::Value;
        }

        let mut mock = MockA::new();
        mock.expect_foo()
            .returning(|x| i32::from(x));
        assert_eq!(4, mock.foo(4));
    }

    fn checkpoint() {
        #[automock]
        pub trait A {
            fn foo(&self, x: i16);
        }

        let mut mock = MockA::new();
        mock.expect_foo().with(eq(1));
        mock.foo(1);
        mock.checkpoint();
        mock.expect_foo().with(eq(2));
        mock.foo(2);
    }

    fn closures() {
        #[automock]
        pub trait Foo {
            fn foo<F: Fn(u32) -> u32 + 'static>(&self, f: F) -> u32;
        }

        let mut mock = MockFoo::new();
        mock.expect_foo()
            .withf(|f| f(10) == 0)
            .returning(|f| f(13));

        assert_eq!(3, mock.foo(|x| x % 5));
    }

    fn reference_parameters() {
        #[automock]
        pub trait A {
            fn foo(&self, x: &u32);
        }

        let mut mock = MockA::new();
        mock.expect_foo()
            .with(eq(1))
            .return_const(());

        mock.foo(&1);
    }

    fn consume_parameters() {
        #[automock]
        pub trait A {
            fn foo(&self, x: UniquelyOwned);
        }

        let dest: Arc<Mutex<Option<UniquelyOwned>>> =
            Arc::new(Mutex::new(None));
        let dest2 = dest.clone();

        let mut mock = MockA::new();
        mock.expect_foo().returning(move |x| {
            *dest2.lock().unwrap() = Some(x);
        });

        mock.foo(UniquelyOwned(42));
        assert!(dest.lock().unwrap().is_some());
    }

    fn consume_self() {
        #[automock]
        pub trait A {
            fn into_nothing(self);
        }

        let mut mock = MockA::new();
        mock.expect_into_nothing();
        mock.into_nothing();
    }

    fn external_trait() {
        pub trait A {
            fn foo(&self);
        }

        mock! {
            A {}
            pub trait A {
                fn foo(&self);
            }
        }

        let _mock = MockA::new();
    }

    fn foreign() {
        #[automock(mod mock_foo;)]
        extern "C" {
            pub fn foo();
        }
        #[automock(mod mock_bar;)]
        extern "Rust" {
            pub fn bar();
        }

        let foo_ctx = mock_foo::foo_context();
        let bar_ctx = mock_bar::bar_context();
        foo_ctx.expect();
        bar_ctx.expect();

        unsafe { mock_foo::foo(); }
        unsafe { mock_bar::bar(); }
    }

    fn generic_method() {
        // Mockall requires generic methods' generic parameters to be 'static
        #[automock]
        pub trait A {
            fn foo<T: 'static>(&self, t:T);
        }

        let mut mock = MockA::new();
        mock.expect_foo::<i16>().with(eq(-1));
        mock.expect_foo::<u32>().with(eq(1));

        mock.foo::<i16>(-1);
        mock.foo::<u32>(1);
    }

    fn generic_return() {
        // Mockall requires generic methods' generic parameters to be 'static
        #[automock]
        pub trait A {
            fn foo<T: 'static>(&self) -> T;
        }

        let mut mock = MockA::new();
        mock.expect_foo::<u32>().returning(|| 42);
        assert_eq!(42u32, mock.foo());
    }

    fn generic_struct() {
        mock! {
            A<T: 'static> {
                fn foo(&self, t: T) -> u32;
            }
        }

        let mut mock: MockA<i16> = MockA::new();
        mock.expect_foo().with(eq(-1));
        mock.foo(-1);
    }

    fn generic_trait() {
        #[automock]
        pub trait A<T: 'static> {
            fn foo(&self, t: T) -> u32;
        }

        let mut mock: MockA<i16> = MockA::new();
        mock.expect_foo().with(eq(-1));
        mock.foo(-1);
    }

    fn impl_trait() {
        struct Foo {}

        #[automock]
        impl Foo {
            fn foo(&self) -> impl Debug {unimplemented!();}
        }

        let mut mock = MockFoo::new();
        mock.expect_foo()
            .returning(|| Box::new(String::from("Hello, World!")));
        let r = format!("{:?}", mock.foo());
        assert_eq!("\"Hello, World!\"", r);
    }

    fn inherited_trait() {
        pub trait A {
            fn foo(&self);
        }

        pub trait B: A {
            fn bar(&self);
        }

        mock! {
            C {}
            trait A {
                fn foo(&self);
            }
            trait B: A {
                fn bar(&self);
            }
        }

        let mut mock = MockC::new();
        mock.expect_foo();
        mock.expect_bar();
        mock.foo();
        mock.bar();
     }

    fn match_method() {
        #[automock]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let mut mock = MockA::new();
        mock.expect_foo()
            .with(function(|k| *k == 5));
        mock.foo(5);
    }

    fn mock_struct() {
        struct Bean();
        #[automock]
        impl Bean {
            pub fn eat(&self) -> u32 {0}
        }

        let mut mock = MockBean::new();
        mock.expect_eat();
        mock.eat();
    }

    fn mock_trait() {
        #[automock]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let _mock = MockA::new();
    }

    fn multi_trait() {
        pub trait A {
            fn foo(&self) -> u32;
        }
        pub trait B {
            fn bar(&self) -> u32;
        }
        mock!{
            C {}
            trait A {
                fn foo(&self) -> u32;
            }
            trait B {
                fn bar(&self) -> u32;
            }
        }

        let mut mock = MockC::new();
        mock.expect_foo().returning(|| 42);
        mock.expect_bar().returning(|| 99);

        assert_eq!(42, mock.foo());
        assert_eq!(99, mock.bar());
    }

    fn return_call_with_args() {
        #[automock]
        pub trait A {
            fn foo(&self, x: i16) -> i16;
        }

        let mut mock = MockA::new();
        mock.expect_foo()
            .returning(|x| x+1);
        assert_eq!(mock.foo(2), 3);
    }

    fn return_reference() {
        #[automock]
        pub trait A {
            fn foo(&self) -> &u32;
        }

        let mut mock = MockA::new();
        let x = 5u32;
        mock.expect_foo()
            .return_const(x);
        assert_eq!(5, *mock.foo());
    }

    fn return_mutable_reference() {
        #[automock]
        pub trait A {
            fn foo(&mut self) -> &mut u32;
        }

        let mut mock = MockA::new();
        mock.expect_foo()
            .return_var(5);
        {
            let x = mock.foo();
            assert_eq!(5, *x);
            *x = 6;
        }
        {
            let y = mock.foo();
            assert_eq!(6, *y);
        }
    }

    fn return_owned() {
        #[automock]
        pub trait A {
            fn foo(&self) -> UniquelyOwned;
        }

        let mut mock = MockA::new();
        let result = UniquelyOwned(42);
        mock.expect_foo()
            .return_once(|| result);
        assert_eq!(mock.foo(), UniquelyOwned(42));
    }

    fn return_parameters() {
        mod m {
            use super::*;
            #[automock]
            pub trait A {
                fn foo(&self, x: &mut u32);
            }
        }
        use m::*;

        let mut value = 1;
        let mut mock = MockA::new();
        mock.expect_foo()
            .returning(|x| { *x = 2; });

        mock.foo(&mut value);
        assert_eq!(value, 2);
    }

    fn send() {
        #[automock]
        pub trait A {}

        let mock = MockA::new();
        let _ = Box::new(mock) as Box<dyn A + Send>;
    }

    fn static_method() {
        #[automock]
        pub trait A {
            fn foo(&self) -> u32;
            fn bar() -> u32;
        }

        let mut mock = MockA::new();
        mock.expect_foo().returning(|| 42);
        let bar_ctx = MockA::bar_context();
        bar_ctx.expect().returning(|| 99);
        assert_eq!(42, mock.foo());
        assert_eq!(99, MockA::bar());
    }

    fn times_range() {
        #[automock]
        pub trait A {
            fn foo(&self);
        }
        let mut mock = MockA::new();
        mock.expect_foo()
            .times(2..4);
        mock.foo();
        mock.foo();
    }

    fn derive() {
        #[automock]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let _mock = MockA::new();
    }

    fn fallback() {
        // Mockall does not have this functionality explicitly builtin, but it
        // can be implemented using a catch-all expectation that matches all
        // parameters
        unimplemented!()
    }

    fn match_combo() {
        #[automock]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let mut mock = MockA::new();
        mock.expect_foo()
            .with(gt(1).and(lt(10)));
        mock.expect_foo()
            .with(gt(10).or(lt(0)));
        mock.foo(5);
        mock.foo(-1);
    }

    fn match_constant() {
        #[automock]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let mut mock = MockA::new();
        mock.expect_foo()
            .with(eq(5));
        mock.foo(5);
    }

    fn match_operator() {
        #[automock]
        pub trait A {
            fn foo_eq(&self, key: i16);
            fn foo_ge(&self, key: i16);
            fn foo_gt(&self, key: i16);
            fn foo_le(&self, key: i16);
            fn foo_lt(&self, key: i16);
            fn foo_ne(&self, key: i16);
        }

        let mut mock = MockA::new();
        mock.expect_foo_eq().with(eq(3));
        mock.expect_foo_ge().with(ge(3));
        mock.expect_foo_gt().with(gt(3));
        mock.expect_foo_le().with(le(3));
        mock.expect_foo_lt().with(lt(3));
        mock.expect_foo_ne().with(ne(3));
        mock.foo_eq(3);
        mock.foo_ge(3);
        mock.foo_gt(4);
        mock.foo_le(3);
        mock.foo_lt(2);
        mock.foo_ne(5);
    }

    fn match_pattern() {
        unimplemented!()
    }

    fn match_range() {
        unimplemented!()
    }

    fn match_wildcard() {
        // This is the default behavior
        #[automock]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let mut mock = MockA::new();
        mock.expect_foo();
        mock.foo(2);
    }

    fn modules() {
        #[automock]
        mod a {
            pub fn foo() -> u32 {0}
            pub fn bar() -> u32 {0}
        }
        let foo_ctx = mock_a::foo_context();
        let bar_ctx = mock_a::bar_context();
        foo_ctx.expect().returning(|| 42);
        bar_ctx.expect().returning(|| 69);
        assert_eq!(42, mock_a::foo());
        assert_eq!(69, mock_a::bar());
    }

    fn return_constant() {
        #[automock]
        pub trait A {
            fn foo(&self) -> i16;
        }

        let mut mock = MockA::new();
        mock.expect_foo().return_const(2i16);
        assert_eq!(mock.foo(), 2);
    }

    // This is the default behavior when the nightly feature is enabled.
    fn return_default() {
        #[automock]
        pub trait A {
            fn foo(&self) -> i16;
        }

        let mut mock = MockA::new();
        mock.expect_foo().times(1);
        assert_eq!(mock.foo(), 0);
    }

    fn return_panic() { unimplemented!() }
    fn times_once() {
        #[automock]
        pub trait A {
            fn foo(&self);
        }

        let mut mock = MockA::new();
        mock.expect_foo().times(1);
        mock.foo();
    }

    // This is the default behavior for mockall, but there's also a handy helper
    // method for it in case you forget that it's the default
    fn times_any() {
        #[automock]
        pub trait A {
            fn foo(&self);
        }

        let mut mock = MockA::new();
        mock.expect_foo().times(..);
        mock.foo();
        mock.foo();
    }

    fn times_n() {
        #[automock]
        pub trait A {
            fn foo(&self);
        }

        let mut mock = MockA::new();
        mock.expect_foo().times(2);
        mock.foo();
        mock.foo();
    }

    fn times_never() {
        #[automock]
        pub trait A {
            fn foo(&self);
        }

        let mut mock = MockA::new();
        mock.expect_foo().never();
    }

    fn many_args() {
        #[automock]
        pub trait A {
            fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8,
                   h: i8, i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8,
                   p: i8);
        }

        let mut mock = MockA::new();
        mock.expect_foo();
        mock.foo(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        print!("≥ 16 ");
    }

    fn sequence() {
        #[automock]
        pub trait A {
            fn foo(&self);
            fn bar(&self);
        }

        let mut seq = Sequence::new();
        let mut mock1 = MockA::new();
        let mut mock2 = MockA::new();
        mock1.expect_foo()
            .times(1)
            .in_sequence(&mut seq);
        mock2.expect_bar()
            .times(1)
            .in_sequence(&mut seq);
        mock1.foo();
        mock2.bar();
        print!("multi object ");
    }

    fn version() {
        let ver = crate::built_info::DEPENDENCIES.iter()
            .find(|(name, _)| *name == "mockall")
            .unwrap()
            .1;
        print!("{} ", ver);
    }

    fn where_clause() {
        #[automock]
        // Mockall requires generic methods' generic parameters to be 'static
        trait Foo<T1> where T1: PartialEq<u32> + 'static {
            fn foo<T2>(&self, t1: T1, t2: T2) -> Holder<T1, T2>
                where T2: PartialEq<f32> + 'static;
        }

        let mut mock = MockFoo::<u32>::new();
        mock.expect_foo::<f32>()
            .returning(|t1, t2| Holder((t1, t2)));
        let _h = mock.foo(42, 3.14159);
    }
}

test!{Mockall}

}
