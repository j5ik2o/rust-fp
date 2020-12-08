use std::rc::Rc;

use apply::Apply;
use pure::Pure;

pub trait Applicative: Apply + Pure {}

impl<A> Applicative for Rc<A> {}
impl<A> Applicative for Box<A> {}

impl<A> Applicative for Option<A> {}
impl<A, E: Clone> Applicative for Result<A, E> {}
impl<A> Applicative for Vec<A> {}
