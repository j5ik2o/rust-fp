use crate::{Empty, Semigroup};

pub trait Monoid: Empty + Semigroup {}

macro_rules! monoid_numeric_impl {
    ($($t:ty)*) => ($(
       impl Monoid for $t {}
    )*)
}

monoid_numeric_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl<T> Monoid for Vec<T> {}
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
