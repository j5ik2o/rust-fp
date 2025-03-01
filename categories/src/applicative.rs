use std::rc::Rc;

use crate::{Apply, Pure};

pub trait Applicative: Apply + Pure {}

macro_rules! applicative_numeric_impl {
    ($($t:ty)*) => ($(
       impl Applicative for $t {}
    )*)
}

applicative_numeric_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl<A> Applicative for Rc<A> {}
impl<A> Applicative for Box<A> {}

impl<A> Applicative for Option<A> {}
impl<A, E> Applicative for Result<A, E> {}
impl<A> Applicative for Vec<A> {}
