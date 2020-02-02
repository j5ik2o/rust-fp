use applicative::Applicative;
use bind::Bind;
use std::rc::Rc;

trait Monad<A, B>: Bind<B> + Applicative<A, B> {}

impl<A, B> Monad<A, B> for Rc<A> {}
impl<A, B> Monad<A, B> for Box<A> {}

impl<A, B> Monad<A, B> for Option<A> {}
impl<A, B, E: Clone> Monad<A, B> for Result<A, E> {}
impl<A, B> Monad<A, B> for Vec<A> {}
