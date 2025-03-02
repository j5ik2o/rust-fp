use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::Mutex;

use rust_fp_categories::r#async::{
    AsyncApplicative, AsyncApply, AsyncBind, AsyncEmpty, AsyncFoldable, AsyncFunctor, AsyncMonad,
    AsyncPure,
};
use rust_fp_categories::Empty;

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

        // Use tokio::task::block_in_place to avoid creating a new runtime
        // This works both inside and outside of a Tokio runtime
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                let elements = self.elements.lock().await;
                elements.is_empty()
            })
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncEmpty for TokioQueue<A> {
    fn empty<'a>() -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        Self: Sized + 'a,
    {
        Box::pin(async move { TokioQueue::new() })
    }

    fn is_empty<'a>(&'a self) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
        let self_clone = self.clone();
        Box::pin(async move { self_clone.async_is_empty().await })
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

impl<A: Clone + Send + Sync + 'static> AsyncFunctor for TokioQueue<A> {
    type Elm = A;
    type M<B: Clone + Send + Sync + 'static> = TokioQueue<B>;

    fn fmap<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        f: F,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> B + Send + Sync + 'a,
    {
        let self_clone = self.clone();
        Box::pin(async move {
            let mut result_queue = <TokioQueue<B> as rust_fp_categories::Empty>::empty();
            let mut current_queue = self_clone;

            while !(AsyncEmpty::is_empty(&current_queue).await) {
                match current_queue.dequeue().await {
                    Ok((value, new_queue)) => {
                        let mapped_value = f(&value);
                        result_queue = result_queue.enqueue(mapped_value).await;
                        current_queue = new_queue;
                    }
                    Err(_) => break,
                }
            }

            result_queue
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncPure for TokioQueue<A> {
    type Elm = A;

    fn pure<'a>(value: Self::Elm) -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        Self: Sized + 'a,
    {
        Box::pin(async move {
            let empty_queue = <TokioQueue<_> as rust_fp_categories::Empty>::empty();
            empty_queue.enqueue(value).await
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncApply for TokioQueue<A> {
    fn ap<'a, B: Clone + Send + Sync + 'static, F: Clone + Send + Sync + 'static>(
        &'a self,
        fs: &'a Self::M<F>,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> B + Send + Sync + 'a,
    {
        let self_clone = self.clone();
        let fs_clone = fs.clone();
        Box::pin(async move {
            let mut result_queue = <TokioQueue<B> as rust_fp_categories::Empty>::empty();
            let mut fs_queue = fs_clone;

            while !(AsyncEmpty::is_empty(&fs_queue).await) {
                match fs_queue.dequeue().await {
                    Ok((f, new_fs_queue)) => {
                        let mut current_queue = self_clone.clone();
                        while !(AsyncEmpty::is_empty(&current_queue).await) {
                            match current_queue.dequeue().await {
                                Ok((value, new_queue)) => {
                                    let applied_value = f(&value);
                                    result_queue = result_queue.enqueue(applied_value).await;
                                    current_queue = new_queue;
                                }
                                Err(_) => break,
                            }
                        }
                        fs_queue = new_fs_queue;
                    }
                    Err(_) => break,
                }
            }

            result_queue
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncBind for TokioQueue<A> {
    type Elm = A;
    type M<B: Clone + Send + Sync + 'static> = TokioQueue<B>;

    fn bind<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        f: F,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>> + Send + Sync + 'a,
    {
        let self_clone = self.clone();
        Box::pin(async move {
            let mut result_queue = <TokioQueue<B> as rust_fp_categories::Empty>::empty();
            let mut current_queue = self_clone;

            while !(AsyncEmpty::is_empty(&current_queue).await) {
                match current_queue.dequeue().await {
                    Ok((value, new_queue)) => {
                        let bound_queue = f(&value).await;

                        // Concatenate the bound queue to the result queue
                        let mut bound_queue_clone = bound_queue;
                        while !(AsyncEmpty::is_empty(&bound_queue_clone).await) {
                            match bound_queue_clone.dequeue().await {
                                Ok((bound_value, new_bound_queue)) => {
                                    result_queue = result_queue.enqueue(bound_value).await;
                                    bound_queue_clone = new_bound_queue;
                                }
                                Err(_) => break,
                            }
                        }

                        current_queue = new_queue;
                    }
                    Err(_) => break,
                }
            }

            result_queue
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncApplicative for TokioQueue<A> {}

impl<A: Clone + Send + Sync + 'static> AsyncMonad for TokioQueue<A> {}

impl<A: Clone + Send + Sync + 'static> AsyncFoldable for TokioQueue<A> {
    type Elm = A;

    fn fold_left<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        b: B,
        f: F,
    ) -> Pin<Box<dyn Future<Output = B> + 'a>>
    where
        F: Fn(B, &Self::Elm) -> Pin<Box<dyn Future<Output = B> + 'a>> + Send + Sync + 'a,
    {
        let self_clone = self.clone();
        Box::pin(async move {
            let mut result = b;
            let mut current_queue = self_clone;

            while !(AsyncEmpty::is_empty(&current_queue).await) {
                match current_queue.dequeue().await {
                    Ok((value, new_queue)) => {
                        result = f(result, &value).await;
                        current_queue = new_queue;
                    }
                    Err(_) => break,
                }
            }

            result
        })
    }

    fn fold_right<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        b: B,
        f: F,
    ) -> Pin<Box<dyn Future<Output = B> + 'a>>
    where
        F: Fn(&Self::Elm, B) -> Pin<Box<dyn Future<Output = B> + 'a>> + Send + Sync + 'a,
    {
        let self_clone = self.clone();
        Box::pin(async move {
            // 右畳み込みは左畳み込みを使って実装
            // 要素を逆順にして左畳み込みを適用
            let mut elements = Vec::new();
            let mut current_queue = self_clone;

            while !(AsyncEmpty::is_empty(&current_queue).await) {
                match current_queue.dequeue().await {
                    Ok((value, new_queue)) => {
                        elements.push(value);
                        current_queue = new_queue;
                    }
                    Err(_) => break,
                }
            }

            let mut result = b;
            for elem in elements.iter().rev() {
                result = f(elem, result).await;
            }

            result
        })
    }
}

// Implementation of AsyncEmpty is already provided in the file

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
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                let elements = self.elements.lock().await;
                if elements.is_empty() {
                    return Err(QueueError::EmptyQueueError);
                }

                // Now we can return a cloned value, which is more appropriate for this design
                Ok(elements[0].clone())
            })
        })
    }

    fn size(&self) -> usize {
        // This is a blocking operation, but it's necessary for the AsyncQueue trait.
        // For truly asynchronous size checking, use the async_size method.
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                let elements = self.elements.lock().await;
                elements.len()
            })
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
        let queue: TokioQueue<i32> = <TokioQueue<i32> as rust_fp_categories::Empty>::empty();
        assert!(queue.async_is_empty().await);
        assert_eq!(queue.async_size().await, 0);
        assert!(queue.async_peek().await.is_err());
    }

    #[tokio::test]
    async fn test_enqueue_dequeue() {
        let queue = <TokioQueue<i32> as rust_fp_categories::Empty>::empty();
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
        let queue = <TokioQueue<i32> as rust_fp_categories::Empty>::empty();
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
        let mut queue = <TokioQueue<i32> as rust_fp_categories::Empty>::empty();
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
