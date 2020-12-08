#[allow(dead_code)]
use std::rc::Rc;

pub trait Functor {
    type Elm;
    type M<B>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B;
}

impl<A> Functor for Rc<A> {
    type Elm = A;
    type M<U> = Rc<U>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> B,
    {
        let v = f(&self);
        Rc::new(v)
    }
}

impl<A> Functor for Box<A> {
    type Elm = A;
    type M<U> = Box<U>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> B,
    {
        let v = f(&self);
        Box::new(v)
    }
}

// ---

impl<A> Functor for Option<A> {
    type Elm = A;
    type M<B> = Option<B>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> B,
    {
        match self {
            Some(ref v) => Some(f(v)),
            None => None,
        }
    }
}

impl<A, E> Functor for Result<A, E> {
    type Elm = A;
    type M<B> = Result<B, E>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> B,
    {
        match self {
            Ok(v) => Ok(f(&v)),
            Err(e) => Err(e),
        }
    }
}

impl<A> Functor for Vec<A> {
    type Elm = A;
    type M<B> = Vec<B>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        self.iter().map(f).collect::<Vec<B>>()
    }
}

#[cfg(test)]
mod laws {
    use functor::Functor;
    use std::convert::identity;

    #[quickcheck]
    fn option_law1(n: Option<i32>) -> bool {
        n.fmap(|x| identity(*x)) == n
    }

    #[quickcheck]
    fn option_law2(n: Option<i32>) -> bool {
        let f1: fn(&i32) -> i32 = |x| *x * 2;
        let f2: fn(&i32) -> i32 = |x| *x + 4;
        n.fmap(f1).fmap(f2) == n.fmap(|x| f2(&f1(x)))
    }

    #[quickcheck]
    fn result_law1(n: Result<i32, String>) -> bool {
        let expected = n.clone();
        n.fmap(|x| identity(*x)) == expected
    }

    #[quickcheck]
    fn result_law2(n: Result<i32, String>) -> bool {
        let expected = n.clone();
        let f1: fn(&i32) -> i32 = |x| *x * 2;
        let f2: fn(&i32) -> i32 = |x| *x + 4;
        n.fmap(f1).fmap(f2) == expected.fmap(|x| f2(&f1(x)))
    }

    #[quickcheck]
    fn vec_law1(n: Vec<i32>) -> bool {
        let expected = n.clone();
        n.fmap(|x| identity(*x)) == expected
    }

    #[quickcheck]
    fn vec_law2(n: Vec<i32>) -> bool {
        let expected = n.clone();
        let f1: fn(&i32) -> i32 = |x| *x * 2;
        let f2: fn(&i32) -> i32 = |x| *x + 4;
        n.fmap(f1).fmap(f2) == expected.fmap(|x| f2(&f1(x)))
    }
}
