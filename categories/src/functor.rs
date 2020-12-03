use std::rc::Rc;

use hkt::HKT;

pub trait Functor<A>: HKT<A> {
    fn fmap<F>(self, f: F) -> Self::T
    where
        F: Fn(&Self::C) -> A;
}

impl<A, B> Functor<B> for Rc<A> {
    fn fmap<F>(self, f: F) -> Rc<B>
    where
        F: FnOnce(&A) -> B,
    {
        let v = f(&self);
        Rc::new(v)
    }
}

impl<A, B> Functor<B> for Box<A> {
    fn fmap<F>(self, f: F) -> Box<B>
    where
        F: FnOnce(&A) -> B,
    {
        let v = f(&self);
        Box::new(v)
    }
}

// ---

impl<A, B> Functor<B> for Option<A> {
    fn fmap<F>(self, f: F) -> Option<B>
    where
        F: FnOnce(&A) -> B,
    {
        match self {
            Some(ref v) => Some(f(v)),
            None => None,
        }
    }
}

impl<A, B, E> Functor<B> for Result<A, E> {
    fn fmap<F>(self, f: F) -> Result<B, E>
    where
        F: FnOnce(&A) -> B,
    {
        match self {
            Ok(v) => Ok(f(&v)),
            Err(e) => Err(e),
        }
    }
}

impl<A, B> Functor<B> for Vec<A> {
    fn fmap<F>(self, f: F) -> Vec<B>
    where
        F: Fn(&A) -> B,
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
