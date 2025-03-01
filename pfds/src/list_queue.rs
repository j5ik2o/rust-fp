use std::rc::Rc;

use crate::{List, Queue, QueueError, Stack, StackError};
use rust_fp_categories::Empty;

/// A queue implementation using two lists.
/// 
/// This implementation uses two lists to achieve amortized O(1) operations.
/// The front list contains elements that can be dequeued, in reverse order.
/// The rear list contains elements that have been enqueued, in order.
/// When the front list is empty and an element needs to be dequeued,
/// the rear list is reversed and becomes the new front list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListQueue<A> {
    front: Rc<List<A>>,
    rear: Rc<List<A>>,
}

impl<A> ListQueue<A> {
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
        if rust_fp_categories::Empty::is_empty(&*front) && !rust_fp_categories::Empty::is_empty(&*rear) {
            let reversed_rear = Rc::new(rear.reverse());
            (reversed_rear, Rc::new(List::empty()))
        } else {
            (front, rear)
        }
    }
}

impl<A> Empty for ListQueue<A> {
    fn empty() -> Self {
        ListQueue::new()
    }

    fn is_empty(&self) -> bool {
        rust_fp_categories::Empty::is_empty(&*self.front) && rust_fp_categories::Empty::is_empty(&*self.rear)
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
        if rust_fp_categories::Empty::is_empty(&*self.front) && rust_fp_categories::Empty::is_empty(&*self.rear) {
            return Err(QueueError::EmptyQueueError);
        }
        
        let (front, rear) = ListQueue::check_queue(self.front.clone(), self.rear.clone());
        
        match (*front).head() {
            Ok(value) => {
                let new_front = (*front).tail();
                let (checked_front, checked_rear) = ListQueue::check_queue(new_front, rear);
                Ok((value.clone(), ListQueue {
                    front: checked_front,
                    rear: checked_rear,
                }))
            }
            Err(_) => Err(QueueError::EmptyQueueError),
        }
    }

    fn peek(&self) -> Result<A, QueueError> where A: Clone {
        if rust_fp_categories::Empty::is_empty(&*self.front) && rust_fp_categories::Empty::is_empty(&*self.rear) {
            return Err(QueueError::EmptyQueueError);
        }
        
        // frontが空でない場合は、frontの先頭要素を返す
        if !rust_fp_categories::Empty::is_empty(&*self.front) {
            (*self.front).head().map(|v| v.clone()).map_err(|_| QueueError::EmptyQueueError)
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
