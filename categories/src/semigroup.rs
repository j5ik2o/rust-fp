pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

use crate::impl_semigroup_for_numeric;

impl_semigroup_for_numeric!();

impl<T: Clone> Semigroup for Vec<T> {
    fn combine(self, other: Self) -> Self {
        let mut concat = self;
        concat.extend(other);
        concat
    }
}

impl Semigroup for String {
    fn combine(self, other: Self) -> Self {
        format!("{}{}", self, other)
    }
}
