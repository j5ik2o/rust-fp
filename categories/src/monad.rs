use applicative::Applicative;
use bind::Bind;
use std::rc::Rc;

pub trait Monad: Bind + Applicative {}

impl<A> Monad for Rc<A> {}
impl<A> Monad for Box<A> {}

impl<A> Monad for Option<A> {}
impl<A, E: Clone> Monad for Result<A, E> {}
impl<A> Monad for Vec<A> {}
