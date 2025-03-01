use rust_fp_categories::Empty;

/// Error type for Queue operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueError {
    /// Error when trying to dequeue from an empty queue
    EmptyQueueError,
}

/// A trait for persistent queue data structures.
/// 
/// A queue is a first-in-first-out (FIFO) data structure.
/// All operations create a new queue instance, preserving the original.
pub trait Queue<A: Clone>: Empty {
    /// Adds an element to the end of the queue.
    /// 
    /// Returns a new queue with the element added.
    fn enqueue(self, value: A) -> Self;
    
    /// Removes an element from the front of the queue.
    /// 
    /// Returns a tuple containing the removed element and the new queue,
    /// or an error if the queue is empty.
    fn dequeue(self) -> Result<(A, Self), QueueError>;
    
    /// Returns a reference to the element at the front of the queue without removing it.
    /// 
    /// Returns an error if the queue is empty.
    fn peek(&self) -> Result<&A, QueueError>;
    
    /// Returns the number of elements in the queue.
    fn size(&self) -> usize;
    
    /// Returns true if the queue is empty.
    fn is_empty(&self) -> bool;
    
    /// Creates a queue from an iterator.
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
