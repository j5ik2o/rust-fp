#[derive(Debug)]
pub enum StackError {
    NoSuchElementError,
    IndexOutOfRange,
}

pub trait Stack<A> {
    fn cons(&self, value: A) -> Self;
    fn head(&self) -> Result<A, StackError>;
    fn tail(&self) -> Arc<Self>;
    fn size(&self) -> usize;
}

