use std::rc::Rc;

pub trait Apply {
    type Elm;
    type M<B>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B;
}

// ---

impl<A> Apply for Rc<A> {
    type Elm = A;
    type M<U> = Rc<U>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        let v = fs(&self);
        Rc::new(v)
    }
}

impl<A> Apply for Box<A> {
    type Elm = A;
    type M<U> = Box<U>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        let v = fs(&self);
        Box::new(v)
    }
}

// ---

impl<A> Apply for Option<A> {
    type Elm = A;
    type M<U> = Option<U>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        let v = self?;
        let f = fs?;
        Some(f(&v))
    }
}

impl<A, E: Clone> Apply for Result<A, E> {
    type Elm = A;
    type M<U> = Result<U, E>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        let x = self?;
        let fs = fs?;
        Ok(fs(&x))
    }
}

impl<A> Apply for Vec<A> {
    type Elm = A;
    type M<U> = Vec<U>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        let zipped = self.iter().zip(fs.iter());
        zipped.map(|(x, f)| f(x)).collect::<Vec<B>>()
    }
}
