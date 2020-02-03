extern crate rust_fp_categories;

pub mod stack;
pub mod list;


#[cfg(test)]
mod tests {
    use list::List;
    use stack::Stack;
    use rust_fp_categories::empty::Empty;

    #[test]
    fn it_works() {
        let list1: List<i32> = List::empty().cons(30).cons(20).cons(10);
        println!("{:?}", list1);
    }
}
