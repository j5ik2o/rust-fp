use crate::{Empty, Semigroup};

pub trait Monoid: Empty + Semigroup {}

use crate::impl_marker_trait_for_numeric;

impl_marker_trait_for_numeric!(Monoid);

impl<T: Clone> Monoid for Vec<T> {}
impl Monoid for String {}

#[cfg(test)]
mod laws {
    use crate::{Empty, Semigroup};

    #[quickcheck]
    fn monoid_left_identity(n: i32) {
        assert_eq!(i32::empty().combine(n), n)
    }

    #[quickcheck]
    fn monoid_right_identity(n: i32) {
        assert_eq!(n.combine(i32::empty()), n)
    }
}
