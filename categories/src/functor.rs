#[allow(dead_code)]
use std::rc::Rc;

pub trait Functor {
    type Elm;
    type M<B>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B;
}

macro_rules! functor_numeric_impl {
    ($($t:ty)*) => ($(
        impl Functor for $t {
          type Elm = $t;
          type M<U> = U;

          fn fmap<B, F>(self, f: F) -> Self::M<B>
          where
            F: Fn(&Self::Elm) -> B,
          {
            f(&self)
          }
        }
    )*)
}

functor_numeric_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

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
    mod option {
        use crate::Functor;
        use std::convert::identity;

        #[quickcheck]
        fn covariant_identity(n: Option<i32>) -> bool {
            n.fmap(|x| identity(*x)) == n
        }

        #[quickcheck]
        fn covariant_composition(n: Option<i32>) -> bool {
            let f1: fn(&i32) -> i32 = |x| *x * 2;
            let f2: fn(&i32) -> i32 = |x| *x + 4;
            n.fmap(f1).fmap(f2) == n.fmap(|x| f2(&f1(x)))
        }
    }

    mod result {
        use crate::Functor;
        use std::convert::identity;

        #[quickcheck]
        fn covariant_identity(n: Result<i32, String>) -> bool {
            let expected = n.clone();
            n.fmap(|x| identity(*x)) == expected
        }

        #[quickcheck]
        fn covariant_composition(n: Result<i32, String>) -> bool {
            let expected = n.clone();
            let f1: fn(&i32) -> i32 = |x| *x * 2;
            let f2: fn(&i32) -> i32 = |x| *x + 4;
            n.fmap(f1).fmap(f2) == expected.fmap(|x| f2(&f1(x)))
        }
    }

    mod vec {
        use crate::Functor;
        use std::convert::identity;

        #[quickcheck]
        fn covariant_identity(n: Vec<i32>) -> bool {
            let expected = n.clone();
            n.fmap(|x| identity(*x)) == expected
        }

        #[quickcheck]
        fn covariant_composition(n: Vec<i32>) -> bool {
            let expected = n.clone();
            let f1: fn(&i32) -> i32 = |x| *x * 2;
            let f2: fn(&i32) -> i32 = |x| *x + 4;
            n.fmap(f1).fmap(f2) == expected.fmap(|x| f2(&f1(x)))
        }
    }
}
