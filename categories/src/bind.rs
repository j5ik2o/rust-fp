use std::rc::Rc;

use hkt::HKT;

pub trait Bind<A>: HKT<A> {
    fn bind<F>(&self, f: F) -> Self::T
    where
        F: Fn(&Self::C) -> Self::T;
}

impl<A, B> Bind<B> for Rc<A> {
    fn bind<F>(&self, f: F) -> Self::T
    where
        F: FnOnce(&Self::C) -> Self::T,
    {
        f(self)
    }
}

impl<A, B> Bind<B> for Box<A> {
    fn bind<F>(&self, f: F) -> Self::T
    where
        F: FnOnce(&Self::C) -> Self::T,
    {
        f(self)
    }
}

// ---

impl<A, B> Bind<B> for Option<A> {
    fn bind<F>(&self, f: F) -> Option<B>
    where
        F: FnOnce(&A) -> Option<B>,
    {
        match self {
            &Some(ref value) => f(value),
            &None => None,
        }
    }
}

impl<A, B, E: Clone> Bind<B> for Result<A, E> {
    fn bind<F>(&self, f: F) -> Result<B, E>
    where
        F: FnOnce(&Self::C) -> Result<B, E>,
    {
        match self {
            &Ok(ref v) => f(v),
            &Err(ref e) => Err(e.clone()),
        }
    }
}

impl<A, B> Bind<B> for Vec<A> {
    fn bind<F>(&self, f: F) -> Vec<B>
    where
        F: Fn(&Self::C) -> Vec<B>,
    {
        self.iter().flat_map(f).collect()
    }
}
