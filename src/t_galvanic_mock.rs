/// ```
/// extern crate galvanic_mock;
/// extern crate galvanic_assert;
/// use galvanic_mock::{mockable, use_mocks};
///
/// #[mockable]
/// pub trait A {
///     fn foo(&self, x: i16) -> i32;
/// }
/// 
/// #[use_mocks]
/// fn testit() {
///     let mock = new_mock!(A);
///     given! {
///         <mock as A>::foo(|_| true) then_return 12 always;
///     }
///     assert_eq!(12, mock.foo(0));
/// }
/// fn main() {
///     testit();
/// }
/// ```
fn doctest() {}

extern crate galvanic_assert;
extern crate galvanic_mock;
#[cfg(test)] use galvanic_assert::matchers::*;
use galvanic_mock::mockable;
#[cfg(test)] use galvanic_mock::use_mocks;
use UniquelyOwned;

// Galvanic_mock's macros don't work in function-scope, so we have to define all
// the traits up here.  This is unlikely to hinder most users.
#[mockable]
pub trait A {
    fn foo(&self, x: i16) -> i16;
    fn bar(self);
    fn baz(&self) -> UniquelyOwned;
}

#[mockable]
pub trait Derived: A {
    fn bean(self);
}

#[mockable]
pub trait B<'a> {
    fn foo(&'a self) -> &'a u32;
}

#[mockable]
pub trait C {
    fn boo(&self) -> i32;
}

#[derive(Clone, Copy)]
pub struct ConcreteC();
impl C for ConcreteC {
    fn boo(&self) -> i32 { 42 }
}

#[mockable]
pub trait GenericTrait<T> {
    fn foo(&self) -> T;
}

#[mockable]
pub trait GenericMethodTrait {
    fn foo<T>(&self, t: T) -> u32;
}

#[mockable]
pub trait AssociatedTrait {
    type Key;
    fn foo(&self, k: Self::Key) -> bool;
}

#[mockable]
pub trait ManyArgsTrait {
    fn foo(&self, a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8,
        i: i8, j: i8, k: i8, l: i8, m: i8, n: i8, o: i8, p: i8) -> u32;
}

// The docs suggest that this should work, but it doesn't and I can't figure out
// what's wrong.
//#[mockable(extern ::std::io)]
//trait Write {
    //fn write(&mut self, buf: &[u8]) -> Result<usize>;
    //fn flush(&mut self) -> Result<()>;
//}

#[allow(unreachable_code)]
#[use_mocks]
#[cfg(test)]
mod t {
    use std;
    use super::*;

use {TestSuite, UniquelyOwned};

pub struct MockGalvanicMock;
impl TestSuite for MockGalvanicMock{
    const NAME: &'static str = "galvanic-mock";
    fn associated_types() {
        let mock = new_mock!(AssociatedTrait<Key=i32>);
        given! {
            <mock as AssociatedTrait<Key=i32>>::foo(|_| true) then_return true always;
        }
        assert!(mock.foo(5i32));
    }

    fn checkpoint() { unimplemented!() }
    fn consume_parameters() {
        // Galvanic_mock match and return functions take parameters by reference
        unimplemented!()
    }

