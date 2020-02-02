use std::rc::Rc;

use hkt::HKT;

pub trait Functor<A>: HKT<A> {
    fn fmap<F>(&self, f: F) -> Self::T
    where
        F: Fn(&Self::C) -> A;
}

impl<A, B> Functor<B> for Rc<A> {
    fn fmap<F>(&self, f: F) -> Rc<B>
    where
        F: FnOnce(&A) -> B,
    {
        let v = f(self);
        Rc::new(v)
    }
}

impl<A, B> Functor<B> for Box<A> {
    fn fmap<F>(&self, f: F) -> Box<B>
    where
        F: FnOnce(&A) -> B,
    {
        let v = f(self);
        Box::new(v)
    }
}

// ---

impl<A, B> Functor<B> for Option<A> {
    fn fmap<F>(&self, f: F) -> Option<B>
    where
        F: FnOnce(&A) -> B,
    {
        match *self {
            Some(ref v) => Some(f(v)),
            None => None,
        }
    }
}

impl<A, B, E: Clone> Functor<B> for Result<A, E> {
    fn fmap<F>(&self, f: F) -> Result<B, E>
    where
        F: FnOnce(&A) -> B,
    {
        match self {
            &Ok(ref v) => Ok(f(v)),
            &Err(ref e) => Err(e.clone()),
        }
    }
}

impl<A, B> Functor<B> for Vec<A> {
    fn fmap<F>(&self, f: F) -> Vec<B>
    where
        F: Fn(&A) -> B,
    {
        self.iter().map(f).collect::<Vec<B>>()
    }
}
