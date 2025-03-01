extern crate rust_fp_categories;

mod list;
mod set;
mod stack;
mod tree;

pub use list::*;
pub use set::*;
pub use stack::*;
pub use tree::*;

#[cfg(test)]
mod tests {
    use crate::{List, Stack};
    use rust_fp_categories::*;

    #[test]
    fn it_works() {
        let list1: List<i32> = List::empty().cons(30).cons(20).cons(10);
        println!("{:?}", list1);
        let list2 = list1.bind(|x| List::empty().cons(x * 2).fmap(|x| x - 1));
        println!("{:?}", list2);
    }
}
