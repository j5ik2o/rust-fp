#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

// 型クラス階層の基本構造については hierarchy.md を参照してください
mod applicative;
mod apply;
mod bind;
mod common;
pub mod common_optimized;
mod empty;
mod foldable;
mod functor;
mod macros;
mod monad;
mod monoid;
mod pure;
mod semigroup;

pub use applicative::*;
pub use apply::*;
pub use bind::*;
pub use common::*;
// pub use common_optimized as common_opt;
pub use empty::*;
pub use foldable::*;
pub use functor::*;
pub use monad::*;
pub use monoid::*;
pub use pure::*;
pub use semigroup::*;

#[cfg(test)]
mod tests {
    use crate::{Apply, Bind, Foldable, Functor};

    #[test]
    fn it_works() {
        let v: Option<i32> = Some(10).bind(|x| Some(20).fmap(|y| x + y));
        println!("{:?}", v);
        let v2: Option<i32> = Some(10).ap(Some(|x: &i32| x + 20));
        println!("{:?}", v2);
        let vec = vec![1, 3, 5];
        let n: i32 = vec.fold_left(0, |x, y: &i32| x + y);
        println!("{:?}", n)
    }
}
