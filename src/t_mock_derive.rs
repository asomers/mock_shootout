/// ```
/// extern crate mock_derive;
/// use mock_derive::mock;
/// #[mock]
/// pub trait A {
///     fn foo(&self, key: i16) -> i32;
/// }
/// 
/// let mock = MockA::new();
/// mock.foo.first_call.set_result(42);
/// assert_eq!(42, mock.foo());
/// ```
fn doctest() {}

#[cfg(test)]
mod t {

extern crate mock_derive;
use mock_derive::mock;
use {TestSuite, UniquelyOwned};

pub struct MockDerive;
impl TestSuite for MockDerive{
    const NAME: &'static str = "mock_derive";
    fn associated_types(){unimplemented!()}
    fn checkpoint(){unimplemented!()}
    fn consume_parameters() { unimplemented!() }
    fn consume_self(){
        #[mock]
        trait A {
            fn into_u32(self) -> u32;
        }

        let mut mock = MockA::new();
        let method = mock.method_into_u32().first_call().set_result(3);
        mock.set_into_u32(method);
        assert_eq!(3, mock.into_u32());
    }

    fn derive(){
        #[mock]
        trait A {
            fn foo(&self) -> u32;
        }

        let _mock = MockA::new();
    }

    fn external_trait(){unimplemented!()}

    fn fallback() {
        #[mock]
        trait A {
            fn foo(&self) -> u32;
        }

        struct Concrete {};
        impl A for Concrete {
            fn foo(&self) -> u32 { 5 }
        }

        let mut mock = MockA::new();
        let concrete = Concrete{};
        let method = mock.method_foo().first_call().set_result(1);
        mock.set_foo(method);
        mock.set_fallback(concrete);
        assert_eq!(1, mock.foo());
        assert_eq!(5, mock.foo());
    }

    fn foreign() {
        #[mock]
        extern "C" {
            fn foo() -> i32;
        }
        #[mock]
        extern "Rust" {
            fn bar(x: f64) -> f64;
        }
        let cmock = ExternCMocks::method_foo()
            .first_call()
            .set_result(2);
        ExternCMocks::set_foo(cmock);
        let rmock = ExternRustMocks::method_bar()
            .first_call()
            .set_result(3.14);
        ExternRustMocks::set_bar(rmock);
        unsafe{ assert_eq!(foo(), 2) };
        unsafe{ assert_eq!(bar(0.0), 3.14) };
    }

    fn generic_method(){
        //#[mock]
        //trait A {
            //fn foo<T: Clone>(&self) -> T;
        //}

        //let mut mock = MockA::new();
        //let method = mock.method_foo::<u32>().first_call().set_result(3);
        //mock.set_foo(method);
        //assert_eq!(3, mock.foo::<u32>());
        unimplemented!();
    }

    fn generic_return() {unimplemented!()}
    fn generic_trait(){
        #[mock]
        trait A<T> where T: Clone {
            fn foo(&self) -> T;
        }

        let _mock = MockA::<u32>::new();
    }

    fn inherited_trait(){
        #[mock]
        trait A {
            fn foo(&self) -> u32;
        }

        #[mock]
        trait B: A {
            fn bar(&self) -> i32;
        }

        let mut mock = MockB::new();
        let method = mock.method_foo().first_call().set_result(3);
        mock.set_foo(method);
        let method = mock.method_bar().first_call().set_result(-3);
        mock.set_bar(method);
        assert_eq!(3, mock.foo());
        assert_eq!(-3, mock.bar());
    }

    fn many_args(){
         #[mock]
         pub trait A {
             fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8,
                 h: i8, i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8,
                 p: i8) -> u32;
         }
        let mut mock = MockA::new();
        let method = mock.method_foo().first_call().set_result(3);
        mock.set_foo(method);
        assert_eq!(3, mock.foo(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
                               14, 15));
        print!(">= 16 ");
    }

