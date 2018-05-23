/// ```
/// #![feature(proc_macro)]
/// #[macro_use] extern crate double;
/// 
/// pub trait A {
///     fn foo(&self, key: i16) -> i32;
/// }
///
/// mock_trait!(
///     MockA,
///     foo(i16) -> i32
/// );
/// impl A for MockA {
///     mock_method!(foo(&self, key: i16) -> i32);
/// }
///
/// let mock = MockA::default();
/// mock.foo.return_value(250);
/// assert_eq!(250, mock.foo(0));
/// ```
fn doctest() {}

#[cfg(test)]
mod t {

use double::matcher::*;
use {TestSuite, UniquelyOwned};

pub struct MockDouble;
impl TestSuite for MockDouble{
    fn associated_types() { unimplemented!() }
    fn checkpoint() { unimplemented!() }
    fn consume() { 
        //pub trait A {
            //fn into_i32(self) -> i32;
        //}

        //mock_trait!(
            //MockA,
            //into_i32() -> i32
        //);
        //impl A for MockA {
            //mock_method!(into_i32(self) -> i32);
        //}
        //let mock = MockA::default();
        //mock.foo.return_value(6);
        //assert_eq!(6, mock.foo());
        unimplemented!();
    }

    fn derive() { unimplemented!() }
    fn external_trait() {
        pub trait A {
            fn foo(&self, key: i16) -> i32;
        }

        mock_trait!(
            MockA,
            foo(i16) -> i32
        );
        impl A for MockA {
            mock_method!(foo(&self, key: i16) -> i32);
        }
    }
    fn fallback() { unimplemented!() }
    fn foreign() {
        // Double's documentation describe mocking free functions.  But that's
        // not the same thing as mocking foreign functions.  It's really just
        // mocking the `Fn` trait.  In particular, it only works for functions
        // that are called via `Fn` references.
        unimplemented!()
    }

    fn generic_method() {
        // Double's docs describe mocking generic methods by converting the
        // arguments to Strings.  But IMHO, that's too limited, cumbersome, and
        // lossy
        unimplemented!()
    }

    fn generic_trait() {
        //pub trait A<T> {
            //fn foo(&self, key: i16) -> T;
        //}

        //mock_trait!(
            //MockA<T>,
            //foo(i16) -> T
        //);
        //impl<T> A for MockA<T> {
            //mock_method!(foo(&self, key: i16) -> T);
        //}
        unimplemented!()
    }

    fn inherited_trait() {
        //pub trait A {
            //fn foo(&self);
        //}

        //pub trait B: A {
            //fn bar(&self);
        //}

        //mock_trait!(
            //MockB,
            //bar() -> ()
        //);

        //impl B for MockB {
            //mock_method!(bar(&self));
        //}
        unimplemented!()
    }

    fn many_args() {
        // Double is limited to 12 arguments per function
        //pub trait A {
            //fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8,
                //h: i8, i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8,
                //p: i8) -> u32;
        //}
        //mock_trait!(
            //MockA,
            //foo(i8, i8, i8, i8, i8, i8, i8, i8,
                //i8, i8, i8, i8, i8, i8, i8, i8) -> u32;
        //);
        //impl A for MockA {
            //mock_method!(foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8,
                //g: i8, h: i8, i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8,
                //p: i8) -> u32);
        //}
        //let mock = MockA::default();
        //mock.foo.return_value(6);
        //assert_eq!(3, mock.foo(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
                               //14, 15));
        unimplemented!()
    }

    fn match_and() {
        pub trait A {
            fn foo(&self, key: i16);
        }

        mock_trait!(
            MockA,
            foo(i16) -> ()
        );
        impl A for MockA {
            mock_method!(foo(&self, key: i16));
        }
        let mock = MockA::default();
        mock.foo(5);
        mock.foo.called_with_pattern(
            matcher!(
                p!(all_of,
                   vec![p!(gt, 0), p!(lt, 10)])));
    }

    fn match_constant() { 
        pub trait A {
            fn foo(&self, key: i16) -> i32;
        }

        mock_trait!(
            MockA,
            foo(i16) -> i32
        );
        impl A for MockA {
            mock_method!(foo(&self, key: i16) -> i32);
        }
        let mock = MockA::default();
        mock.foo.return_value(6);
        mock.foo(0);
        assert!(mock.foo.called_with(0i16));
    }

    fn match_method() {
        pub trait A {
            fn foo(&self, key: i16);
        }

        mock_trait!(
            MockA,
            foo(i16) -> ()
        );

        impl A for MockA {
            mock_method!(foo(&self, key: i16));
        }

        fn my_matcher(arg: &i16) -> bool {
            *arg == 15
        }

        let mock = MockA::default();
        mock.foo(15);
        assert!(mock.foo.called_with_pattern(p!(my_matcher)));
    }

    fn match_operator() {
        pub trait A {
            fn foo(&self, a:u32, b:u32, c:u32, d:u32, e:u32, f:u32);
        }

        mock_trait!(
            MockA,
            foo(u32, u32, u32, u32, u32, u32) -> ()
        );
        impl A for MockA {
            mock_method!(foo(&self, a: u32, b:u32, c:u32, d:u32, e:u32, f:u32));
        }
        let mock = MockA::default();
        mock.foo(15, 10, 10, 5, 5, 1);
        assert!(mock.foo.called_with_pattern(
            matcher!(p!(eq, 15), p!(ge, 10), p!(gt, 9), p!(le, 5), p!(lt, 6), p!(ne, 0))
        ));
    }

