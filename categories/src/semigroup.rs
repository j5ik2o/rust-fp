pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

use crate::impl_semigroup_for_numeric;

impl_semigroup_for_numeric!();

impl<T> Semigroup for Vec<T> {
    fn combine(self, other: Self) -> Self {
        let mut concat = self;
        concat.extend_from_slice(&other);
        concat
    }
}

impl Semigroup for String {
    fn combine(self, other: Self) -> Self {
        format!("{}{}", self, other)
    }
}
