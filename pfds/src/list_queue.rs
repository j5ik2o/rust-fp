use std::rc::Rc;

use crate::{List, Queue, QueueError, Stack, StackError};
use rust_fp_categories::{Applicative, Apply, Bind, Empty, Functor, Monad, Pure};

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
    type M<B: Clone> = ListQueue<B>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
        B: Clone,
    {
        // Map the function over both the front and rear lists
        let mapped_front = Rc::new((*self.front).clone().fmap(|x| f(x)));
        let mapped_rear = Rc::new((*self.rear).clone().fmap(|x| f(x)));
        
        // Create a new ListQueue with the mapped lists
        ListQueue {
            front: mapped_front,
            rear: mapped_rear,
        }
    }
}

impl<A: Clone> Pure for ListQueue<A> {
    type Elm = A;
    type M<B: Clone> = ListQueue<B>;

    fn pure(value: A) -> Self {
        // Create a new ListQueue with a single element in the front list
        ListQueue {
            front: Rc::new(List::pure(value)),
            rear: Rc::new(List::empty()),
        }
    }

    fn unit() -> Self::M<()> {
        // Create a new ListQueue with a single unit element in the front list
        ListQueue {
            front: Rc::new(List::<()>::unit()),
            rear: Rc::new(List::empty()),
        }
    }
}

impl<A: Clone> Apply for ListQueue<A> {
    type Elm = A;
    type M<B: Clone> = ListQueue<B>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&A) -> B + Clone,
        B: Clone,
    {
        // Check if the queue is empty
        if rust_fp_categories::Empty::is_empty(&self) || rust_fp_categories::Empty::is_empty(&fs) {
            return ListQueue::empty();
        }

        // Process each function and apply it to each element
        let mut result_queue = ListQueue::empty();
        
        // Dequeue all functions from fs
        let mut current_fs = fs;
        while !rust_fp_categories::Empty::is_empty(&current_fs) {
            match current_fs.dequeue() {
                Ok((f, new_fs)) => {
                    // Apply f to each element in self
                    let mut current_self = self.clone();
                    while !rust_fp_categories::Empty::is_empty(&current_self) {
                        match current_self.dequeue() {
                            Ok((a, new_self)) => {
                                result_queue = result_queue.enqueue(f(&a));
                                current_self = new_self;
                            },
                            Err(_) => break,
                        }
                    }
                    current_fs = new_fs;
                },
                Err(_) => break,
            }
        }
        
        result_queue
    }
}

impl<A: Clone> Applicative for ListQueue<A> {}

impl<A: Clone> Bind for ListQueue<A> {
    type Elm = A;
    type M<B: Clone> = ListQueue<B>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> Self::M<B>,
        B: Clone,
    {
        let mut result = ListQueue::empty();
        
        // Process each element in the queue
        let mut current_queue = self;
        while !rust_fp_categories::Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((item, new_queue)) => {
                    // Apply the function to the item
                    let new_items = f(&item);
                    
                    // Process each item in the resulting queue
                    let mut current_new_items = new_items;
                    while !rust_fp_categories::Empty::is_empty(&current_new_items) {
                        match current_new_items.dequeue() {
                            Ok((new_item, new_items_queue)) => {
                                result = result.enqueue(new_item);
                                current_new_items = new_items_queue;
                            },
                            Err(_) => break,
                        }
                    }
                    
                    current_queue = new_queue;
                },
                Err(_) => break,
            }
        }
        
        result
    }
}

impl<A: Clone> Monad for ListQueue<A> {}

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
