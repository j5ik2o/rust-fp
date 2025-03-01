extern crate rust_fp_categories;

mod array_stack;
mod btree_set;
mod hash_set;
mod list;
mod list_optimized;
mod persistent_stack;
mod set;
mod stack;
mod tree;

pub use array_stack::*;
pub use btree_set::*;
pub use hash_set::*;
pub use list::*;
pub use list_optimized::List as ListOptimized;
pub use persistent_stack::*;
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
