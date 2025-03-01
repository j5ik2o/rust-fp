use std::rc::Rc;

use crate::{Applicative, Bind};

pub trait Monad: Bind + Applicative {}

use crate::impl_marker_trait_for_numeric;

impl_marker_trait_for_numeric!(Monad);

impl<A> Monad for Rc<A> {}
impl<A> Monad for Box<A> {}

impl<A> Monad for Option<A> {}
impl<A, E> Monad for Result<A, E> {}
impl<A> Monad for Vec<A> {}

#[cfg(test)]
mod laws {
    use crate::{Bind, Pure};

    #[quickcheck]
    fn monad_left_identity_law(n: i64) {
        assert_eq!(Option::pure(n).bind(|x| Option::pure(*x)), Option::pure(n))
    }

    #[quickcheck]
    fn monad_right_identity_law(n: i64) {
        assert_eq!(
            Option::pure(n)
                .bind(|x| Option::pure(*x))
                .bind(|y| Option::pure(*y)),
            Option::pure(n).bind(|x| Option::pure(*x).bind(|y| Option::pure(*y)))
        )
    }
}
