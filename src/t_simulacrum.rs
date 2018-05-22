/// ```
/// #![feature(proc_macro)]
/// extern crate simulacrum;
/// use simulacrum::*;
///
/// pub trait A {
///     fn foo(&self);
/// }
///
/// create_mock! {
///     impl A for AMock (self) {
///         expect_foo("foo"):
///         fn foo(&self);
///     }
/// }
///
/// let mut mock = AMock::new();
/// mock.expect_foo().called_once();
/// mock.foo();
/// ```
fn doctest() {}

#[cfg(test)]
mod t {

extern crate simulacrum;
use simulacrum::*;
use TestSuite;

struct Simulacrum {}
impl TestSuite for Simulacrum {
    fn associated_types() {
        // Traits with associated types can be mocked more easily than Generic
        // Traits.
        pub trait A {
            type Key;
            type Value;
            fn foo(&self, k: Self::Key) -> Self::Value;
        }

        create_mock_struct! {
            struct AMock: {
                expect_foo("foo") i16 => u32;
            }
        }

        impl A for AMock {
            type Key=i16;
            type Value=u32;

            fn foo(&self, k: Self::Key) -> Self::Value {
                was_called!(self, "foo", (k: i16) -> u32)
            }
        }

        let mut mock: AMock = AMock::new();
        mock.expect_foo().called_once().with(-1).returning(|_| 5);

        assert_eq!(5, mock.foo(-1));
    }

