use std::rc::Rc;

use crate::hkt::HKT;

pub trait Pure<A>: HKT<A> {
    fn pure(value: A) -> Self::T
    where
        Self: HKT<A, C = A>;
}

impl<A> Pure<A> for Rc<A> {
    fn pure(value: A) -> Self::T {
        Rc::new(value)
    }
}

impl<A> Pure<A> for Box<A> {
    fn pure(value: A) -> Self::T {
        Box::new(value)
    }
}

// ---

impl<A> Pure<A> for Option<A> {
    fn pure(value: A) -> Self::T {
        Some(value)
    }
}

impl<A, E: Clone> Pure<A> for Result<A, E> {
    fn pure(value: A) -> Self::T {
        Ok(value)
    }
}

impl<A> Pure<A> for Vec<A> {
    fn pure(value: A) -> Self::T {
        vec![value]
    }
}
