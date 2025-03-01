use rust_fp_categories::Empty;

/// Error type for Deque operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DequeError {
    /// Error when trying to remove from an empty deque
    EmptyDequeError,
}

/// A trait for persistent double-ended queue (deque) data structures.
///
/// A deque allows adding and removing elements from both ends.
/// All operations create a new deque instance, preserving the original.
pub trait Deque<A: Clone>: Empty {
    /// Adds an element to the front of the deque.
    ///
    /// Returns a new deque with the element added.
    fn push_front(self, value: A) -> Self;
    
    /// Adds an element to the back of the deque.
    ///
    /// Returns a new deque with the element added.
    fn push_back(self, value: A) -> Self;
    
    /// Removes an element from the front of the deque.
    ///
    /// Returns a tuple containing the removed element and the new deque,
    /// or an error if the deque is empty.
    fn pop_front(self) -> Result<(A, Self), DequeError> where Self: Sized;
    
    /// Removes an element from the back of the deque.
    ///
    /// Returns a tuple containing the removed element and the new deque,
    /// or an error if the deque is empty.
    fn pop_back(self) -> Result<(A, Self), DequeError> where Self: Sized;
    
    /// Returns the element at the front of the deque without removing it.
    ///
    /// Returns an error if the deque is empty.
    fn peek_front(&self) -> Result<A, DequeError>;
    
    /// Returns the element at the back of the deque without removing it.
    ///
    /// Returns an error if the deque is empty.
    fn peek_back(&self) -> Result<A, DequeError>;
    
    /// Returns the number of elements in the deque.
    fn size(&self) -> usize;
    
    /// Creates a deque from an iterator.
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
