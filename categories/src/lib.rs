#![feature(generic_associated_types)]

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod applicative;
pub mod apply;
pub mod bind;
pub mod empty;
pub mod foldable;
pub mod functor;
pub mod monad;
pub mod monoid;
pub mod pure;
pub mod semigroup;

#[cfg(test)]
mod tests {
    use apply::Apply;
    use bind::Bind;
    use foldable::Foldable;
    use functor::Functor;

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