    fn checkpoint() {
        pub trait A {
            fn foo(&self);
            fn bar(&self);
            fn baz(&self);
            fn bang(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
                expect_bar("bar"):
                fn bar(&self);
                expect_baz("baz"):
                fn baz(&self);
                expect_bang("bang"):
                fn bang(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once();
        mock.expect_bar().called_once();
        mock.then().expect_baz().called_once();
        mock.expect_bang().called_once();

        mock.foo();
        mock.bar();
        mock.bang();
        mock.baz();
    }

    fn consume() {
        pub trait A {
            fn foo(self);
        }

        create_mock_struct! {
            struct AMock: {
                expect_foo("foo");
            }
        }

        impl A for AMock {
            fn foo(self) {
                was_called!(self, "foo")
            }
        }

        let mut mock = AMock::new();
        mock.expect_foo().called_once();

        mock.foo();
    }

    fn derive() {
        // Simulacrum does not yet support Deriving mocks.  That feature is
        // planned for the upcoming simulacrum_auto crate
        unimplemented!()
    }

    fn external_trait() {
        pub trait A {
            fn foo(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
            }
         }

        let _mock = AMock::new();
    }

    fn fallback() {
        // Simulacrum lacks this capability.  In some cases, it can be
        // implemented with returning and a lambda.  But that doesn't always
        // work, because returning supplies its lambda with function arguments
        // by reference instead of by value.
        unimplemented!()
    }

    fn foreign() { unimplemented!() }
    fn generic_method() {
        pub trait A {
            fn foo<T: 'static>(&self, t:T);
            fn bar<T: 'static>(&self) -> T;
        }

        create_mock_struct! {
            struct AMock: {
                expect_foo_i16("foo") i16;
                expect_foo_u32("foo") u32;
                expect_bar_i16("bar") () => i16;
                expect_bar_u32("bar") () => u32;
            }
        }

        impl A for AMock {
            fn foo<T: 'static>(&self, t: T) {
                was_called!(self, "foo", (t: T))
            }

            fn bar<T: 'static>(&self) -> T {
                was_called!(self, "bar", () -> T)
            }
        }

        let mut mock: AMock = AMock::new();
        mock.expect_foo_i16().called_once().with(-1);
        mock.expect_bar_i16().called_once().returning(|_| -5);
        mock.then().expect_foo_u32().called_once().with(1);
        mock.expect_bar_u32().called_once().returning(|_| 1_000_000);

        mock.foo::<i16>(-1);
        mock.foo::<u32>(1);
        assert_eq!(-5, mock.bar::<i16>());
        assert_eq!(1_000_000, mock.bar::<u32>());
    }

    fn generic_trait() {
        // Generic Traits can be mocked using Simulacrum's mid-level macros.
        // But the Mock struct will be concrete, not generic.
        pub trait A<T> {
            fn foo(&self, t: T) -> u32;
        }

        create_mock_struct! {
            struct AMock: {
                expect_foo("foo") i16 => u32;
            }
        }

        impl A<i16> for AMock {
            fn foo(&self, t: i16) -> u32 {
                was_called!(self, "foo", (t: i16) -> u32)
            }
        }

        //pub struct AMock<T> {
            //e: Expectations,
            //phantom: PhantomData<T>
        //}

        //impl<T: 'static> AMock<T> {
            //pub fn new() -> Self {
                //Self {
                    //e: Expectations::new(),
                    //phantom: PhantomData
                //}
            //}

            //pub fn expect_foo(&mut self) -> Method<T, u32> {
                //self.e.expect::<T, u32>("foo")
            //}
        //}

        //impl<T: 'static> A<T> for AMock<T> {
            //fn foo(&self, t: T) -> u32 {
                //self.e.was_called_returning::<T, u32>("foo", t)
            //}
        //}

        let mut mock: AMock = AMock::new();
        //let mut mock: AMock<i16> = AMock::new();
        mock.expect_foo().called_once().with(-1).returning(|_| 5);

        assert_eq!(5, mock.foo(-1));
    }

    fn inherited_trait() {
        // Simulacrum can mock inherited traits using mid-level macros
        pub trait A {
            fn foo(&self) -> u32;
        }
        pub trait B: A {
            fn bar(&self) -> u32;
        }

        create_mock_struct! {
            struct BMock: {
                expect_foo("foo") () => u32;
                expect_bar("bar") () => u32;
            }
        }
        impl A for BMock {
            fn foo(&self) -> u32 {
                was_called!(self, "foo", () -> u32)
            }
        }
        impl B for BMock {
            fn bar(&self) -> u32 {
                was_called!(self, "bar", () -> u32)
            }
         }

        let mut mock = BMock::new();
        mock.expect_foo().called_any().returning(|_| 42);
        mock.expect_bar().called_any().returning(|_| 99);

        assert_eq!(42, mock.foo());
        assert_eq!(99, mock.bar());
    }

    fn many_args() {
        // Simulacrum's params! macro works with a maximum of 9 arguments
        unimplemented!()
    }

    fn match_and() { unimplemented!() }
    fn match_constant() {
        pub trait A {
            fn foo(&self, x: u32);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: u32);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any().with(42);

        mock.foo(42);
    }

    fn match_method() { unimplemented!() }
    fn match_operator() { unimplemented!() }
    fn match_or() { unimplemented!() }
    fn match_pattern() { unimplemented!() }
    fn match_range() { unimplemented!() }
    fn match_wildcard() {
        // Matching any value is the default behavior
        pub trait A {
            fn foo(&self, x: u32);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: u32);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any();

        mock.foo(42);
    }

    fn multi_trait() {
        // Simulacrum can mock multiple traits using mid-level macros
        pub trait A {
            fn foo(&self) -> u32;
        }
        pub trait B {
            fn bar(&self) -> u32;
        }

        create_mock_struct! {
            struct BMock: {
                expect_foo("foo") () => u32;
                expect_bar("bar") () => u32;
            }
        }
        impl A for BMock {
            fn foo(&self) -> u32 {
                was_called!(self, "foo", () -> u32)
            }
        }
        impl B for BMock {
            fn bar(&self) -> u32 {
                was_called!(self, "bar", () -> u32)
            }
         }

        let mut mock = BMock::new();
        mock.expect_foo().called_any().returning(|_| 42);
        mock.expect_bar().called_any().returning(|_| 99);

        assert_eq!(42, mock.foo());
        assert_eq!(99, mock.bar());
    }

    fn return_call() {
        pub trait A {
            fn foo(&self) -> u32;
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self) -> u32;
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once().returning(|_| 5);

        assert_eq!(5, mock.foo());
    }

    fn return_call_with_args() {
        pub trait A {
            fn foo(&self, x: u32) -> u32;
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: u32) -> u32;
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once().returning(|x| x + 1);

        assert_eq!(5, mock.foo(4));
    }

    fn return_constant() {
        // Simulacrum lacks this explicit functionality, but it can be
        // implemented using `returning`
        pub trait A {
            fn foo(&self) -> u32;
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self) -> u32;
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once().returning(|_| 5);

        assert_eq!(5, mock.foo());
    }

    fn return_default() {
        // Simulacrum lacks this explicit functionality, but it can be
        // implemented using `returning`
        pub trait A {
            fn foo(&self) -> u32;
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self) -> u32;
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once().returning(|_| u32::default());

        assert_eq!(0, mock.foo());
    }

    fn return_lifetime() {
        // I can't get this code to work.
        //pub trait A<'a> {
            //fn foo(&'a self) -> &'a u32;
        //}

        //struct AMock {
            //e: Expectations
        //}

        //impl<'a> AMock {
            //pub fn new() -> Self {
                //Self {
                    //e: Expectations::new()
                //}
            //}

            //pub fn expect_foo(&'a mut self) -> Method<(), &u32> {
                //self.e.expect::<(), &u32>("foo")
            //}
        //}

        //impl<'a> A<'a> for AMock {
            //fn foo(&'a self) -> &'a u32 {
                //self.e.was_called_returning::<(), &u32>("foo", ())
            //}
        //}

        //let mut mock = AMock::new();
        //mock.expect_foo().called_any().returning(|_| &5);

        //assert_eq!(5, *mock.foo());
        unimplemented!()
    }

    fn return_owned() {
        pub trait A {
            fn foo(&self) -> String;
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self) -> String;
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once().returning(|_| "foo".to_owned());

        assert_eq!("foo", mock.foo());
    }

    fn return_panic() {
        // Simulacrum lacks this explicit functionality, but it can be
        // implemented using `modifying`
        pub trait A {
            fn foo(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once().modifying(|_| panic!("xxx"));

        mock.foo();
    }

    fn return_parameters() {
        // Simulacrum can do this, but it needs unsafe code
        pub trait A {
            fn foo(&self, x: &mut u32);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self, x: &mut u32);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any().modifying(|x: &mut *mut u32|
            unsafe {**x = 42}
        );

        let mut x = 0;
        mock.foo(&mut x);
        assert_eq!(42, x);
    }

    fn sequence() {
        // Simulacrum lacks this explicit functionality, but it can be
        // implemented using checkpoints, aka Eras.
        pub trait A {
            fn foo(&self);
            fn bar(&self);
            fn baz(&self);
            fn bang(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
                expect_bar("bar"):
                fn bar(&self);
                expect_baz("baz"):
                fn baz(&self);
                expect_bang("bang"):
                fn bang(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once();
        mock.then().expect_bar().called_once();
        mock.then().expect_baz().called_once();
        mock.then().expect_bang().called_once();

        mock.foo();
        mock.bar();
        mock.baz();
        mock.bang();
    }

    fn times_once() {
        pub trait A {
            fn foo(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_once();

        mock.foo();
    }

    fn times_any() {
        pub trait A {
            fn foo(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_any();

        mock.foo();
        mock.foo();
    }

    fn times_never() {
        pub trait A {
            fn foo(&self);
        }

        create_mock! {
            impl A for AMock (self) {
                expect_foo("foo"):
                fn foo(&self);
            }
         }

        let mut mock = AMock::new();
        mock.expect_foo().called_never();
    }

    fn times_range() { unimplemented!() }
}

test!{Simulacrum}

}
