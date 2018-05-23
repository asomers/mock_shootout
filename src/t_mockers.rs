/// ```
/// #![feature(proc_macro)]
/// extern crate mockers;
/// extern crate mockers_derive;
/// use mockers::*;
/// #[macro_use] use mockers_derive::*;
/// #[derive_mock]
/// pub trait A {
///     fn foo(&self, key: i16);
/// }
/// 
/// let scenario = Scenario::new();
/// let _mock = scenario.create_mock_for::<A>();
/// ```
fn doctest() {}

#[cfg(test)]
mod t {

extern crate mockers;
extern crate mockers_derive;
use mockers::*;
#[macro_use] use mockers_derive::*;
use {TestSuite, UniquelyOwned};

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
    fn associated_types(){
        #[derive_mock]
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
        #[derive_mock]
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

    fn consume(){
        #[derive_mock]
        pub trait A {
            fn into_nothing(self);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.into_nothing_call().and_return(()));
        mock.into_nothing();
    }

    fn derive(){
        #[derive_mock]
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
        unimplemented!()
    }

    fn foreign() {
        unimplemented!()
    }

    fn generic_method(){
        //#[derive_mock]
        //pub trait A {
            //fn foo<T>(&self, t:T);
            //fn bar<T>(&self) -> T;
        //}

        //let scenario = Scenario::new();
        //let mock = scenario.create_mock_for::<A>();
        unimplemented!();
    }

    fn generic_trait(){
        unimplemented!();
    }

    fn inherited_trait(){
        #[derive_mock(module="::t_mockers::t")]
        pub trait A {
            fn foo(&self);
        }
       
        #[derive_mock(refs="A => ::t_mockers::t::A")]
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
         #[derive_mock]
         pub trait A {
             fn foo(&self, a: i8, b: i8, c: i8, d: i8);
         }

         let scenario = Scenario::new();
         let mock = scenario.create_mock_for::<A>();
         scenario.expect(mock.foo_call(0, 1, 2, 3).and_return(()));
         mock.foo(0, 1, 2, 3);
         println!("4");
    }

    fn match_and(){
        #[derive_mock]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(
            matchers::and(matchers::gt(1), matchers::lt(10))).and_return(()));
        mock.foo(5);
    }

    fn match_constant(){
        #[derive_mock]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(5).and_return(()));
        mock.foo(5);
    }

    fn match_method(){
        #[derive_mock]
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
        #[derive_mock]
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

    fn match_or(){
        #[derive_mock]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(
            matchers::or(matchers::gt(10), matchers::lt(0))).and_return(()));
        mock.foo(-1);
    }

    fn match_pattern(){
        #[derive_mock]
        pub trait A {
            fn foo(&self, key: Option<i16>);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(arg!(Some(_))).and_return(()));
        mock.foo(Some(1));
    }

    fn match_range(){
        #[derive_mock]
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
        #[derive_mock]
        pub trait A {
            fn foo(&self, key: i16);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(matchers::ANY).and_return(()));
        mock.foo(2);
    }

    fn mock_struct() { unimplemented!() }
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
        #[derive_mock]
        pub trait A {
            fn foo(&self, x: i16) -> i16;
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call(matchers::ANY).and_call(|x| x+1));
        assert_eq!(mock.foo(2), 3);
    }

    fn return_constant(){
        #[derive_mock]
        pub trait A {
            fn foo(&self) -> i16;
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_return(2));
        assert_eq!(mock.foo(), 2);
    }

    fn return_default(){
        #[derive_mock]
        pub trait A {
            fn foo(&self) -> i16;
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_return_default().times(1));
        assert_eq!(mock.foo(), 0);
    }

    fn return_lifetime(){
        //#[derive_mock]
        //pub trait A {
            //fn foo(&'a self) -> &'a A;
        //}

        //let scenario = Scenario::new();
        //let _mock = scenario.create_mock_for::<A>();
        unimplemented!()
    }

    fn return_owned(){
        #[derive_mock]
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
        #[derive_mock]
        pub trait A {
            fn foo(&self);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_panic("Panic".to_owned()));
        mock.foo();
    }

    fn return_parameters() {
        // Mockers can't implement this, because matchers only provide their
        // arguments by immutable reference.
        unimplemented!()
    }

    fn static_method() {
        unimplemented!()
    }

    fn sequence(){
        #[derive_mock]
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
        println!("multi object");
    }

    fn times_any(){
        #[derive_mock]
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
        #[derive_mock]
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
        #[derive_mock]
        pub trait A {
            fn foo(&self);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().never());
    }

    fn times_once(){
        // By default, Mockers expects a mock to be called once
        #[derive_mock]
        pub trait A {
            fn foo(&self);
        }

        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_return(()));
        mock.foo();
    }

    fn times_range(){
        #[derive_mock]
        pub trait A {
            fn foo(&self);
        }
        let scenario = Scenario::new();
        let mock = scenario.create_mock_for::<A>();
        scenario.expect(mock.foo_call().and_return_clone(()).times(2..4));
        mock.foo();
        mock.foo();
    }
}

test!{Mockers}

}
