#![feature(proc_macro_hygiene)] // Needed by galvanic_mock and mocktopus
#![feature(specialization)] // Used by mockers 0.13.1 for generic methods

#![allow(dead_code)]

#[allow(unused)]
macro_rules! test {
    ( $self:ident) => {
        #[test] fn associated_types() { $self::associated_types() }
        #[test] fn checkpoint() { $self::checkpoint() }
        #[test] fn reference_parameters() { $self::reference_parameters() }
        #[test] fn consume_parameters() { $self::consume_parameters() }
        #[test] fn consume_self() { $self::consume_self() }
        #[test] fn derive() { $self::derive() }
        #[test] fn external_trait() { $self::external_trait() }
        #[test] fn fallback() { $self::fallback() }
        #[test] fn foreign() { $self::foreign() }
        #[test] fn generic_method() { $self::generic_method() }
        #[test] fn generic_return() { $self::generic_return() }
        #[test] fn generic_struct() { $self::generic_struct() }
        #[test] fn generic_trait() { $self::generic_trait() }
        #[test] fn inherited_trait() { $self::inherited_trait() }
        #[test] fn many_args() { $self::many_args() }
        #[test] fn match_combo() { $self::match_combo() }
        #[test] fn match_constant() { $self::match_constant() }
        #[test] fn match_method() { $self::match_method() }
        #[test] fn match_operator() { $self::match_operator() }
        #[test] fn match_pattern() { $self::match_pattern() }
        #[test] fn match_range() { $self::match_range() }
        #[test] fn match_wildcard() { $self::match_wildcard() }
        #[test] fn modules() { $self::modules() }
        #[test] fn mock_struct() { $self::mock_struct() }
        #[test] fn mock_trait() { $self::mock_trait() }
        #[test] fn multi_trait() { $self::multi_trait() }
        #[test] fn return_call_with_args() { $self::return_call_with_args() }
        #[test] fn return_constant() { $self::return_constant() }
        #[test] fn return_default() { $self::return_default() }
        #[test] fn return_lifetime() { $self::return_lifetime() }
        #[test] fn return_owned() { $self::return_owned() }
        #[should_panic(expected="Panic")] #[test] fn return_panic() {
            $self::return_panic()
        }
        #[test] fn return_parameters() { $self::return_parameters() }
        #[test] fn send() { $self::send() }
        #[test] fn sequence() { $self::sequence() }
        #[test] fn static_method() { $self::static_method() }
        #[test] fn times_once() { $self::times_once() }
        #[test] fn times_any() { $self::times_any() }
        #[test] fn times_n() { $self::times_n() }
        #[test] fn times_never() { $self::times_never() }
        #[test] fn times_range() { $self::times_range() }
        #[test] fn version() { $self::version() }
        #[test] fn where_clause() { $self::where_clause() }
        #[test] fn link() {
            print!("<a href=\"https://crates.io/crates/{}\"> <img src=\"https://img.shields.io/crates/v/{}.svg\"> </a>", $self::NAME, $self::NAME);
        }
    }
}

mod t_double;
mod t_galvanic_mock;
//// Disable mock_derive until it gets fixed for newer nightly toolchains
//// https://github.com/DavidDeSimone/mock_derive/issues/18
////mod t_mock_derive;
mod t_mock_it;
mod t_mockall;
mod t_mockers;
mod t_mocktopus;
mod t_pseudo;
mod t_simulacrum;

/// A handy type that is non-Clone and non-Copy
#[derive(Debug, Eq, PartialEq)]
pub struct UniquelyOwned(u32);

pub trait TestSuite {
    const NAME: &'static str;

    // Core features.  These are the essential features that cannot be
    // implemented by the user
    /// A mocked `Trait` can have associated types
    fn associated_types();
    /// A mock can set a barrier.  All expectations defined before the barrier must
    /// be satisified before, and all expectations defined after the barrier
    /// must be satisfied after.
    fn checkpoint();
    /// A mock method can take its parameters by reference.
    fn reference_parameters();
    /// A mock method can consume its parameters, passing them by value to an
    /// arbitrary function.
    fn consume_parameters();
    /// A mock method can consume `self`
    fn consume_self();
    /// A Mock can be defined for a `Trait` in an external crate;
    fn external_trait();
    /// Can mock foreign functions
    fn foreign();
    /// A `Trait` with a method that has generic parameters can be mocked.
    fn generic_method();
    /// A `Trait` with a method that has a generic return value can be mocked.
    fn generic_return();
    /// A generic `struct` can be mocked.
    fn generic_struct();
    /// A generic `Trait` can be mocked.
    fn generic_trait();
    /// An inherited `Trait` can be mocked.
    fn inherited_trait();
    /// A method call can match an argument by an arbitrary method
    fn match_method();
    /// A plain `Struct` can be mocked.
    fn mock_struct();
    /// An abstract Trait can be mocked
    fn mock_trait();
    /// A Mock can be defined that implements multiple `Trait`s.
    fn multi_trait();
    /// A mock method can return the result of a function that depends on its
    /// arguments.
    fn return_call_with_args();
    /// A mock method can return a reference with a non-`'static` lifetime
    fn return_lifetime();
    /// A mock method can return an owned object that is neither `Copy` nor
    /// `Clone`
    fn return_owned();
    /// A mock method can mutate its parameters when supplied by mutable
    /// reference.
    fn return_parameters();
    /// A `Send` `Trait` can be mocked
    fn send();
    /// A `Trait` with a static method can be mocked (though the static method
    /// cannot neccessarily be expected)
    fn static_method();
    /// An expectation can assert that it's called a variable number of times
    fn times_range();

    // Convenience features.  These are nice-to-have, but their absence doesn't
    // fundamentally limit the library's capabilities.  The user can either
    // implement these himself, or make do with their lack.
    /// A Mock can be `Derive`d from a `Trait`
    fn derive();
    /// A mock object can fallback to the real object's behavior
    fn fallback();
    /// A method call can match a combination of conditions
    fn match_combo();
    /// A method call can match a constant value
    fn match_constant();
    /// A method call can match an argument by common operators
    fn match_operator();
    /// A method call can match a pattern
    fn match_pattern();
    /// A method call can match a range of values
    fn match_range();
    /// A method call can match any argument at all
    fn match_wildcard();
    /// An entire module can be mocked
    fn modules();
    /// A mock method can return a constant value
    fn return_constant();
    /// A mock method can return the type's default value
    fn return_default();
    /// A mock method can panic, rather than return
    fn return_panic();
    /// An expectation can assert that it's called only once
    fn times_once();
    /// An expectation can be called any number of times
    fn times_any();
    /// An expectation can be called a specific number of times
    fn times_n();
    /// An expectation can assert that it's never called
    fn times_never();

    // Quantitative features.  These are measured in degrees, rather than as
    // booleans.  The degree of support is printed to stdout.
    /// How many arguments can be mocked?
    fn many_args();
    /// A scenario can expect calls to multiple methods in a specified order
    fn sequence();
    /// Current crate version
    fn version();
    /// Structs, traits, and methods with where clauses can be mocked, and the
    /// expectation will satisfy the where clauses.
    fn where_clause();
}

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}
