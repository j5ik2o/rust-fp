use std::rc::Rc;

use crate::{List, Queue, QueueError, Stack};
use rust_fp_categories::{Applicative, Apply, Bind, Empty, Foldable, Functor, Monad, Pure};

/// An optimized queue implementation using two lists.
///
/// This implementation uses two lists to achieve amortized O(1) operations.
/// The front list contains elements that can be dequeued, in reverse order.
/// The rear list contains elements that have been enqueued, in order.
/// When the front list is empty and an element needs to be dequeued,
/// the rear list is reversed and becomes the new front list.
///
/// This implementation includes several optimizations:
/// 1. Lazy evaluation of the check_queue operation
/// 2. Caching of size to avoid recalculation
/// 3. More efficient handling of empty queues
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizedQueue<A: Clone> {
    front: Rc<List<A>>,
    rear: Rc<List<A>>,
    front_size: usize,
    rear_size: usize,
}

impl<A: Clone> OptimizedQueue<A> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        OptimizedQueue {
            front: Rc::new(List::empty()),
            rear: Rc::new(List::empty()),
            front_size: 0,
            rear_size: 0,
        }
    }

    /// Helper method to check if the queue is valid.
    /// A queue is valid if it's either empty or has elements in the front list.
    /// This operation is performed lazily, only when needed.
    fn check_queue(
        front: Rc<List<A>>,
        rear: Rc<List<A>>,
        front_size: usize,
        rear_size: usize,
    ) -> (Rc<List<A>>, Rc<List<A>>, usize, usize)
    where
        A: Clone,
    {
        if front_size == 0 && rear_size > 0 {
            let reversed_rear = Rc::new((*rear).reverse());
            (reversed_rear, Rc::new(List::empty()), rear_size, 0)
        } else {
            (front, rear, front_size, rear_size)
        }
    }
}

impl<A: Clone> Empty for OptimizedQueue<A> {
    fn empty() -> Self {
        OptimizedQueue::new()
    }

    fn is_empty(&self) -> bool {
        self.front_size == 0 && self.rear_size == 0
    }
}

impl<A: Clone> Functor for OptimizedQueue<A> {
    type Elm = A;
    type M<U: Clone> = OptimizedQueue<U>;

    fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            OptimizedQueue::empty()
        } else {
            let mut result = OptimizedQueue::empty();
            let mut current_queue = self;

            while !rust_fp_categories::Empty::is_empty(&current_queue) {
                match current_queue.dequeue() {
                    Ok((value, new_queue)) => {
                        result = result.enqueue(f(&value));
                        current_queue = new_queue;
                    }
                    Err(_) => break,
                }
            }

            result
        }
    }
}

impl<A: Clone> Pure for OptimizedQueue<A> {
    type Elm = A;
    type M<U: Clone> = OptimizedQueue<U>;

    fn pure(value: A) -> OptimizedQueue<A> {
        OptimizedQueue::empty().enqueue(value)
    }

    fn unit() -> Self::M<()> {
        OptimizedQueue::empty().enqueue(())
    }
}

impl<A: Clone> Apply for OptimizedQueue<A> {
    type Elm = A;
    type M<U: Clone> = OptimizedQueue<U>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&A) -> B,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            OptimizedQueue::empty()
        } else {
            let mut result = OptimizedQueue::empty();
            let mut fs_clone = fs;

            while let Ok((f, new_fs)) = fs_clone.dequeue() {
                let mut self_clone = self.clone();
                while let Ok((a, new_self)) = self_clone.dequeue() {
                    result = result.enqueue(f(&a));
                    self_clone = new_self;
                }
                fs_clone = new_fs;
            }

            result
        }
    }
}

impl<A: Clone> Applicative for OptimizedQueue<A> {}

impl<A: Clone> Bind for OptimizedQueue<A> {
    type Elm = A;
    type M<U: Clone> = OptimizedQueue<U>;

    fn bind<B: Clone, F>(self, f: F) -> OptimizedQueue<B>
    where
        F: Fn(&A) -> OptimizedQueue<B>,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            OptimizedQueue::empty()
        } else {
            let mut result = OptimizedQueue::empty();
            let mut current_queue = self;

            while !rust_fp_categories::Empty::is_empty(&current_queue) {
                match current_queue.dequeue() {
                    Ok((value, new_queue)) => {
                        let mut inner_queue = f(&value);
                        while let Ok((inner_value, new_inner_queue)) = inner_queue.dequeue() {
                            result = result.enqueue(inner_value);
                            inner_queue = new_inner_queue;
                        }
                        current_queue = new_queue;
                    }
                    Err(_) => break,
                }
            }

            result
        }
    }
}

impl<A: Clone> Monad for OptimizedQueue<A> {}

impl<A: Clone> Foldable for OptimizedQueue<A> {
    type Elm = A;