    fn consume_self() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::bar() then_return () always
        }
        mock.bar();
    }

    fn derive() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(|_| true) then_return 12 always;
        }
        assert_eq!(12, mock.foo(0));
    }

    fn external_trait() { unimplemented!() }
    fn fallback() {
        // Galvanic-mock lacks this capability.  In some cases, it can be
        // implemented with then_return and a closure.  But that doesn't always
        // work, because then_return supplies its closure with function arguments
        // by reference instead of by value.
        unimplemented!()
    }

    fn foreign() { unimplemented!() }
    // https://github.com/mindsbackyard/galvanic-mock/issues/7
    fn generic_method() {
        //let mock = new_mock!(GenericMethodTrait);
        //given! {
            //<mock as GenericMethodTrait>::foo(|x: &u32| *x == 42u32)
                //then_return 100u32 always;
            //<mock as GenericMethodTrait>::foo(|x: &i16| *x == 42i16)
                //then_return 1u32 always;
        //}
        //assert_eq!(100, mock.foo(42u32));
        //assert_eq!(1, mock.foo(42u32));
        unimplemented!()
    }

    fn generic_return() {
        // galvanic_mock can mock a method with a generic return value, but
        // there's no way to specify the return value except for default().
        unimplemented!()
    }

    fn generic_struct() {
        // galvanic_mock can't mock structs
        unimplemented!()
    }

    fn generic_trait() {
        let mock = new_mock!(GenericTrait<i32>);
        given! {
            <mock as GenericTrait<i32>>::foo() then_return 5 always;
        }
        assert_eq!(5, mock.foo());
    }

    fn inherited_trait() {
        let mock = new_mock!(Derived, A);
        given! {
            <mock as A>::foo(|_| true) then_return 12 always;
            <mock as Derived>::bean() then_return () always;
        }
        assert_eq!(12, mock.foo(0));
        mock.bean();
    }

    fn many_args() {
        let mock = new_mock!(ManyArgsTrait);
        given!{
            <mock as ManyArgsTrait>::foo(
                any_value(), any_value(), any_value(), any_value(),
                any_value(), any_value(), any_value(), any_value(),
                any_value(), any_value(), any_value(), any_value(),
                any_value(), any_value(), any_value(), any_value())
                then_return 1 always;
        }
        assert_eq!(1, mock.foo(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
                               14, 15));
        print!(">= 16 ");
    }

    fn match_combo() {
        // The docs suggest that this should work, but I can't get it to
        // compile
        //let mock = new_mock!(A);
        //given! {
            //<mock as A>::foo(All::of(gt(0)).and(lt(10))) then_return 1 always;
            //<mock as A>::foo(Any::of(gt(10)).or(lt(0))) then_return 1 always;
        //}
        //assert_eq!(1, mock.foo(5));
        //assert_eq!(1, mock.foo(-1));
        unimplemented!()
    }

    fn match_constant() { unimplemented!() }

    fn match_method() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(|&x| x == 5) then_return 12 always;
        }
        assert_eq!(12, mock.foo(5));
    }

    fn match_operator() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(eq(5)) then_return 1 always;
            <mock as A>::foo(geq(10)) then_return 2 always;
            <mock as A>::foo(gt(8)) then_return 3 always;
            <mock as A>::foo(leq(2)) then_return 4 always;
            <mock as A>::foo(lt(4)) then_return 5 always;
            <mock as A>::foo(not(eq(5))) then_return 6 always;
        }
        assert_eq!(1, mock.foo(5));
        assert_eq!(2, mock.foo(10));
        assert_eq!(3, mock.foo(9));
        assert_eq!(4, mock.foo(2));
        assert_eq!(5, mock.foo(3));
        assert_eq!(6, mock.foo(6));
    }

    fn match_pattern() {
        // galvanic_assert has matchers for Option and Result types, but no way
        // to match arbitrary Enum types
        unimplemented!()
    }

    fn match_range() {
        unimplemented!()
    }

    fn match_wildcard() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(any_value()) then_return 12 always;
        }
        assert_eq!(12, mock.foo(5));
    }

    fn mock_struct() { unimplemented!() }
    fn modules() { unimplemented!() }
    fn mock_trait() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(|_| true) then_return_from |_| 1 always;
        }
        assert_eq!(1, mock.foo(5));
    }

    fn multi_trait() {
        let mock = new_mock!(A, C);
        given! {
            <mock as A>::foo(|_| true) then_return 12 always;
            <mock as C>::boo() then_return -4 always;
        }
        assert_eq!(12, mock.foo(5));
        assert_eq!(-4, mock.boo());
    }

    fn return_call() {
        unimplemented!()
    }

    fn return_call_with_args() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(|_| true) then_return_from |&(x)| x + 1 always;
        }
        assert_eq!(6, mock.foo(5));
    }

    fn return_constant() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(|_| true) then_return 12 always;
        }
        assert_eq!(12, mock.foo(0));
    }

    fn return_default() { unimplemented!() }
    fn return_lifetime() {
        let mock = new_mock!(B);
        given! {
            <mock as B>::foo() then_return &5u32 always;
        }
        assert_eq!(5, *mock.foo());
    }

    fn return_owned() {
        // Galvanic mock supports two kinds of returns:
        // then_return returns constants (which can't be moved)
        // then_return_from returns from a function (not a closure, so it can't
        // move anything out).
        unimplemented!()
    }

    fn return_panic() {
        // Galvanic-mock has this ability, though you can't set the panic
        // message.
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(|_| true) then_panic always;
        }
        mock.foo(0);
    }

    fn return_parameters() {
        // Galvanic mock can't implement this, because matchers only provide
        // their arguments by immutable reference.
        unimplemented!()
    }

    // https://github.com/mindsbackyard/galvanic-mock/issues/6
    fn send() {
        unimplemented!()
    }

    fn static_method() {
        unimplemented!()
    }

    fn sequence() { unimplemented!() }
    fn times_once() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(|_| true) then_return 12 always;
        }
        expect_interactions! {
            <mock as A>::foo(|_| true) times 1;
        }
        mock.foo(0);
    }

    fn times_any() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(|_| true) then_return 12 always;
        }
        mock.foo(0);
        mock.foo(0);
        mock.foo(0);
    }

    fn times_n() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(|_| true) then_return 12 always;
        }
        expect_interactions! {
            <mock as A>::foo(|_| true) times 2;
        }
        mock.foo(0);
        mock.foo(0);
    }

    fn times_never() {
        let mock = new_mock!(A);
        given! {
            <mock as A>::foo(|_| true) then_return 12 always
        }
        expect_interactions! {
            <mock as A>::foo(|_| true) times 0;
        }
    }

    fn times_range() { unimplemented!() }

    fn version() {
        let ver = ::built_info::DEPENDENCIES.iter()
            .find(|(name, _)| *name == "galvanic-mock")
            .unwrap()
            .1;
        print!("{} ", ver);
    }
}

test!{MockGalvanicMock}

}
