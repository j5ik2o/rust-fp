use std::future::Future;
use std::pin::Pin;

use rust_fp_categories::r#async::AsyncEmpty;
use rust_fp_categories::Empty;

use crate::QueueError;

/// A trait for asynchronous persistent queue data structures.
///
/// An async queue is a first-in-first-out (FIFO) data structure with asynchronous operations.
/// All operations create a new queue instance, preserving the original.
pub trait AsyncQueue<A: Clone>: Empty + AsyncEmpty {
    /// Adds an element to the end of the queue asynchronously.
    ///
    /// Returns a future that resolves to a new queue with the element added.
    fn enqueue<'a>(&'a self, value: A) -> Pin<Box<dyn Future<Output = Self> + 'a>>;

    /// Removes an element from the front of the queue asynchronously.
    ///
    /// Returns a future that resolves to a tuple containing the removed element and the new queue,
    /// or an error if the queue is empty.
    fn dequeue<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<(A, Self), QueueError>> + 'a>>
    where
        Self: Sized;

    /// Returns the element at the front of the queue without removing it.
    ///
    /// Returns an error if the queue is empty.
    fn peek(&self) -> Result<A, QueueError>;

    /// Returns the number of elements in the queue.
    fn size(&self) -> usize;

    /// Returns true if the queue is empty.
    fn is_empty(&self) -> bool;

    /// Creates a queue from an iterator asynchronously.
    fn from_iter<'a, T: IntoIterator<Item = A> + 'a>(
        iter: T,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a>>;
}
