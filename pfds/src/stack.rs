use std::rc::Rc;

#[derive(Debug)]
pub enum StackError {
    NoSuchElementError,
    IndexOutOfRange,
}

pub trait Stack<A> {
    fn cons(self, value: A) -> Self;
    fn head(&self) -> Result<&A, StackError>;
    fn tail(&self) -> Rc<Self>;
    fn size(&self) -> usize;
    fn update(self, index: u32, new_value: A) -> Result<Self, StackError>
    where
        Self: Sized;
    fn get(&self, i: u32) -> Result<&A, StackError>;
}
