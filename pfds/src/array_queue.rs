use std::rc::Rc;

use crate::{Queue, QueueError};
use rust_fp_categories::{Empty, Functor};

/// An array-based queue implementation.
///
/// This implementation uses a vector to store elements in a persistent manner.
/// All operations create a new queue instance, preserving the original.
///
/// Time complexity:
/// - enqueue: O(n) - due to cloning the entire vector
/// - dequeue: O(n) - due to cloning the entire vector
/// - peek: O(1)
/// - size: O(1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayQueue<A> {
    elements: Rc<Vec<A>>,
}

impl<A> ArrayQueue<A> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        ArrayQueue {
            elements: Rc::new(Vec::new()),
        }
    }
}

impl<A> Empty for ArrayQueue<A> {
    fn empty() -> Self {
        ArrayQueue::new()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<A: Clone> Functor for ArrayQueue<A> {
    type Elm = A;
    type M<B: Clone> = ArrayQueue<B>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
        B: Clone,
    {
        let mut new_elements = Vec::with_capacity(self.size());
        for item in self.elements.iter() {
            new_elements.push(f(item));
        }
        ArrayQueue {
            elements: Rc::new(new_elements),
        }
    }
}

impl<A: Clone> Queue<A> for ArrayQueue<A> {
    fn enqueue(self, value: A) -> Self {
        let mut new_elements = (*self.elements).clone();
        new_elements.push(value);
        ArrayQueue {
            elements: Rc::new(new_elements),
        }
    }

    fn dequeue(self) -> Result<(A, Self), QueueError>
    where
        Self: Sized,
    {
        if self.elements.is_empty() {
            return Err(QueueError::EmptyQueueError);
        }

        let value = self.elements[0].clone();
        let mut new_elements = (*self.elements).clone();
        new_elements.remove(0);

        Ok((
            value,
            ArrayQueue {
                elements: Rc::new(new_elements),
            },
        ))
    }

    fn peek(&self) -> Result<A, QueueError>
    where
        A: Clone,
    {
        if self.elements.is_empty() {
            return Err(QueueError::EmptyQueueError);
        }

        self.elements
            .get(0)
            .map(|v| v.clone())
            .ok_or(QueueError::EmptyQueueError)
    }

    fn size(&self) -> usize {
        self.elements.len()
    }

    fn is_empty(&self) -> bool {
        rust_fp_categories::Empty::is_empty(self)
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let elements: Vec<A> = iter.into_iter().collect();
        ArrayQueue {
            elements: Rc::new(elements),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Queue;

    #[test]
    fn test_empty_queue() {
        let queue: ArrayQueue<i32> = ArrayQueue::empty();
        assert!(rust_fp_categories::Empty::is_empty(&queue));
        assert_eq!(queue.size(), 0);
        assert!(queue.peek().is_err());
    }

    #[test]
    fn test_enqueue_dequeue() {
        let queue = ArrayQueue::empty();
        let queue = queue.enqueue(1).enqueue(2).enqueue(3);

        assert_eq!(queue.size(), 3);
        assert!(!rust_fp_categories::Empty::is_empty(&queue));

        let (value, queue) = queue.dequeue().unwrap();
        assert_eq!(value, 1);
        assert_eq!(queue.size(), 2);

        let (value, queue) = queue.dequeue().unwrap();
        assert_eq!(value, 2);
        assert_eq!(queue.size(), 1);

        let (value, queue) = queue.dequeue().unwrap();
        assert_eq!(value, 3);
        assert_eq!(queue.size(), 0);
        assert!(rust_fp_categories::Empty::is_empty(&queue));

        assert!(queue.dequeue().is_err());
    }

    #[test]
    fn test_peek() {
        let queue = ArrayQueue::empty().enqueue(1).enqueue(2);

        assert_eq!(queue.peek().unwrap(), 1);

        let (_, queue) = queue.dequeue().unwrap();
        assert_eq!(queue.peek().unwrap(), 2);
    }

    #[test]
    fn test_from_iter() {
        let queue = ArrayQueue::from_iter(vec![1, 2, 3]);

        assert_eq!(queue.size(), 3);

        let (value, queue) = queue.dequeue().unwrap();
        assert_eq!(value, 1);

        let (value, queue) = queue.dequeue().unwrap();
        assert_eq!(value, 2);

        let (value, _) = queue.dequeue().unwrap();
        assert_eq!(value, 3);
    }

    #[test]
    fn test_large_queue() {
        let mut queue = ArrayQueue::empty();
        for i in 0..100 {
            queue = queue.enqueue(i);
        }

        assert_eq!(queue.size(), 100);

        for i in 0..100 {
            let (value, new_queue) = queue.dequeue().unwrap();
            assert_eq!(value, i);
            queue = new_queue;
        }

        assert!(rust_fp_categories::Empty::is_empty(&queue));
    }

    #[test]
    fn test_clone() {
        let queue1 = ArrayQueue::from_iter(vec![1, 2, 3]);
        let queue2 = queue1.clone();

        // Both queues should be equal
        assert_eq!(queue1, queue2);

        // Modifying one queue should not affect the other
        let (_, queue1_new) = queue1.dequeue().unwrap();
        assert_eq!(queue1_new.size(), 2);
        assert_eq!(queue2.size(), 3);
    }
}
