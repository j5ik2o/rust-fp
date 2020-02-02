use std::rc::Rc;

use apply::Apply;
use pure::Pure;

pub trait Applicative<A, B>: Apply<B> + Pure<A> {}

impl<A, B> Applicative<A, B> for Rc<A> {}
impl<A, B> Applicative<A, B> for Box<A> {}

impl<A, B> Applicative<A, B> for Option<A> {}
impl<A, B, E: Clone> Applicative<A, B> for Result<A, E> {}
impl<A, B> Applicative<A, B> for Vec<A> {}
