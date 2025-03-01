use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{Deque, DequeError};
use rust_fp_categories::Empty;

/// An asynchronous implementation of a double-ended queue (deque) using Tokio.
///
/// This implementation wraps another Deque implementation and provides asynchronous
/// access to it using Tokio's synchronization primitives.
///
/// Tokioを使用した両端キュー（deque）の非同期実装。
///
/// この実装は、別のDeque実装をラップし、Tokioの同期プリミティブを使用して
/// 非同期アクセスを提供します。これにより、非同期コードでも効率的にdequeを
/// 操作することができます。
///
/// Time complexity:
/// - All operations have the same time complexity as the wrapped deque implementation,
///   plus the overhead of asynchronous locking.
#[derive(Debug, Clone)]
pub struct TokioDeque<D> {
    inner: Arc<Mutex<D>>,
}

impl<D> TokioDeque<D> {
    /// Creates a new TokioDeque wrapping the given deque.
    pub fn new(deque: D) -> Self {
        TokioDeque {
            inner: Arc::new(Mutex::new(deque)),
        }
    }

    /// Asynchronously gets a reference to the inner deque.
    pub async fn get_inner(&self) -> tokio::sync::MutexGuard<'_, D> {
        self.inner.lock().await
    }

    /// Asynchronously push a value to the front of the deque.
    pub async fn push_front_async<A: Clone>(&self, value: A) -> Self
    where
        D: Deque<A> + Clone,
    {
        let mut inner = self.inner.lock().await;
        let new_inner = (*inner).clone().push_front(value);
        *inner = new_inner;
        self.clone()
    }

    /// Asynchronously push a value to the back of the deque.
    pub async fn push_back_async<A: Clone>(&self, value: A) -> Self
    where
        D: Deque<A> + Clone,
    {
        let mut inner = self.inner.lock().await;
        let new_inner = (*inner).clone().push_back(value);
        *inner = new_inner;
        self.clone()
    }

    /// Asynchronously pop a value from the front of the deque.
    pub async fn pop_front_async<A: Clone>(&self) -> Result<(A, Self), DequeError>
    where
        D: Deque<A> + Clone,
    {
        let mut inner = self.inner.lock().await;
        match (*inner).clone().pop_front() {
            Ok((value, new_inner)) => {
                *inner = new_inner;
                Ok((value, self.clone()))
            }
            Err(e) => Err(e),
        }
    }

    /// Asynchronously pop a value from the back of the deque.
    pub async fn pop_back_async<A: Clone>(&self) -> Result<(A, Self), DequeError>
    where
        D: Deque<A> + Clone,
    {
        let mut inner = self.inner.lock().await;
        match (*inner).clone().pop_back() {
            Ok((value, new_inner)) => {
                *inner = new_inner;
                Ok((value, self.clone()))
            }
            Err(e) => Err(e),
        }
    }

    /// Asynchronously peek at the front value of the deque.
    pub async fn peek_front_async<A: Clone>(&self) -> Result<A, DequeError>
    where
        D: Deque<A>,
    {
        let inner = self.inner.lock().await;
        inner.peek_front()
    }

    /// Asynchronously peek at the back value of the deque.
    pub async fn peek_back_async<A: Clone>(&self) -> Result<A, DequeError>
    where
        D: Deque<A>,
    {
        let inner = self.inner.lock().await;
        inner.peek_back()
    }

    /// Asynchronously get the size of the deque.
    pub async fn size_async<A: Clone>(&self) -> usize
    where
        D: Deque<A>,
    {
        let inner = self.inner.lock().await;
        inner.size()
    }

    /// Asynchronously check if the deque is empty.
    pub async fn is_empty_async(&self) -> bool
    where
        D: Empty,
    {
        let inner = self.inner.lock().await;
        inner.is_empty()
    }
}

impl<D: Empty + Clone> Empty for TokioDeque<D> {
    fn empty() -> Self {
        TokioDeque::new(D::empty())
    }

    fn is_empty(&self) -> bool {
        // This is a blocking operation, but it's required by the Empty trait.
        // For asynchronous code, use is_empty_async instead.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { self.inner.lock().await.is_empty() })
    }
}

impl<A: Clone, D: Deque<A> + Clone> Deque<A> for TokioDeque<D> {
    fn push_front(self, value: A) -> Self {
        // This is a blocking operation, but it's required by the Deque trait.
        // For asynchronous code, use push_front_async instead.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { self.push_front_async(value).await })
    }

