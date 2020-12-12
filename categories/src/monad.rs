use std::rc::Rc;

use crate::{Applicative, Bind};

pub trait Monad: Bind + Applicative {}

macro_rules! monad_numeric_impl {
    ($($t:ty)*) => ($(
       impl Monad for $t {}
    )*)
}

monad_numeric_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl<A> Monad for Rc<A> {}
impl<A> Monad for Box<A> {}

impl<A> Monad for Option<A> {}
impl<A, E: Clone> Monad for Result<A, E> {}
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
