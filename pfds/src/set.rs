pub trait Set<A> {
    fn insert(self, value: A) -> Self;
    fn member(self, value: A) -> bool;
}
