use std::rc::Rc;

pub trait Bind {
    type Elm;
    type M<B>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> Self::M<B>;
}

use crate::impl_bind_for_numeric;

impl_bind_for_numeric!();

impl<A> Bind for Rc<A> {
    type Elm = A;
    type M<U> = Rc<U>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> Self::M<B>,
    {
        f(&self)
    }
}

impl<A> Bind for Box<A> {
    type Elm = A;
    type M<U> = Box<U>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> Self::M<B>,
    {
        f(&self)
    }
}

// ---

impl<A> Bind for Option<A> {
    type Elm = A;
    type M<U> = Option<U>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> Self::M<B>,
    {
        self.and_then(|e| f(&e))
    }
}

impl<A, E> Bind for Result<A, E> {
    type Elm = A;
    type M<U> = Result<U, E>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> Self::M<B>,
    {
        self.and_then(|e| f(&e))
    }
}

impl<A> Bind for Vec<A> {
    type Elm = A;
    type M<U> = Vec<U>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> Self::M<B>,
    {
        self.iter().flat_map(f).collect()
    }
}
