use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::Mutex;

use rust_fp_categories::Empty;

use crate::{AsyncDeque, DequeError};

/// A Tokio-based asynchronous deque implementation.
///
/// This implementation uses a vector wrapped in an Arc<Mutex<>> to provide
/// thread-safe asynchronous operations. All operations create a new deque
/// instance, preserving the original.
///
/// Time complexity:
/// - push_front: O(n) - due to shifting all elements
/// - push_back: O(1) - amortized
/// - pop_front: O(n) - due to shifting all elements
/// - pop_back: O(1)
/// - peek_front/peek_back: O(1)
/// - size: O(1)
#[derive(Debug, Clone)]
pub struct TokioDeque<A> {
    elements: Arc<Mutex<Vec<A>>>,
}

impl<A> TokioDeque<A> {
    /// Creates a new empty deque.
    pub fn new() -> Self {
        TokioDeque {
            elements: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<A> Empty for TokioDeque<A> {
    fn empty() -> Self {
        TokioDeque::new()
    }

    fn is_empty(&self) -> bool {
        // This is a blocking operation, but it's necessary for the Empty trait.
        // For truly asynchronous checking, use the async_is_empty method.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let elements = self.elements.lock().await;
            elements.is_empty()
        })
    }
}

impl<A: Clone + Send + 'static> TokioDeque<A> {
    /// Asynchronously checks if the deque is empty.
    pub async fn async_is_empty(&self) -> bool {
        let elements = self.elements.lock().await;
        elements.is_empty()
    }

    /// Asynchronously gets the size of the deque.
    pub async fn async_size(&self) -> usize {
        let elements = self.elements.lock().await;
        elements.len()
    }

    /// Asynchronously peeks at the front element of the deque.
    pub async fn async_peek_front(&self) -> Result<A, DequeError> {
        let elements = self.elements.lock().await;
        if elements.is_empty() {
            return Err(DequeError::EmptyDequeError);
        }

        Ok(elements[0].clone())
    }

    /// Asynchronously peeks at the back element of the deque.
    pub async fn async_peek_back(&self) -> Result<A, DequeError> {
        let elements = self.elements.lock().await;
        if elements.is_empty() {
            return Err(DequeError::EmptyDequeError);
        }

        let last_index = elements.len() - 1;
        Ok(elements[last_index].clone())
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncDeque<A> for TokioDeque<A> {
    fn push_front<'a>(&'a self, value: A) -> Pin<Box<dyn Future<Output = Self> + 'a>> {
        let elements_clone = self.elements.clone();
        Box::pin(async move {
            let mut elements = elements_clone.lock().await;
            let mut new_elements = elements.clone();
            new_elements.insert(0, value);
            drop(elements);

            TokioDeque {
                elements: Arc::new(Mutex::new(new_elements)),
            }
        })
    }