    fn push_back(self, value: A) -> Self {
        // This is a blocking operation, but it's required by the Deque trait.
        // For asynchronous code, use push_back_async instead.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { self.push_back_async(value).await })
    }

    fn pop_front(self) -> Result<(A, Self), DequeError> {
        // This is a blocking operation, but it's required by the Deque trait.
        // For asynchronous code, use pop_front_async instead.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { self.pop_front_async().await })
    }

    fn pop_back(self) -> Result<(A, Self), DequeError> {
        // This is a blocking operation, but it's required by the Deque trait.
        // For asynchronous code, use pop_back_async instead.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { self.pop_back_async().await })
    }

    fn peek_front(&self) -> Result<A, DequeError> {
        // This is a blocking operation, but it's required by the Deque trait.
        // For asynchronous code, use peek_front_async instead.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { self.peek_front_async().await })
    }

    fn peek_back(&self) -> Result<A, DequeError> {
        // This is a blocking operation, but it's required by the Deque trait.
        // For asynchronous code, use peek_back_async instead.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { self.peek_back_async().await })
    }

    fn size(&self) -> usize {
        // This is a blocking operation, but it's required by the Deque trait.
        // For asynchronous code, use size_async instead.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async { self.size_async::<A>().await })
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        // Convert the iterator to a Vec
        let items: Vec<A> = iter.into_iter().collect();

        // Create an empty deque
        let mut deque = TokioDeque::empty();

        // Add all items to the deque
        for item in items {
            deque = deque.push_back(item);
        }

        deque
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArrayDeque, Deque};

    #[tokio::test]
    async fn test_empty_deque_async() {
        let deque: TokioDeque<ArrayDeque<i32>> = TokioDeque::empty();
        assert!(deque.is_empty_async().await);
        assert_eq!(deque.size_async::<i32>().await, 0);
        assert!(deque.peek_front_async::<i32>().await.is_err());
        assert!(deque.peek_back_async::<i32>().await.is_err());
    }

    #[tokio::test]
    async fn test_push_front_pop_front_async() {
        let deque: TokioDeque<ArrayDeque<i32>> = TokioDeque::empty();
        let deque = deque.push_front_async(1).await;
        let deque = deque.push_front_async(2).await;
        let deque = deque.push_front_async(3).await;

        assert_eq!(deque.size_async::<i32>().await, 3);
        assert!(!deque.is_empty_async().await);

        let (value, deque) = deque.pop_front_async::<i32>().await.unwrap();
        assert_eq!(value, 3);
        assert_eq!(deque.size_async::<i32>().await, 2);

        let (value, deque) = deque.pop_front_async::<i32>().await.unwrap();
        assert_eq!(value, 2);
        assert_eq!(deque.size_async::<i32>().await, 1);

        let (value, deque) = deque.pop_front_async::<i32>().await.unwrap();
        assert_eq!(value, 1);
        assert_eq!(deque.size_async::<i32>().await, 0);
        assert!(deque.is_empty_async().await);

        assert!(deque.pop_front_async::<i32>().await.is_err());
    }

    #[tokio::test]
    async fn test_push_back_pop_back_async() {
        let deque: TokioDeque<ArrayDeque<i32>> = TokioDeque::empty();
        let deque = deque.push_back_async(1).await;
        let deque = deque.push_back_async(2).await;
        let deque = deque.push_back_async(3).await;

        assert_eq!(deque.size_async::<i32>().await, 3);
        assert!(!deque.is_empty_async().await);

        let (value, deque) = deque.pop_back_async::<i32>().await.unwrap();
        assert_eq!(value, 3);
        assert_eq!(deque.size_async::<i32>().await, 2);

        let (value, deque) = deque.pop_back_async::<i32>().await.unwrap();
        assert_eq!(value, 2);
        assert_eq!(deque.size_async::<i32>().await, 1);

        let (value, deque) = deque.pop_back_async::<i32>().await.unwrap();
        assert_eq!(value, 1);
        assert_eq!(deque.size_async::<i32>().await, 0);
        assert!(deque.is_empty_async().await);

        assert!(deque.pop_back_async::<i32>().await.is_err());
    }

