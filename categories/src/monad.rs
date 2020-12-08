use applicative::Applicative;
use bind::Bind;
use std::rc::Rc;

pub trait Monad: Bind + Applicative {}

impl<A> Monad for Rc<A> {}
impl<A> Monad for Box<A> {}

impl<A> Monad for Option<A> {}
impl<A, E: Clone> Monad for Result<A, E> {}
impl<A> Monad for Vec<A> {}

#[cfg(test)]
mod laws {
    use bind::Bind;

    #[quickcheck]
    fn left_point(n: i64) {
        assert_eq!(Some(n).bind(|x| Some(*x)), Some(n))
    }

    #[quickcheck]
    fn combine(n: i64) {
        assert_eq!(
            Some(n).bind(|x| Some(*x)).bind(|y| Some(*y)),
            Some(n).bind(|x| Some(*x).bind(|y| Some(*y)))
        )
    }
}