    fn fold_left<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(B, &Self::Elm) -> B,
    {
        let mut result = b;
        let mut current_queue = self.clone();

        while !rust_fp_categories::Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    result = f(result, &value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        result
    }

    fn fold_right<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(&Self::Elm, B) -> B,
    {
        // 右畳み込みは左畳み込みを使って実装
        // 要素を逆順にして左畳み込みを適用
        let mut elements = Vec::new();
        let mut current_queue = self.clone();

        while !rust_fp_categories::Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    elements.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        elements.iter().rev().fold(b, |acc, elem| f(elem, acc))
    }
}

impl<A: Clone> Queue<A> for OptimizedQueue<A> {
    fn enqueue(self, value: A) -> Self {
        let new_rear = Rc::new(List::cons((*self.rear).clone(), value));
        let new_rear_size = self.rear_size + 1;

        OptimizedQueue {
            front: self.front,
            rear: new_rear,
            front_size: self.front_size,
            rear_size: new_rear_size,
        }
    }

    fn dequeue(self) -> Result<(A, Self), QueueError>
    where
        Self: Sized,
    {
        if self.front_size == 0 && self.rear_size == 0 {
            return Err(QueueError::EmptyQueueError);
        }

        // Only perform the check_queue operation if we need to dequeue
        let (front, rear, front_size, rear_size) =
            OptimizedQueue::check_queue(self.front, self.rear, self.front_size, self.rear_size);

        match (*front).head() {
            Ok(value) => {
                let new_front = (*front).tail();
                let new_front_size = front_size - 1;

                Ok((
                    value.clone(),
                    OptimizedQueue {
                        front: new_front,
                        rear,
                        front_size: new_front_size,
                        rear_size,
                    },
                ))
            }
            Err(_) => Err(QueueError::EmptyQueueError),
        }
    }

    fn peek(&self) -> Result<A, QueueError>
    where
        A: Clone,
    {
        if self.front_size == 0 && self.rear_size == 0 {
            return Err(QueueError::EmptyQueueError);
        }

        // frontが空でない場合は、frontの先頭要素を返す
        if self.front_size > 0 {
            (*self.front)
                .head()
                .map(|v| v.clone())
                .map_err(|_| QueueError::EmptyQueueError)
        } else {
            // frontが空でrearが空でない場合は、
            // 一時的にrearを反転させたリストを作成し、その先頭要素を返す
            let reversed_rear = (*self.rear).clone().reverse();
            match reversed_rear.head() {
                Ok(value) => Ok(value.clone()),
                Err(_) => Err(QueueError::EmptyQueueError),
            }
        }
    }

    fn size(&self) -> usize {
        // O(1) operation due to cached sizes
        self.front_size + self.rear_size
    }

    fn is_empty(&self) -> bool {
        rust_fp_categories::Empty::is_empty(self)
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut queue = OptimizedQueue::empty();
        for item in iter {
            queue = queue.enqueue(item);
        }
        queue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Queue;

    #[test]
    fn test_empty_queue() {
        let queue: OptimizedQueue<i32> = OptimizedQueue::empty();
        assert!(rust_fp_categories::Empty::is_empty(&queue));
        assert_eq!(queue.size(), 0);
        assert!(queue.peek().is_err());
    }

    #[test]
    fn test_enqueue_dequeue() {
        let queue = OptimizedQueue::empty();
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
        let queue = OptimizedQueue::empty().enqueue(1).enqueue(2);

        assert_eq!(queue.peek().unwrap(), 1);

        let (_, queue) = queue.dequeue().unwrap();
        assert_eq!(queue.peek().unwrap(), 2);
    }

    #[test]
    fn test_from_iter() {
        let queue = OptimizedQueue::from_iter(vec![1, 2, 3]);

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
        let mut queue = OptimizedQueue::empty();
        for i in 0..1000 {
            queue = queue.enqueue(i);
        }

        assert_eq!(queue.size(), 1000);

        for i in 0..1000 {
            let (value, new_queue) = queue.dequeue().unwrap();
            assert_eq!(value, i);
            queue = new_queue;
        }

        assert!(rust_fp_categories::Empty::is_empty(&queue));
    }

    #[test]
    fn test_alternating_operations() {
        let mut queue = OptimizedQueue::empty();

        // Enqueue and dequeue alternately
        for i in 0..100 {
            queue = queue.enqueue(i);
            let (value, new_queue) = queue.dequeue().unwrap();
            assert_eq!(value, i);
            queue = new_queue;
        }

        assert!(rust_fp_categories::Empty::is_empty(&queue));

        // Enqueue several items, then dequeue several items
        for i in 0..50 {
            queue = queue.enqueue(i);
        }

        assert_eq!(queue.size(), 50);

        for i in 0..25 {
            let (value, new_queue) = queue.dequeue().unwrap();
            assert_eq!(value, i);
            queue = new_queue;
        }

        assert_eq!(queue.size(), 25);

        for i in 50..75 {
            queue = queue.enqueue(i);
        }

        assert_eq!(queue.size(), 50);

        for i in 25..75 {
            let (value, new_queue) = queue.dequeue().unwrap();
            assert_eq!(value, i);
            queue = new_queue;
        }

        assert!(rust_fp_categories::Empty::is_empty(&queue));
    }
}
