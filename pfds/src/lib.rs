extern crate rust_fp_categories;

mod array_queue;
mod array_stack;
mod async_queue;
mod btree_set;
mod hash_set;
mod list;
mod list_optimized;
mod persistent_stack;
mod queue;
mod set;
mod stack;
mod tokio_queue;
mod tree;

pub use array_queue::*;
pub use array_stack::*;
pub use async_queue::*;
pub use btree_set::*;
pub use hash_set::*;
pub use list::*;
pub use list_optimized::List as ListOptimized;
pub use persistent_stack::*;
pub use queue::*;
pub use set::*;
pub use stack::*;
pub use tokio_queue::*;
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