    fn match_or() {
        pub trait A {
            fn foo(&self, key: i16);
        }

        mock_trait!(
            MockA,
            foo(i16) -> ()
        );
        impl A for MockA {
            mock_method!(foo(&self, key: i16));
        }
        let mock = MockA::default();
        mock.foo(15);
        mock.foo.called_with_pattern(
            matcher!(
                p!(any_of,
                   vec![p!(lt, 0), p!(gt, 10)])));
    }

    fn match_pattern() { unimplemented!() }
    fn match_range() { unimplemented!() }
    fn match_wildcard() {
        pub trait A {
            fn foo(&self, key: i16);
        }

        mock_trait!(
            MockA,
            foo(i16) -> ()
        );
        impl A for MockA {
            mock_method!(foo(&self, key: i16));
        }
        let mock = MockA::default();
        mock.foo(15);
        mock.foo.called_with_pattern( matcher!( p!(any)));
    }

    fn mock_struct() { unimplemented!() }
    fn multi_trait() { unimplemented!() }
    fn return_call() { 
        unimplemented!()
    }

    fn return_call_with_args() { 
        pub trait A {
            fn foo(&self, key: i16) -> i16;
        }

        mock_trait!(
            MockA,
            foo(i16) -> i16
        );
        impl A for MockA {
            mock_method!(foo(&self, key: i16) -> i16);
        }
        let mock = MockA::default();
        mock.foo.use_closure(Box::new(|x| x + 6));
        assert_eq!(7, mock.foo(1));
    }

    fn return_constant() { 
        pub trait A {
            fn foo(&self, key: i16) -> i32;
        }

        mock_trait!(
            MockA,
            foo(i16) -> i32
        );
        impl A for MockA {
            mock_method!(foo(&self, key: i16) -> i32);
        }
        let mock = MockA::default();
        mock.foo.return_value(6);
        assert_eq!(6, mock.foo(0));
    }

    fn return_default() { 
        pub trait A {
            fn foo(&self) -> i32;
        }

        mock_trait!(
            MockA,
            foo() -> i32
        );
        impl A for MockA {
            mock_method!(foo(&self) -> i32);
        }
        let mock = MockA::default();
        assert_eq!(0, mock.foo());
    }

    fn return_lifetime() {
        //struct S();
        //pub trait A<'a> {
            //fn foo(&'a self, t: S) -> &'a S;
        //}

        //mock_trait!(
            //MockA,
            //foo(S) -> &'a S
        //);
        //impl A for MockA {
            //mock_method!(foo(&'a self, t: S) -> &'a S);
        //}
        unimplemented!()
    }

    fn return_owned() { 
        // Double requires that return types be Clone
        unimplemented!()
    }

    fn return_panic() {
        unimplemented!()
    }

    fn return_parameters() {
        // Double can't implement this, because matchers only provide
        // their arguments by immutable reference.
        unimplemented!()
    }

    fn static_method() {
        unimplemented!()
    }

    fn sequence() { unimplemented!() }
    fn times_once() { 
        pub trait A {
            fn foo(&self);
        }

        mock_trait!(
            MockA,
            foo() -> ()
        );
        impl A for MockA {
            mock_method!(foo(&self));
        }
        let mock = MockA::default();
        mock.foo();
        assert_eq!(1, mock.foo.num_calls());
    }

    fn times_any() {
        // By default, Double allows a mock to be called any number of times
        pub trait A {
            fn foo(&self);
        }

        mock_trait!(
            MockA,
            foo() -> ()
        );
        impl A for MockA {
            mock_method!(foo(&self));
        }
        let mock = MockA::default();
        mock.foo();
        mock.foo();
        mock.foo();
    }

    fn times_n() {
        pub trait A {
            fn foo(&self);
        }

        mock_trait!(
            MockA,
            foo() -> ()
        );
        impl A for MockA {
            mock_method!(foo(&self));
        }
        let mock = MockA::default();
        mock.foo();
        mock.foo();
        assert_eq!(2, mock.foo.num_calls());
    }

    fn times_never() {
        pub trait A {
            fn foo(&self);
        }

        mock_trait!(
            MockA,
            foo() -> ()
        );
        impl A for MockA {
            mock_method!(foo(&self));
        }
        let mock = MockA::default();
        assert_eq!(0, mock.foo.num_calls());
    }

    fn times_range() { 
        // Double has a different approach to validating the number of
        // calls.  It validates call counts at the end of the test,
        // rather than in Drop.  So even though it doesn't have explicit
        // support for Range, I'm going to count it, because it can be
        // implemented by the user (and the user can't implement Range
        // with other libraries).
        pub trait A {
            fn foo(&self);
        }

        mock_trait!(
            MockA,
            foo() -> ()
        );
        impl A for MockA {
            mock_method!(foo(&self));
        }
        let mock = MockA::default();
        mock.foo();
        mock.foo();
        let num_calls = mock.foo.num_calls();
        assert!(num_calls >= 2 && num_calls < 3);
    }
}

test!{MockDouble}

}
