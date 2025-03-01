use std::rc::Rc;

/// Error type for Queue operations.
#[derive(Debug, PartialEq, Eq)]
pub enum QueueError {
    /// Error when trying to access an element from an empty queue.
    EmptyQueueError,
}

/// Queue trait represents a first-in-first-out (FIFO) data structure.
/// 
/// A queue is a collection of elements that supports adding elements to the back
/// and removing elements from the front.
pub trait Queue<A>: Sized {
    /// Adds an element to the back of the queue.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to add to the queue.
    /// 
    /// # Returns
    /// 
    /// A new queue with the value added.
    fn enqueue(self, value: A) -> Self;
    
    /// Removes and returns the front element of the queue.
    /// 
    /// # Returns
    /// 
    /// A tuple containing the front element and the new queue without that element,
    /// or an error if the queue is empty.
    fn dequeue(self) -> Result<(A, Self), QueueError>;
    
    /// Returns a reference to the front element of the queue without removing it.
    /// 
    /// # Returns
    /// 
    /// A reference to the front element, or an error if the queue is empty.
    fn peek(&self) -> Result<&A, QueueError>;
    
    /// Returns the number of elements in the queue.
    /// 
    /// # Returns
    /// 
    /// The number of elements in the queue.
    fn size(&self) -> usize;
    
    /// Checks if the queue is empty.
    /// 
    /// # Returns
    /// 
    /// `true` if the queue is empty, `false` otherwise.
    fn is_empty(&self) -> bool;
    
    /// Creates a queue from an iterator.
    /// 
    /// # Arguments
    /// 
    /// * `iter` - An iterator that yields elements to add to the queue.
    /// 
    /// # Returns
    /// 
    /// A new queue containing all elements from the iterator.
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
