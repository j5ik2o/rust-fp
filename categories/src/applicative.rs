use std::rc::Rc;

use crate::{Apply, Pure};

pub trait Applicative: Apply + Pure {}

use crate::impl_marker_trait_for_numeric;

impl_marker_trait_for_numeric!(Applicative);

impl<A> Applicative for Rc<A> {}
impl<A> Applicative for Box<A> {}

impl<A> Applicative for Option<A> {}
impl<A, E> Applicative for Result<A, E> {}
impl<A> Applicative for Vec<A> {}
