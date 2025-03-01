use std::future::Future;
use std::pin::Pin;

use crate::DequeError;

/// A trait for asynchronous double-ended queue (deque) data structures.
///
/// This trait defines the operations that can be performed on an asynchronous deque.
/// All operations return futures that resolve to the result of the operation.
pub trait AsyncDeque<A: Clone + Send + Sync + 'static>: Send + Sync {
    /// Adds an element to the front of the deque asynchronously.
    ///
    /// Returns a future that resolves to a new deque with the element added.
    fn push_front<'a>(&'a self, value: A) -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        Self: Sized;

    /// Adds an element to the back of the deque asynchronously.
    ///
    /// Returns a future that resolves to a new deque with the element added.
    fn push_back<'a>(&'a self, value: A) -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        Self: Sized;

    /// Removes an element from the front of the deque asynchronously.
    ///
    /// Returns a future that resolves to a tuple containing the removed element and the new deque,
    /// or an error if the deque is empty.
    fn pop_front<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<(A, Self), DequeError>> + 'a>>
    where
        Self: Sized;

    /// Removes an element from the back of the deque asynchronously.
    ///
    /// Returns a future that resolves to a tuple containing the removed element and the new deque,
    /// or an error if the deque is empty.
    fn pop_back<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<(A, Self), DequeError>> + 'a>>
    where
        Self: Sized;

    /// Returns the element at the front of the deque without removing it.
    ///
    /// This is a synchronous operation that may block. For a truly asynchronous
    /// operation, implementations should provide an async_peek_front method.
    fn peek_front(&self) -> Result<A, DequeError>;

    /// Returns the element at the back of the deque without removing it.
    ///
    /// This is a synchronous operation that may block. For a truly asynchronous
    /// operation, implementations should provide an async_peek_back method.
    fn peek_back(&self) -> Result<A, DequeError>;

    /// Returns the number of elements in the deque.
    ///
    /// This is a synchronous operation that may block. For a truly asynchronous
    /// operation, implementations should provide an async_size method.
    fn size(&self) -> usize;

    /// Checks if the deque is empty.
    ///
    /// This is a synchronous operation that may block. For a truly asynchronous
    /// operation, implementations should provide an async_is_empty method.
    fn is_empty(&self) -> bool;

    /// Creates a deque from an iterator asynchronously.
    ///
    /// Returns a future that resolves to a new deque containing the elements from the iterator.
    fn from_iter<'a, T: IntoIterator<Item = A> + 'a>(
        iter: T,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        Self: Sized;
}
