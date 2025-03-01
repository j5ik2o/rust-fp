use std::rc::Rc;

#[derive(Debug)]
pub enum StackError {
    NoSuchElementError,
    IndexOutOfRangeError,
    RcUnwrapError,
}

/// Stack trait represents a stack data structure.
///
/// A stack is a last-in-first-out (LIFO) data structure.
pub trait Stack<A> {
    /// Adds a new element to the top of the stack.
    fn cons(self, value: A) -> Self;

    /// Returns a reference to the top element of the stack.
    ///
    /// Returns an error if the stack is empty.
    fn head(&self) -> Result<&A, StackError>;

    /// Returns a reference to the top element of the stack without removing it.
    ///
    /// This is similar to `head` but makes the intent clearer.
    /// Returns an error if the stack is empty.
    fn peek(&self) -> Result<&A, StackError>;

    /// Returns the stack without its top element.
    fn tail(&self) -> Rc<Self>;

    /// Returns the number of elements in the stack.
    fn size(&self) -> usize;

    /// Checks if the stack is empty.
    fn is_empty(&self) -> bool;

    /// Updates the element at the specified index.
    fn update(self, index: u32, new_value: A) -> Result<Self, StackError>
    where
        Self: Sized;

    /// Returns a reference to the element at the specified index.
    fn get(&self, i: u32) -> Result<&A, StackError>;

    /// Creates a stack from an iterator.
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self
    where
        Self: Sized;

    /// Removes and returns the first element of the stack along with the remaining stack.
    ///
    /// Returns an error if the stack is empty.
    fn uncons(self) -> Result<(A, Self), StackError> where Self: Sized;
}
