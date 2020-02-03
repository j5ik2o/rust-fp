extern crate rust_fp_categories;

pub mod stack;
pub mod list;
pub mod set;
pub mod tree;

#[cfg(test)]
mod tests {
    use list::List;
    use stack::Stack;
    use rust_fp_categories::empty::*;
    use rust_fp_categories::bind::*;
    use rust_fp_categories::functor::*;

    #[test]
    fn it_works() {
        let list1: List<i32> = List::empty().cons(30).cons(20).cons(10);
        println!("{:?}", list1);
        let list2 = list1.bind(|x| List::empty().cons(x*2).fmap(|x| x - 1));
        println!("{:?}", list2);
    }
}
