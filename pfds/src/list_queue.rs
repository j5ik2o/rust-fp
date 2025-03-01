use std::rc::Rc;

use crate::{List, Queue, QueueError};
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
        if front.is_empty() && !rear.is_empty() {
            let reversed_rear = rear.reverse();
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
        self.front.is_empty() && self.rear.is_empty()
    }
}

impl<A: Clone> Queue<A> for ListQueue<A> {
    fn enqueue(self, value: A) -> Self {
        let new_rear = self.rear.cons(value);
        let (front, _) = ListQueue::check_queue(self.front, new_rear);
        ListQueue {
            front,
            rear: new_rear,
        }
    }

    fn dequeue(self) -> Result<(A, Self), QueueError> {
        if self.is_empty() {
            return Err(QueueError::EmptyQueueError);
        }
        
        let (front, rear) = ListQueue::check_queue(self.front, self.rear);
        
        match front.head() {
            Ok(value) => {
                let new_front = front.tail();
                let (checked_front, checked_rear) = ListQueue::check_queue(new_front, rear);
                Ok((value.clone(), ListQueue {
                    front: checked_front,
                    rear: checked_rear,
                }))
            }
            Err(_) => Err(QueueError::EmptyQueueError),
        }
    }

    fn peek(&self) -> Result<&A, QueueError> {
        if self.is_empty() {
            return Err(QueueError::EmptyQueueError);
        }
        
        self.front.head()
    }

    fn size(&self) -> usize {
        self.front.size() + self.rear.size()
    }

    fn is_empty(&self) -> bool {
        Empty::is_empty(self)
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
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
        assert!(queue.peek().is_err());
    }

    #[test]
    fn test_enqueue_dequeue() {
        let queue = ListQueue::empty();
        let queue = queue.enqueue(1).enqueue(2).enqueue(3);
        
        assert_eq!(queue.size(), 3);
        assert!(!queue.is_empty());
        
        let (value, queue) = queue.dequeue().unwrap();
        assert_eq!(value, 1);
        assert_eq!(queue.size(), 2);
        
        let (value, queue) = queue.dequeue().unwrap();
        assert_eq!(value, 2);
        assert_eq!(queue.size(), 1);
        
        let (value, queue) = queue.dequeue().unwrap();
        assert_eq!(value, 3);
        assert_eq!(queue.size(), 0);
        assert!(queue.is_empty());
        
        assert!(queue.dequeue().is_err());
    }

    #[test]
    fn test_peek() {
        let queue = ListQueue::empty().enqueue(1).enqueue(2);
        
        assert_eq!(*queue.peek().unwrap(), 1);
        
        let (_, queue) = queue.dequeue().unwrap();
        assert_eq!(*queue.peek().unwrap(), 2);
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
