use std::rc::Rc;

pub trait Bind {
    type Elm;
    type M<B>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> Self::M<B>;
}

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
        match self {
            Some(ref value) => f(value),
            None => None,
        }
    }
}

impl<A, E: Clone> Bind for Result<A, E> {
    type Elm = A;
    type M<U> = Result<U, E>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> Self::M<B>,
    {
        match self {
            Ok(v) => f(&v),
            Err(e) => Err(e),
        }
    }
}

impl<A> Bind for Vec<A> {
    type Elm = A;
    type M<U> = Vec<U>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> Self::M<B>,
    {
        self.iter().flat_map(f).collect()
    }
}