    fn match_combo(){unimplemented!()}
    fn match_constant(){unimplemented!()}
    fn match_method(){unimplemented!()}
    fn match_operator(){unimplemented!()}
    fn match_pattern(){unimplemented!()}
    fn match_range(){unimplemented!()}
    fn match_wildcard(){
        // mock_derive ignores arguments.  So basically it's always matching by
        // wildcard.
        #[mock]
        trait A {
            fn foo(&self, x: i16);
        }

        let mut mock = MockA::new();
        let method = mock.method_foo().set_result(());
        mock.set_foo(method);
        mock.foo(45);
    }

    fn mock_trait() {
        #[mock]
        trait A {
            fn foo(&self) -> u32;
        }

        let _mock = MockA::new();
    }
    fn mock_struct() { unimplemented!() }
    fn modules() { unimplemented!() }
    fn multi_trait(){unimplemented!()}
    fn return_call(){
        #[mock]
        trait A {
            fn foo(&self) -> u32;
        }

        let mut mock = MockA::new();
        let method = mock.method_foo()
            .return_result_of(|| 1);
        mock.set_foo(method);
        assert_eq!(1, mock.foo());
    }

    fn return_call_with_args(){
        //#[mock]
        //trait A {
            //fn foo(&self, x: u32) -> u32;
        //}

        //let mut mock = MockA::new();
        //let method = mock.method_foo()
            //.return_result_of(|x| x + 1);
        //mock.set_foo(method);
        //assert_eq!(3, mock.foo(2));
        unimplemented!()
    }

    fn return_constant(){
        #[mock]
        trait A {
            fn foo(&self) -> u32;
        }

        let mut mock = MockA::new();
        let method = mock.method_foo().first_call().set_result(3);
        mock.set_foo(method);
        assert_eq!(3, mock.foo());
    }

    fn return_default(){unimplemented!()}
    fn return_reference(){
        #[mock]
        pub trait A<'a, T> where T: 'a {
            fn foo(&self, t: T) -> &'a T;
        }
        let mut mock = MockA::<'static, f32>::new();
        static F: f32 = 1.0;
        let method = mock.method_foo()
            .set_result(&F);
        mock.set_foo(method);
        assert_eq!(1.0, *mock.foo(2.0));
    }

    fn return_owned(){
        #[mock]
        pub trait A {
            fn foo(&self) -> UniquelyOwned;
        }

        let mut mock = MockA::new();
        let result = UniquelyOwned(42);
        let method = mock.method_foo().first_call().set_result(result);
        mock.set_foo(method);
        assert_eq!(mock.foo(), UniquelyOwned(42));
    }

    fn return_panic(){
        unimplemented!()
    }

    fn return_parameters() {unimplemented!()}

    // https://github.com/DavidDeSimone/mock_derive/issues/15
    fn send() {
        unimplemented!()
    }

    fn static_method() {
        #[mock]
        pub trait A {
            fn bar() -> u32;
            fn foo(&self, x: u32) -> u32;
        }
        let mut mock = MockA::new();
        let method = mock.method_foo().return_result_of(|| 1);
        mock.set_foo(method);
        mock.foo(0);
    }

    fn sequence(){unimplemented!()}
    fn times_once(){
        #[mock]
        trait A {
            fn foo(&self) -> u32;
        }

        let mut mock = MockA::new();
        let method = mock.method_foo()
            .called_once()
            .return_result_of(|| 1);
        mock.set_foo(method);
        assert_eq!(1, mock.foo());
    }
    fn times_any(){
        #[mock]
        trait A {
            fn foo(&self) -> u32;
        }

        let mut mock = MockA::new();
        let method = mock.method_foo()
            .return_result_of(|| 1);
        mock.set_foo(method);
        assert_eq!(1, mock.foo());
        assert_eq!(1, mock.foo());
    }
    fn times_n(){unimplemented!()}
    fn times_never(){unimplemented!()}
    fn times_range(){unimplemented!()}

    fn version() {
        let ver = ::built_info::DEPENDENCIES.iter()
            .find(|(name, _)| *name == "mock_derive")
            .unwrap()
            .1;
        print!("{} ", ver);
    }
}

test!{MockDerive}
}
