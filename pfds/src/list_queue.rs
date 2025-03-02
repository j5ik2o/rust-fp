use std::rc::Rc;

use crate::{List, Queue, QueueError, Stack, StackError};
use rust_fp_categories::{Applicative, Apply, Bind, Empty, Foldable, Functor, Monad, Pure};

/// A queue implementation using two lists.
///
/// This implementation uses two lists to achieve amortized O(1) operations.
/// The front list contains elements that can be dequeued, in reverse order.
/// The rear list contains elements that have been enqueued, in order.
/// When the front list is empty and an element needs to be dequeued,
/// the rear list is reversed and becomes the new front list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListQueue<A: Clone> {
    front: Rc<List<A>>,
    rear: Rc<List<A>>,
}

impl<A: Clone> ListQueue<A> {
    /// Creates a new empty queue.
    pub fn new() -> Self {
        ListQueue {
            front: Rc::new(List::empty()),
            rear: Rc::new(List::empty()),
        }
    }

    /// Helper method to check if the queue is valid.
    /// A queue is valid if it's either empty or has elements in the front list.
    fn check_queue(front: Rc<List<A>>, rear: Rc<List<A>>) -> (Rc<List<A>>, Rc<List<A>>)
    where
        A: Clone,
    {
        if rust_fp_categories::Empty::is_empty(&*front)
            && !rust_fp_categories::Empty::is_empty(&*rear)
        {
            let reversed_rear = Rc::new(rear.reverse());
            (reversed_rear, Rc::new(List::empty()))
        } else {
            (front, rear)
        }
    }
}

impl<A: Clone> Empty for ListQueue<A> {
    fn empty() -> Self {
        ListQueue::new()
    }

    fn is_empty(&self) -> bool {
        rust_fp_categories::Empty::is_empty(&*self.front)
            && rust_fp_categories::Empty::is_empty(&*self.rear)
    }
}

impl<A: Clone> Functor for ListQueue<A> {
    type Elm = A;
    type M<U: Clone> = ListQueue<U>;

    fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            ListQueue::empty()
        } else {
            let mut result = ListQueue::empty();
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

impl<A: Clone> Pure for ListQueue<A> {
    type Elm = A;
    type M<U: Clone> = ListQueue<U>;

    fn pure(value: A) -> ListQueue<A> {
        ListQueue::empty().enqueue(value)
    }

    fn unit() -> Self::M<()> {
        ListQueue::empty().enqueue(())
    }
}

impl<A: Clone> Apply for ListQueue<A> {
    type Elm = A;
    type M<U: Clone> = ListQueue<U>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&A) -> B,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            ListQueue::empty()
        } else {
            let mut result = ListQueue::empty();
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

impl<A: Clone> Applicative for ListQueue<A> {}

impl<A: Clone> Bind for ListQueue<A> {
    type Elm = A;
    type M<U: Clone> = ListQueue<U>;

    fn bind<B: Clone, F>(self, f: F) -> ListQueue<B>
    where
        F: Fn(&A) -> ListQueue<B>,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            ListQueue::empty()
        } else {
            let mut result = ListQueue::empty();
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

impl<A: Clone> Monad for ListQueue<A> {}

impl<A: Clone> Foldable for ListQueue<A> {
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

impl<A: Clone> Queue<A> for ListQueue<A> {
    fn enqueue(self, value: A) -> Self {
        let new_rear = Rc::new(List::cons((*self.rear).clone(), value));
        ListQueue {
            front: self.front,
            rear: new_rear,
        }
    }

    fn dequeue(self) -> Result<(A, Self), QueueError> {
        if rust_fp_categories::Empty::is_empty(&*self.front)
            && rust_fp_categories::Empty::is_empty(&*self.rear)
        {
            return Err(QueueError::EmptyQueueError);
        }

        let (front, rear) = ListQueue::check_queue(self.front.clone(), self.rear.clone());

        match (*front).head() {
            Ok(value) => {
                let new_front = (*front).tail();
                let (checked_front, checked_rear) = ListQueue::check_queue(new_front, rear);
                Ok((
                    value.clone(),
                    ListQueue {
                        front: checked_front,
                        rear: checked_rear,
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
        if rust_fp_categories::Empty::is_empty(&*self.front)
            && rust_fp_categories::Empty::is_empty(&*self.rear)
        {
            return Err(QueueError::EmptyQueueError);
        }

        // frontが空でない場合は、frontの先頭要素を返す
        if !rust_fp_categories::Empty::is_empty(&*self.front) {
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
        (*self.front).size() + (*self.rear).size()
    }

    fn is_empty(&self) -> bool {
        rust_fp_categories::Empty::is_empty(self)
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut queue = ListQueue::empty();
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
        let queue: ListQueue<i32> = ListQueue::empty();
        assert!(rust_fp_categories::Empty::is_empty(&queue));
        assert_eq!(queue.size(), 0);
        assert!(queue.peek().is_err());
    }

    #[test]
    fn test_enqueue_dequeue() {
        let queue = ListQueue::empty();
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
        let queue = ListQueue::empty().enqueue(1).enqueue(2);

        assert_eq!(queue.peek().unwrap(), 1);

        let (_, queue) = queue.dequeue().unwrap();
        assert_eq!(queue.peek().unwrap(), 2);
    }

    #[test]
    fn test_from_iter() {
        let queue = ListQueue::from_iter(vec![1, 2, 3]);

        assert_eq!(queue.size(), 3);

        let (value, queue) = queue.dequeue().unwrap();
        assert_eq!(value, 1);

        let (value, queue) = queue.dequeue().unwrap();
        assert_eq!(value, 2);

        let (value, _) = queue.dequeue().unwrap();
        assert_eq!(value, 3);
    }
}
