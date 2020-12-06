use rust_fp_categories::empty::Empty;

pub trait Set<A>: Empty {
    fn insert(self, value: A) -> Self;
    fn member(&self, value: A) -> bool;
    fn size(&self) -> usize;
}