    #[tokio::test]
    async fn test_push_front_pop_back_async() {
        let deque: TokioDeque<ArrayDeque<i32>> = TokioDeque::empty();
        let deque = deque.push_front_async(1).await;
        let deque = deque.push_front_async(2).await;
        let deque = deque.push_front_async(3).await;

        let (value, deque) = deque.pop_back_async::<i32>().await.unwrap();
        assert_eq!(value, 1);

        let (value, deque) = deque.pop_back_async::<i32>().await.unwrap();
        assert_eq!(value, 2);

        let (value, deque) = deque.pop_back_async::<i32>().await.unwrap();
        assert_eq!(value, 3);

        assert!(deque.is_empty_async().await);
    }

    #[tokio::test]
    async fn test_push_back_pop_front_async() {
        let deque: TokioDeque<ArrayDeque<i32>> = TokioDeque::empty();
        let deque = deque.push_back_async(1).await;
        let deque = deque.push_back_async(2).await;
        let deque = deque.push_back_async(3).await;

        let (value, deque) = deque.pop_front_async::<i32>().await.unwrap();
        assert_eq!(value, 1);

        let (value, deque) = deque.pop_front_async::<i32>().await.unwrap();
        assert_eq!(value, 2);

        let (value, deque) = deque.pop_front_async::<i32>().await.unwrap();
        assert_eq!(value, 3);

        assert!(deque.is_empty_async().await);
    }

    #[tokio::test]
    async fn test_peek_async() {
        let deque: TokioDeque<ArrayDeque<i32>> = TokioDeque::empty();
        let deque = deque.push_front_async(1).await;
        let deque = deque.push_back_async(2).await;

        assert_eq!(deque.peek_front_async::<i32>().await.unwrap(), 1);
        assert_eq!(deque.peek_back_async::<i32>().await.unwrap(), 2);

        let (_, deque) = deque.pop_front_async::<i32>().await.unwrap();
        assert_eq!(deque.peek_front_async::<i32>().await.unwrap(), 2);
        assert_eq!(deque.peek_back_async::<i32>().await.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_mixed_operations_async() {
        let deque: TokioDeque<ArrayDeque<i32>> = TokioDeque::empty();

        // Push elements from both ends
        let deque = deque.push_front_async(1).await;
        let deque = deque.push_back_async(2).await;
        let deque = deque.push_front_async(3).await;
        let deque = deque.push_back_async(4).await;

        // Expected order: [3, 1, 2, 4]
        assert_eq!(deque.size_async::<i32>().await, 4);

        // Check peek operations
        assert_eq!(deque.peek_front_async::<i32>().await.unwrap(), 3);
        assert_eq!(deque.peek_back_async::<i32>().await.unwrap(), 4);

        // Pop from front
        let (value, deque) = deque.pop_front_async::<i32>().await.unwrap();
        assert_eq!(value, 3);

        // Pop from back
        let (value, deque) = deque.pop_back_async::<i32>().await.unwrap();
        assert_eq!(value, 4);

        // Expected order: [1, 2]
        assert_eq!(deque.size_async::<i32>().await, 2);
        assert_eq!(deque.peek_front_async::<i32>().await.unwrap(), 1);
        assert_eq!(deque.peek_back_async::<i32>().await.unwrap(), 2);
    }

    // Tests for the synchronous Deque trait implementation

    #[test]
    fn test_empty_deque() {
        let deque: TokioDeque<ArrayDeque<i32>> = TokioDeque::empty();
        assert!(deque.is_empty());
        assert_eq!(deque.size(), 0);
        assert!(deque.peek_front().is_err());
        assert!(deque.peek_back().is_err());
    }

    #[test]
    fn test_push_front_pop_front() {
        let mut deque: TokioDeque<ArrayDeque<i32>> = TokioDeque::empty();
        deque = deque.push_front(1);
        deque = deque.push_front(2);
        deque = deque.push_front(3);

        assert_eq!(deque.size(), 3);
        assert!(!deque.is_empty());

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 3);
        assert_eq!(new_deque.size(), 2);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 2);
        assert_eq!(new_deque.size(), 1);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 1);
        assert_eq!(new_deque.size(), 0);
        assert!(new_deque.is_empty());
        deque = new_deque;

        assert!(deque.pop_front().is_err());
    }

    #[test]
    fn test_from_iter() {
        let mut deque = TokioDeque::<ArrayDeque<i32>>::from_iter(vec![1, 2, 3]);

        assert_eq!(deque.size(), 3);

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 1);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 2);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 3);
        deque = new_deque;

        assert!(deque.is_empty());
    }
}
