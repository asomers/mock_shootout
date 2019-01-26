/// ```
/// use mockers::*;
/// use mockers_derive::mocked;
/// #[mocked]
/// pub trait A {
///     fn foo(&self, key: i16) -> i32;
/// }
/// 
/// let scenario = Scenario::new();
/// let mock = scenario.create_mock_for::<A>();
/// scenario.expect(mock.foo_call(-1).and_return(42));
/// assert_eq!(42, mock.foo(-1));
/// ```
fn doctest() {}

#[cfg(test)]
mod t {

use mockers::*;
use mockers_derive::*;
use std::{sync::Arc, cell::RefCell};
use crate::{TestSuite, UniquelyOwned};

pub trait ET {}

mock!{
    ETMock,
    self,
    trait ET {}
}

pub trait EITA {}
pub trait EITB: EITA {}

mock!{
    EITBMock,
    self,
    trait EITA {},
    self,
    trait EITB {}
}

pub trait EMTA {}
pub trait EMTB {}

mock!{
    EMTBMock,
    self,
    trait EMTA {},
    self,
    trait EMTB {}
}

struct Mockers {}
impl TestSuite for Mockers {
    const NAME: &'static str = "mockers";
    fn associated_types(){
        #[mocked]
        pub trait A {
            type Key;
            type Value;
            fn foo(&self, x: Self::Key) -> Self::Value;
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A<Key=u32, Value=bool>>();
        scenario.expect(mock.foo_call(1).and_return(false));
        assert_eq!(mock.foo(1), false);
    }

    fn checkpoint(){
        #[mocked]
        pub trait A {
            fn foo(&self, x: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(1).and_return(()));
        mock.foo(1);
        scenario.checkpoint();
        scenario.expect(mock.foo_call(2).and_return(()));
        mock.foo(2);
    }

    fn consume_parameters() {
        #[mocked]
        pub trait A {
            fn foo(&self, x: UniquelyOwned);
        }

        let dest: Arc<RefCell<Option<UniquelyOwned>>> =
            Arc::new(RefCell::new(None));
        let dest2 = dest.clone();
        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(matchers::ANY).and_call(move |x| {
            dest2.replace(Some(x));
        }));

        mock.foo(UniquelyOwned(42));
        assert!(dest.borrow().is_some());
    }

    fn consume_self(){
        #[mocked]
        pub trait A {
            fn into_nothing(self);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.into_nothing_call().and_return(()));
        mock.into_nothing();
    }

    fn derive(){
        #[mocked]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let _mock = scenario.create_mock_for::<A>();
    }

    fn external_trait(){
        let scenario = Scenario::new();
        let _mock = scenario.create_mock::<ETMock>();
    }

    fn fallback() {
        // Mockers does not have this functionality explicitly builtin, but it
        // can be implemented using a catch-all expectation that matches all parameters
        unimplemented!()
    }

    fn foreign() {
        #[mocked(LibFoo)]
        extern "C" {
            fn foo();
        }
        #[mocked(LibBar)]
        extern "Rust" {
            fn bar();
        }

        let scenario = Scenario::new();
        let foo_mock = scenario.create_mock::<LibFoo>();
        let bar_mock = scenario.create_mock::<LibBar>();

        scenario.expect(foo_mock.foo_call().and_return(()));
        scenario.expect(bar_mock.bar_call().and_return(()));

        unsafe { foo(); }
        unsafe { bar(); }
    }

    // https://github.com/kriomant/mockers/issues/39
    fn generic_method(){
        #[mocked]
        pub trait A {
            fn foo<T>(&self, t:T) -> u32;
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock::<AMock>();
        scenario.expect(mock.foo_call(42u32).and_return_clone(100u32).times(..));
        scenario.expect(mock.foo_call(42i16).and_return_clone(1u32).times(..));
        assert_eq!(1u32, mock.foo(42i16));
        assert_eq!(100u32, mock.foo(42u32));
    }

    fn generic_return() {
        #[mocked]
        pub trait A {
            fn foo<T: 'static>(&self) -> T;
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock::<AMock>();
        scenario.expect(mock.foo_call().and_return(42u32));
        assert_eq!(42u32, mock.foo());
    }

    fn generic_struct(){
        unimplemented!();
    }
    fn generic_trait(){
        unimplemented!();
    }

    fn inherited_trait(){
        #[mocked(module="::t_mockers::t")]
        pub trait A {
            fn foo(&self);
        }
       
        #[mocked(refs="A => ::t_mockers::t::A")]
        pub trait B: A {
            fn bar(&self);
        }
       
        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<B>();
        scenario.expect(mock.foo_call().and_return(()));
        scenario.expect(mock.bar_call().and_return(()));
        mock.foo();
        mock.bar();
    }

    fn many_args(){
         #[mocked]
         pub trait A {
             fn foo(&self, a: i8, b: i8, c: i8, d: i8);
         }

         let scenario = Scenario::new();
         let mock = scenario.create_mock_for::<A>();
         scenario.expect(mock.foo_call(0, 1, 2, 3).and_return(()));
         mock.foo(0, 1, 2, 3);
         print!("4 ");
    }

    fn match_combo(){
        #[mocked]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(
            matchers::and(matchers::gt(1), matchers::lt(10))).and_return(()));
        scenario.expect(mock.foo_call(
            matchers::or(matchers::gt(10), matchers::lt(0))).and_return(()));
        mock.foo(5);
        mock.foo(-1);
    }

    fn match_constant(){
        #[mocked]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(5).and_return(()));
        mock.foo(5);
    }

    fn match_method(){
        #[mocked]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(
            matchers::check(|t: &i16| *t == 5)).and_return(()));
        mock.foo(5);
    }