    fn push_back<'a>(&'a self, value: A) -> Pin<Box<dyn Future<Output = Self> + 'a>> {
        let elements_clone = self.elements.clone();
        Box::pin(async move {
            let mut elements = elements_clone.lock().await;
            let mut new_elements = elements.clone();
            new_elements.push(value);
            drop(elements);

            TokioDeque {
                elements: Arc::new(Mutex::new(new_elements)),
            }
        })
    }

    fn pop_front<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<(A, Self), DequeError>> + 'a>> {
        let elements_clone = self.elements.clone();
        Box::pin(async move {
            let elements = elements_clone.lock().await;

            if elements.is_empty() {
                return Err(DequeError::EmptyDequeError);
            }

            let value = elements[0].clone();
            let mut new_elements = elements.clone();
            new_elements.remove(0);
            drop(elements);

            Ok((
                value,
                TokioDeque {
                    elements: Arc::new(Mutex::new(new_elements)),
                },
            ))
        })
    }

    fn pop_back<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<(A, Self), DequeError>> + 'a>> {
        let elements_clone = self.elements.clone();
        Box::pin(async move {
            let elements = elements_clone.lock().await;

            if elements.is_empty() {
                return Err(DequeError::EmptyDequeError);
            }

            let last_index = elements.len() - 1;
            let value = elements[last_index].clone();
            let mut new_elements = elements.clone();
            new_elements.pop();
            drop(elements);

            Ok((
                value,
                TokioDeque {
                    elements: Arc::new(Mutex::new(new_elements)),
                },
            ))
        })
    }

    fn peek_front(&self) -> Result<A, DequeError> {
        // This is a blocking operation, but it's necessary for the AsyncDeque trait.
        // For truly asynchronous peeking, use the async_peek_front method.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let elements = self.elements.lock().await;
            if elements.is_empty() {
                return Err(DequeError::EmptyDequeError);
            }

            Ok(elements[0].clone())
        })
    }

    fn peek_back(&self) -> Result<A, DequeError> {
        // This is a blocking operation, but it's necessary for the AsyncDeque trait.
        // For truly asynchronous peeking, use the async_peek_back method.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let elements = self.elements.lock().await;
            if elements.is_empty() {
                return Err(DequeError::EmptyDequeError);
            }

            let last_index = elements.len() - 1;
            Ok(elements[last_index].clone())
        })
    }

    fn size(&self) -> usize {
        // This is a blocking operation, but it's necessary for the AsyncDeque trait.
        // For truly asynchronous size checking, use the async_size method.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let elements = self.elements.lock().await;
            elements.len()
        })
    }

    fn is_empty(&self) -> bool {
        Empty::is_empty(self)
    }

    fn from_iter<'a, T: IntoIterator<Item = A> + 'a>(
        iter: T,
    ) -> Pin<Box<dyn Future<Output = Self> + 'a>> {
        Box::pin(async move {
            let elements: Vec<A> = iter.into_iter().collect();
            TokioDeque {
                elements: Arc::new(Mutex::new(elements)),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsyncDeque;

    #[tokio::test]
    async fn test_empty_deque() {
        let deque: TokioDeque<i32> = TokioDeque::empty();
        assert!(deque.async_is_empty().await);
        assert_eq!(deque.async_size().await, 0);
        assert!(deque.async_peek_front().await.is_err());
        assert!(deque.async_peek_back().await.is_err());
    }

    #[tokio::test]
    async fn test_push_front_pop_front() {
        let deque = TokioDeque::empty();
        let deque = deque.push_front(1).await;
        let deque = deque.push_front(2).await;
        let deque = deque.push_front(3).await;

        assert_eq!(deque.async_size().await, 3);
        assert!(!deque.async_is_empty().await);
        assert_eq!(deque.async_peek_front().await.unwrap(), 3);
        assert_eq!(deque.async_peek_back().await.unwrap(), 1);

        let (value, deque) = deque.pop_front().await.unwrap();
        assert_eq!(value, 3);
        assert_eq!(deque.async_size().await, 2);

        let (value, deque) = deque.pop_front().await.unwrap();
        assert_eq!(value, 2);
        assert_eq!(deque.async_size().await, 1);

        let (value, deque) = deque.pop_front().await.unwrap();
        assert_eq!(value, 1);
        assert_eq!(deque.async_size().await, 0);
        assert!(deque.async_is_empty().await);

        assert!(deque.pop_front().await.is_err());
    }

    #[tokio::test]
    async fn test_push_back_pop_back() {
        let deque = TokioDeque::empty();
        let deque = deque.push_back(1).await;
        let deque = deque.push_back(2).await;
        let deque = deque.push_back(3).await;

        assert_eq!(deque.async_size().await, 3);
        assert!(!deque.async_is_empty().await);
        assert_eq!(deque.async_peek_front().await.unwrap(), 1);
        assert_eq!(deque.async_peek_back().await.unwrap(), 3);

        let (value, deque) = deque.pop_back().await.unwrap();
        assert_eq!(value, 3);
        assert_eq!(deque.async_size().await, 2);

        let (value, deque) = deque.pop_back().await.unwrap();
        assert_eq!(value, 2);
        assert_eq!(deque.async_size().await, 1);

        let (value, deque) = deque.pop_back().await.unwrap();
        assert_eq!(value, 1);
        assert_eq!(deque.async_size().await, 0);
        assert!(deque.async_is_empty().await);

        assert!(deque.pop_back().await.is_err());
    }

    #[tokio::test]
    async fn test_mixed_operations() {
        let deque = TokioDeque::empty();
        let deque = deque.push_front(1).await;
        let deque = deque.push_back(2).await;
        let deque = deque.push_front(3).await;

        assert_eq!(deque.async_size().await, 3);
        assert_eq!(deque.async_peek_front().await.unwrap(), 3);
        assert_eq!(deque.async_peek_back().await.unwrap(), 2);

        let (value, deque) = deque.pop_front().await.unwrap();
        assert_eq!(value, 3);

        let (value, deque) = deque.pop_back().await.unwrap();
        assert_eq!(value, 2);

        assert_eq!(deque.async_size().await, 1);
        assert_eq!(deque.async_peek_front().await.unwrap(), 1);
        assert_eq!(deque.async_peek_back().await.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_from_iter() {
        let deque = TokioDeque::from_iter(vec![1, 2, 3]).await;

        assert_eq!(deque.async_size().await, 3);
        assert_eq!(deque.async_peek_front().await.unwrap(), 1);
        assert_eq!(deque.async_peek_back().await.unwrap(), 3);
    }

    #[tokio::test]
    async fn test_large_deque() {
        let mut deque = TokioDeque::empty();
        for i in 0..100 {
            deque = deque.push_back(i).await;
        }

        assert_eq!(deque.async_size().await, 100);

        for i in 0..50 {
            let (value, new_deque) = deque.pop_front().await.unwrap();
            assert_eq!(value, i);
            deque = new_deque;
        }

        for i in (50..100).rev() {
            let (value, new_deque) = deque.pop_back().await.unwrap();
            assert_eq!(value, i);
            deque = new_deque;
        }

        assert!(deque.async_is_empty().await);
    }

    #[tokio::test]
    async fn test_clone() {
        let deque1 = TokioDeque::from_iter(vec![1, 2, 3]).await;
        let deque2 = deque1.clone();

        // Both deques should have the same size
        assert_eq!(deque1.async_size().await, deque2.async_size().await);

        // Modifying one deque should not affect the other
        let (_, deque1_new) = deque1.pop_front().await.unwrap();
        assert_eq!(deque1_new.async_size().await, 2);
        assert_eq!(deque2.async_size().await, 3);
    }
}
