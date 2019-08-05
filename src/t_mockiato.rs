// vim: tw=80
/// ```
/// use mockiato::*;
///
/// #[mockable]
/// pub trait A {
///     fn foo(&self, key: i16) -> i32;
/// }
///
/// let mut mock = AMock::new();
/// mock.expect_foo(|k| k.any()).returns(42);
/// assert_eq!(42, mock.foo(-1));
/// ```
fn doctest() {}

#[cfg(test)]
mod t {

use crate::{TestSuite, UniquelyOwned};
use mockiato::*;

struct Mockiato {}
impl TestSuite for Mockiato {
    const NAME: &'static str = "mockiato";

    fn associated_types() {
        // Traits are only allowed to contain methods
        unimplemented!()
    }

    fn checkpoint() {
        unimplemented!()
    }

    fn reference_parameters() {
        #[mockable]
        pub trait A {
            fn foo(&self, x: &u32);
        }

        let mut mock = AMock::new();
        mock.expect_foo(|x| x.partial_eq_owned(42)).returns(());

        mock.foo(&42);
    }

    fn consume_parameters() {
        // Mockiato can't pass any arguments, by clone or by move, to a return
        // function, because it doesn't support return functions
        unimplemented!()
    }

    fn consume_self() {
        #[mockable]
        pub trait A {
            fn into_nothing(self);
        }

        let mut mock = AMock::new();
        mock.expect_into_nothing();
        mock.into_nothing();
    }

    fn external_trait() {
        unimplemented!()
    }

    fn foreign() {
        unimplemented!()
    }

    fn generic_method() {
        // Only lifetimes are supported as generic parameters on methods
        unimplemented!()
    }

    fn generic_return() {
        #[mockable]
        pub trait A<T> {
            fn foo(&self) -> T;
        }

        let mut mock = AMock::<u32>::new();
        mock.expect_foo().returns(42);
        assert_eq!(42u32, mock.foo());
    }

    fn generic_struct() {
        // Only traits can be made mockable
        unimplemented!()
    }

    fn generic_trait() {
        #[mockable]
        pub trait A<T> {
            fn foo(&self, t: T) -> u32;
        }

        let mut mock: AMock<i16> = AMock::new();
        mock.expect_foo(|t| t.any()).returns(42);
        assert_eq!(42, mock.foo(-1));
    }

    fn impl_trait() {
        // Only traits can be made mockable , and traits may not
        // `use -> impl Trait` syntax
        unimplemented!()
    }

    fn inherited_trait() {
        // According to the README, trait bounds are not supported
        unimplemented!()
     }

    fn match_method() {
        unimplemented!()
    }

    fn mock_struct() {
        // Only traits can be made mockable
        unimplemented!()
    }

    fn mock_trait() {
        #[mockable]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let _mock = AMock::new();
    }

    fn multi_trait() {
        // Trait bounds are not supported, and custom derive is the only way to
        // mock a trait, so there's no way for a mock to implement multiple
        // traits.
        unimplemented!()
    }

    fn return_call_with_args() {
        unimplemented!()
    }

    fn return_reference() {
        // Mockiato can only return 'static references
        unimplemented!()
    }

    fn return_mutable_reference() {
        // Mockiato can only return 'static references
        unimplemented!()
    }

    fn return_owned() {
        #[mockable]
        pub trait A {
            fn foo(&self) -> UniquelyOwned;
        }

        let mut mock = AMock::new();
        let result = UniquelyOwned(42);
        mock.expect_foo()
            .returns_once(result);
        assert_eq!(mock.foo(), UniquelyOwned(42));
    }

    fn return_parameters() {
        unimplemented!()
    }

    fn send() {
        #[mockable]
        pub trait A {}

        let mock = AMock::new();
        let _ = Box::new(mock) as Box<A + Send>;
    }

    fn static_method() {
        // The first parameter of a method must be self, so that the trait is
        // object-safe
        unimplemented!()
    }

    fn times_range() {
        #[mockable]
        pub trait A {
            fn foo(&self);
        }
        let mut mock = AMock::new();
        mock.expect_foo().times(2..4).returns(());
        mock.foo();
        mock.foo();
    }

    fn derive() {
        #[mockable]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let _mock = AMock::new();
    }

    fn fallback() {
        unimplemented!()
    }

    fn match_combo() {
        unimplemented!()
    }

    fn match_constant() {
        #[mockable]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let mut mock = AMock::new();
        mock.expect_foo(|key| key.partial_eq(5));
        mock.foo(5);
    }

    fn match_operator() {
        unimplemented!()
    }

    fn match_pattern() {
        unimplemented!()
    }

    fn match_range() {
        unimplemented!()
    }

    fn match_wildcard() {
        #[mockable]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let mut mock = AMock::new();
        mock.expect_foo(|key| key.any());
        mock.foo(2);
    }

    fn modules() {
        // Only traits can be mocked
        unimplemented!()
    }

    fn return_constant() {
        #[mockable]
        pub trait A {
            fn foo(&self) -> i16;
        }

        let mut mock = AMock::new();
        mock.expect_foo().returns(2i16);
        assert_eq!(mock.foo(), 2);
    }

    // Mockiato implemented this feature only for one type: ()
    fn return_default() {
        unimplemented!()
    }

    fn return_panic() {
        unimplemented!()
    }

    // This is the default behavior
    fn times_once() {
        #[mockable]
        pub trait A {
            fn foo(&self);
        }

        let mut mock = AMock::new();
        mock.expect_foo().times(1);
        mock.foo();
    }

    fn times_any() {
        #[mockable]
        pub trait A {
            fn foo(&self);
        }

        let mut mock = AMock::new();
        mock.expect_foo().times(..);
        mock.foo();
        mock.foo();
    }

    fn times_n() {
        #[mockable]
        pub trait A {
            fn foo(&self);
        }

        let mut mock = AMock::new();
        mock.expect_foo().times(2);
        mock.foo();
        mock.foo();
    }

    fn times_never() {
        #[mockable]
        pub trait A {
            fn foo(&self);
        }

        let mut mock = AMock::new();
        mock.expect_foo().times(0);
    }

    fn many_args() {
        #[mockable]
        pub trait A {
            fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8,
                   h: i8, i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8,
                   p: i8);
        }

        let mut mock = AMock::new();
        mock.expect_foo(|x| x.any(), |x| x.any(), |x| x.any(), |x| x.any(),
                        |x| x.any(), |x| x.any(), |x| x.any(), |x| x.any(),
                        |x| x.any(), |x| x.any(), |x| x.any(), |x| x.any(),
                        |x| x.any(), |x| x.any(), |x| x.any(), |x| x.any());
        mock.foo(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
        print!("≥ 16 ");
    }

    fn sequence() {
        #[mockable]
        pub trait A {
            fn foo(&self, x: i32);
        }

        let mut mock = AMock::new();
        mock.expect_foo(|x| x.partial_eq(42)).returns(());
        mock.expect_foo(|x| x.partial_eq(5)).returns(());
        mock.expect_foo_calls_in_order();
        mock.foo(42);
        mock.foo(5);
        print!("single method ");
    }

    fn version() {
        let ver = crate::built_info::DEPENDENCIES.iter()
            .find(|(name, _)| *name == "mockiato")
            .unwrap()
            .1;
        print!("{} ", ver);
    }

    fn where_clause() {
        // I think where clauses work but only for generic traits, not generic
        // methods.
        unimplemented!()
    }
}

test!{Mockiato}

}