    fn match_operator(){
        #[mocked]
        pub trait A {
            fn foo_eq(&self, key: i16);
            fn foo_ge(&self, key: i16);
            fn foo_gt(&self, key: i16);
            fn foo_le(&self, key: i16);
            fn foo_lt(&self, key: i16);
            fn foo_ne(&self, key: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_eq_call(matchers::eq(3)).and_return(()));
        scenario.expect(mock.foo_ge_call(matchers::ge(3)).and_return(()));
        scenario.expect(mock.foo_gt_call(matchers::gt(3)).and_return(()));
        scenario.expect(mock.foo_le_call(matchers::le(3)).and_return(()));
        scenario.expect(mock.foo_lt_call(matchers::lt(3)).and_return(()));
        scenario.expect(mock.foo_ne_call(matchers::ne(3)).and_return(()));
        mock.foo_eq(3);
        mock.foo_ge(3);
        mock.foo_gt(4);
        mock.foo_le(3);
        mock.foo_lt(2);
        mock.foo_ne(5);
    }

    fn match_pattern(){
        #[mocked]
        pub trait A {
            fn foo(&self, key: Option<i16>);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(arg!(Some(_))).and_return(()));
        mock.foo(Some(1));
    }

    fn match_range(){
        #[mocked]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(
            matchers::in_range(5..10)).and_return(()));
        mock.foo(5);
    }

    fn match_wildcard(){
        #[mocked]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(matchers::ANY).and_return(()));
        mock.foo(2);
    }

    fn mock_struct() { unimplemented!() }
    fn modules() { unimplemented!() }

    fn mock_trait() {
        #[mocked]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let _mock = scenario.create_mock_for::<A>();
    }

    fn multi_trait(){
        fn foo<T: EMTA + EMTB>(_t: T) {
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock::<EMTBMock>();
        foo(mock);
    }

    fn return_call(){
        unimplemented!()
    }

    fn return_call_with_args(){
        #[mocked]
        pub trait A {
            fn foo(&self, x: i16) -> i16;
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(matchers::ANY).and_call(|x| x+1));
        assert_eq!(mock.foo(2), 3);
    }

    fn return_constant(){
        #[mocked]
        pub trait A {
            fn foo(&self) -> i16;
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_return(2));
        assert_eq!(mock.foo(), 2);
    }

    fn return_default(){
        #[mocked]
        pub trait A {
            fn foo(&self) -> i16;
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_return_default().times(1));
        assert_eq!(mock.foo(), 0);
    }

    fn return_lifetime(){
        //#[mocked]
        //pub trait A {
            //fn foo(&'a self) -> &'a A;
        //}

        //let scenario = Scenario::new();
        //let _mock = scenario.create_mock_for::<A>();
        unimplemented!()
    }

    fn return_owned(){
        #[mocked]
        pub trait A {
            fn foo(&self) -> UniquelyOwned;
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        let result = UniquelyOwned(42);
        scenario.expect(mock.foo_call().and_return(result));
        assert_eq!(mock.foo(), UniquelyOwned(42));
    }

    fn return_panic(){
        #[mocked]
        pub trait A {
            fn foo(&self);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_panic("Panic".to_owned()));
        mock.foo();
    }

    fn return_parameters() {
        #[mocked]
        pub trait A {
            fn foo(&self, x: &mut u32);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(matchers::ANY)
                        .and_call(|arg| { *arg = 2 }));

        let mut value = 1;
        mock.foo(&mut value);
        assert_eq!(value, 2);
    }

    // https://github.com/kriomant/mockers/issues/22
    fn send() {
        //#[mocked]
        //pub trait A {
            //fn foo(&self);
        //}

        //let scenario = Scenario::new();
        //let mock = scenario.create_mock_for::<A>();
        //let _ = Box::new(mock) as Box<A + Send>;
        unimplemented!()
    }

    fn static_method() {
        #[mocked]
        pub trait A {
            fn foo(&self, x: u32) -> u32;
            fn bar() -> u32;
        }

        let scenario = Scenario::new();
        let mock_class = scenario.create_mock::<AMockStatic>();
        let mock_object = scenario.create_mock::<AMock>();
        scenario.expect(mock_object.foo_call(5).and_return(42));
        scenario.expect(mock_class.bar_call().and_return(99));
        assert_eq!(42, mock_object.foo(5));
        assert_eq!(99, AMock::bar());
    }

    fn sequence(){
        #[mocked]
        pub trait A {
            fn foo(&self);
            fn bar(&self);
        }

        let scenario = Scenario::new();
        let mut seq = Sequence::new();
        let mock1 = scenario.create_mock_for::<A>();
        let mock2 = scenario.create_mock_for::<A>();
        seq.expect(mock1.foo_call().and_return(()));
        seq.expect(mock2.bar_call().and_return(()));
        scenario.expect(seq);
        mock1.foo();
        mock2.bar();
        print!("multi object ");
    }

    fn times_any(){
        #[mocked]
        pub trait A {
            fn foo(&self);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_return_clone(()).times(..));
        mock.foo();
        mock.foo();
    }

    fn times_n(){
        #[mocked]
        pub trait A {
            fn foo(&self);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_return_clone(()).times(2));
        mock.foo();
        mock.foo();
    }

    fn times_never(){
        #[mocked]
        pub trait A {
            fn foo(&self);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().never());
    }

    fn times_once(){
        // By default, Mockers expects a mock to be called once
        #[mocked]
        pub trait A {
            fn foo(&self);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_return(()));
        mock.foo();
    }

    fn times_range(){
        #[mocked]
        pub trait A {
            fn foo(&self);
        }
        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_return_clone(()).times(2..4));
        mock.foo();
        mock.foo();
    }

    fn version() {
        let ver = crate::built_info::DEPENDENCIES.iter()
            .find(|(name, _)| *name == "mockers")
            .unwrap()
            .1;
        print!("{} ", ver);
    }
}

test!{Mockers}

}
