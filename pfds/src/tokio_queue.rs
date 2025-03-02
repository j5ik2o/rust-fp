use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

use futures::future::{ready, Ready};
use tokio::sync::Mutex;

use rust_fp_categories::{Empty, Functor};

use crate::{AsyncQueue, QueueError};

/// A Tokio-based asynchronous queue implementation.
///
/// This implementation uses a vector wrapped in an Arc<Mutex<>> to provide
/// thread-safe asynchronous operations. All operations create a new queue
/// instance, preserving the original.
///
/// Time complexity:
/// - enqueue: O(n) - due to cloning the entire vector
/// - dequeue: O(n) - due to cloning the entire vector
/// - peek: O(1)
/// - size: O(1)
#[derive(Debug, Clone)]
pub struct TokioQueue<A> {
    elements: Arc<Mutex<Vec<A>>>,
}

impl<A> TokioQueue<A> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        TokioQueue {
            elements: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl<A> Empty for TokioQueue<A> {
    fn empty() -> Self {
        TokioQueue::new()
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

impl<A: Clone + Send + Sync + 'static> Functor for TokioQueue<A> {
    type Elm = A;
    type M<B: Clone> = TokioQueue<B>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
        B: Clone,
    {
        // This is a blocking operation, but it's necessary for the Functor trait.
        // For truly asynchronous mapping, an async_fmap method could be added.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let elements = self.elements.lock().await;
            let mut new_elements = Vec::with_capacity(elements.len());
            
            for item in elements.iter() {
                new_elements.push(f(item));
            }
            
            TokioQueue {
                elements: Arc::new(Mutex::new(new_elements)),
            }
        })
    }
}

impl<A: Clone + Send + 'static> TokioQueue<A> {
    /// Asynchronously checks if the queue is empty.
    pub async fn async_is_empty(&self) -> bool {
        let elements = self.elements.lock().await;
        elements.is_empty()
    }

    /// Asynchronously gets the size of the queue.
    pub async fn async_size(&self) -> usize {
        let elements = self.elements.lock().await;
        elements.len()
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncQueue<A> for TokioQueue<A> {
    fn enqueue<'a>(&'a self, value: A) -> Pin<Box<dyn Future<Output = Self> + 'a>> {
        let elements_clone = self.elements.clone();
        Box::pin(async move {
            let elements = elements_clone.lock().await;
            let mut new_elements = elements.clone();
            new_elements.push(value);
            drop(elements);

            TokioQueue {
                elements: Arc::new(Mutex::new(new_elements)),
            }
        })
    }

    fn dequeue<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<(A, Self), QueueError>> + 'a>> {
        let elements_clone = self.elements.clone();
        Box::pin(async move {
            let elements = elements_clone.lock().await;

            if elements.is_empty() {
                return Err(QueueError::EmptyQueueError);
            }

            let value = elements[0].clone();
            let mut new_elements = elements.clone();
            new_elements.remove(0);
            drop(elements);

            Ok((
                value,
                TokioQueue {
                    elements: Arc::new(Mutex::new(new_elements)),
                },
            ))
        })
    }

    fn peek(&self) -> Result<A, QueueError> {
        // This is a blocking operation, but it's necessary for the AsyncQueue trait.
        // For truly asynchronous peeking, use the async_peek method.
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let elements = self.elements.lock().await;
            if elements.is_empty() {
                return Err(QueueError::EmptyQueueError);
            }

            // Now we can return a cloned value, which is more appropriate for this design
            Ok(elements[0].clone())
        })
    }

    fn size(&self) -> usize {
        // This is a blocking operation, but it's necessary for the AsyncQueue trait.
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
            TokioQueue {
                elements: Arc::new(Mutex::new(elements)),
            }
        })
    }
}

impl<A: Clone + Send + Sync + 'static> TokioQueue<A> {
    /// Asynchronously peeks at the front element of the queue.
    pub async fn async_peek(&self) -> Result<A, QueueError> {
        let elements = self.elements.lock().await;
        if elements.is_empty() {
            return Err(QueueError::EmptyQueueError);
        }

        Ok(elements[0].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsyncQueue;

    #[tokio::test]
    async fn test_empty_queue() {
        let queue: TokioQueue<i32> = TokioQueue::empty();
        assert!(queue.async_is_empty().await);
        assert_eq!(queue.async_size().await, 0);
        assert!(queue.async_peek().await.is_err());
    }

    #[tokio::test]
    async fn test_enqueue_dequeue() {
        let queue = TokioQueue::empty();
        let queue = queue.enqueue(1).await;
        let queue = queue.enqueue(2).await;
        let queue = queue.enqueue(3).await;

        assert_eq!(queue.async_size().await, 3);
        assert!(!queue.async_is_empty().await);

        let (value, queue) = queue.dequeue().await.unwrap();
        assert_eq!(value, 1);
        assert_eq!(queue.async_size().await, 2);

        let (value, queue) = queue.dequeue().await.unwrap();
        assert_eq!(value, 2);
        assert_eq!(queue.async_size().await, 1);

        let (value, queue) = queue.dequeue().await.unwrap();
        assert_eq!(value, 3);
        assert_eq!(queue.async_size().await, 0);
        assert!(queue.async_is_empty().await);

        assert!(queue.dequeue().await.is_err());
    }

    #[tokio::test]
    async fn test_peek() {
        let queue = TokioQueue::empty();
        let queue = queue.enqueue(1).await;
        let queue = queue.enqueue(2).await;

        assert_eq!(queue.async_peek().await.unwrap(), 1);

        let (_, queue) = queue.dequeue().await.unwrap();
        assert_eq!(queue.async_peek().await.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_from_iter() {
        let queue = TokioQueue::from_iter(vec![1, 2, 3]).await;

        assert_eq!(queue.async_size().await, 3);

        let (value, queue) = queue.dequeue().await.unwrap();
        assert_eq!(value, 1);

        let (value, queue) = queue.dequeue().await.unwrap();
        assert_eq!(value, 2);

        let (value, _) = queue.dequeue().await.unwrap();
        assert_eq!(value, 3);
    }

    #[tokio::test]
    async fn test_large_queue() {
        let mut queue = TokioQueue::empty();
        for i in 0..100 {
            queue = queue.enqueue(i).await;
        }

        assert_eq!(queue.async_size().await, 100);

        for i in 0..100 {
            let (value, new_queue) = queue.dequeue().await.unwrap();
            assert_eq!(value, i);
            queue = new_queue;
        }

        assert!(queue.async_is_empty().await);
    }

    #[tokio::test]
    async fn test_clone() {
        let queue1 = TokioQueue::from_iter(vec![1, 2, 3]).await;
        let queue2 = queue1.clone();

        // Both queues should have the same size
        assert_eq!(queue1.async_size().await, queue2.async_size().await);

        // Modifying one queue should not affect the other
        let (_, queue1_new) = queue1.dequeue().await.unwrap();
        assert_eq!(queue1_new.async_size().await, 2);
        assert_eq!(queue2.async_size().await, 3);
    }
}
