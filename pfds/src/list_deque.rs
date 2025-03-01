use std::rc::Rc;

use crate::{Deque, DequeError};
use rust_fp_categories::Empty;

/// A list-based implementation of a double-ended queue (deque).
///
/// This implementation uses two lists to represent the front and back of the deque.
/// Elements are stored in reverse order in the front list and in normal order in the back list.
/// This allows for efficient operations on both ends of the deque.
///
/// Time complexity:
/// - push_front: O(1)
/// - push_back: O(1)
/// - pop_front: O(1) amortized
/// - pop_back: O(1) amortized
/// - peek_front: O(1)
/// - peek_back: O(1)
/// - size: O(1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListDeque<A> {
    front: Rc<Vec<A>>,
    back: Rc<Vec<A>>,
    size: usize,
}

impl<A: Clone> ListDeque<A> {
    /// Creates a new empty deque.
    pub fn new() -> Self {
        ListDeque {
            front: Rc::new(Vec::new()),
            back: Rc::new(Vec::new()),
            size: 0,
        }
    }

    /// Balances the deque by redistributing elements between the front and back lists.
    ///
    /// This is called when one of the lists becomes empty, to ensure efficient operations.
    /// The front list stores elements in reverse order, and the back list stores elements in normal order.
    fn balance(self) -> Self {
        // If both lists have elements or both are empty, no balancing needed
        if (!self.front.is_empty() && !self.back.is_empty())
            || (self.front.is_empty() && self.back.is_empty())
        {
            return self;
        }

        let total_len = self.front.len() + self.back.len();

        // If there are no elements, return an empty deque
        if total_len == 0 {
            return ListDeque::empty();
        }

        // Create a single vector with all elements in the correct order
        let mut all_elements = Vec::with_capacity(total_len);

        // Add elements from front (in reverse order, so we need to reverse them)
        for i in (0..self.front.len()).rev() {
            all_elements.push(self.front[i].clone());
        }

        // Add elements from back (already in correct order)
        for i in 0..self.back.len() {
            all_elements.push(self.back[i].clone());
        }

        // Split the elements evenly between front and back
        let mid = total_len / 2;

        let mut new_front = Vec::with_capacity(mid);
        let mut new_back = Vec::with_capacity(total_len - mid);

        // Front elements are stored in reverse order
        for i in (0..mid).rev() {
            new_front.push(all_elements[i].clone());
        }

        // Back elements are stored in normal order
        for i in mid..total_len {
            new_back.push(all_elements[i].clone());
        }

        ListDeque {
            front: Rc::new(new_front),
            back: Rc::new(new_back),
            size: self.size,
        }
    }
}

impl<A: Clone> Empty for ListDeque<A> {
    fn empty() -> Self {
        ListDeque::new()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<A: Clone> Deque<A> for ListDeque<A> {
    fn push_front(self, value: A) -> Self {
        // Push to front list - elements are stored in reverse order
        // so the most recently added element is at the end of the front list
        let mut new_front = (*self.front).clone();
        new_front.push(value);

        ListDeque {
            front: Rc::new(new_front),
            back: self.back,
            size: self.size + 1,
        }
    }

    fn push_back(self, value: A) -> Self {
        // Push to back list - elements are stored in normal order
        // so the most recently added element is at the end of the back list
        let mut new_back = (*self.back).clone();
        new_back.push(value);

        ListDeque {
            front: self.front,
            back: Rc::new(new_back),
            size: self.size + 1,
        }
    }

    fn pop_front(self) -> Result<(A, Self), DequeError> {
        if rust_fp_categories::Empty::is_empty(&self) {
            return Err(DequeError::EmptyDequeError);
        }

        if !self.front.is_empty() {
            // If front is not empty, pop from front
            // In the front list, elements are stored in reverse order
            // so the last element of the front list is the first element of the deque
            let front_len = self.front.len();
            let value = self.front[front_len - 1].clone();

            let mut new_front = (*self.front).clone();
            new_front.pop();

            // Check if we need to balance
            let need_balance = new_front.is_empty() && !self.back.is_empty();

            let new_deque = ListDeque {
                front: Rc::new(new_front),
                back: self.back,
                size: self.size - 1,
            };

            // Balance the deque if front becomes empty
            if need_balance {
                Ok((value, new_deque.balance()))
            } else {
                Ok((value, new_deque))
            }
        } else {
            // If front is empty, pop from back (which means taking the first element)
            // In the back list, elements are stored in normal order
            // so the first element of the back list is the first element of the deque
            let value = self.back[0].clone();

            let mut new_back = (*self.back).clone();
            new_back.remove(0);

            // Check if we need to balance
            let need_balance = new_back.is_empty() && !self.front.is_empty();

            let new_deque = ListDeque {
                front: self.front,
                back: Rc::new(new_back),
                size: self.size - 1,
            };

            // Balance the deque if back becomes empty
            if need_balance {
                Ok((value, new_deque.balance()))
            } else {
                Ok((value, new_deque))
            }
        }
    }

    fn pop_back(self) -> Result<(A, Self), DequeError> {
        if rust_fp_categories::Empty::is_empty(&self) {
            return Err(DequeError::EmptyDequeError);
        }

        if !self.back.is_empty() {
            // If back is not empty, pop from back
            // In the back list, elements are stored in normal order
            // so the last element of the back list is the last element of the deque
            let back_len = self.back.len();
            let value = self.back[back_len - 1].clone();

            let mut new_back = (*self.back).clone();
            new_back.pop();

            // Check if we need to balance
            let need_balance = new_back.is_empty() && !self.front.is_empty();

            let new_deque = ListDeque {
                front: self.front,
                back: Rc::new(new_back),
                size: self.size - 1,
            };

            // Balance the deque if back becomes empty
            if need_balance {
                Ok((value, new_deque.balance()))
            } else {
                Ok((value, new_deque))
            }
        } else {
            // If back is empty, pop from front (which means taking the first element)
            // In the front list, elements are stored in reverse order
            // so the first element of the front list is the last element of the deque
            let value = self.front[0].clone();

            let mut new_front = (*self.front).clone();
            new_front.remove(0);

            // Check if we need to balance
            let need_balance = new_front.is_empty() && !self.back.is_empty();

            let new_deque = ListDeque {
                front: Rc::new(new_front),
                back: self.back,
                size: self.size - 1,
            };

            // Balance the deque if front becomes empty
            if need_balance {
                Ok((value, new_deque.balance()))
            } else {
                Ok((value, new_deque))
            }
        }
    }

    fn peek_front(&self) -> Result<A, DequeError> {
        if rust_fp_categories::Empty::is_empty(self) {
            return Err(DequeError::EmptyDequeError);
        }

        if !self.front.is_empty() {
            // If front is not empty, peek at the last element of front
            let front_len = self.front.len();
            Ok(self.front[front_len - 1].clone())
        } else {
            // If front is empty, peek at the first element of back
            Ok(self.back[0].clone())
        }
    }

    fn peek_back(&self) -> Result<A, DequeError> {
        if rust_fp_categories::Empty::is_empty(self) {
            return Err(DequeError::EmptyDequeError);
        }

        if !self.back.is_empty() {
            // If back is not empty, peek at the last element of back
            let back_len = self.back.len();
            Ok(self.back[back_len - 1].clone())
        } else {
            // If back is empty, peek at the first element of front
            Ok(self.front[0].clone())
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        // Create a deque from an iterator by pushing each item to the back
        let mut deque = ListDeque::empty();
        for item in iter {
            deque = deque.push_back(item);
        }
        deque
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Deque;

    #[test]
    fn test_empty_deque() {
        let deque: ListDeque<i32> = ListDeque::empty();
        assert!(deque.is_empty());
        assert_eq!(deque.size(), 0);
        assert!(deque.peek_front().is_err());
        assert!(deque.peek_back().is_err());
    }

    #[test]
    fn test_push_front_pop_front() {
        let mut deque = ListDeque::empty();
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
    fn test_push_back_pop_back() {
        let mut deque = ListDeque::empty();
        deque = deque.push_back(1);
        deque = deque.push_back(2);
        deque = deque.push_back(3);

        assert_eq!(deque.size(), 3);
        assert!(!deque.is_empty());

        let (value, new_deque) = deque.pop_back().unwrap();
        assert_eq!(value, 3);
        assert_eq!(new_deque.size(), 2);
        deque = new_deque;

        let (value, new_deque) = deque.pop_back().unwrap();
        assert_eq!(value, 2);
        assert_eq!(new_deque.size(), 1);
        deque = new_deque;

        let (value, new_deque) = deque.pop_back().unwrap();
        assert_eq!(value, 1);
        assert_eq!(new_deque.size(), 0);
        assert!(new_deque.is_empty());
        deque = new_deque;

        assert!(deque.pop_back().is_err());
    }

    #[test]
    fn test_push_front_pop_back() {
        let mut deque = ListDeque::empty();
        deque = deque.push_front(1);
        deque = deque.push_front(2);
        deque = deque.push_front(3);

        let (value, new_deque) = deque.pop_back().unwrap();
        assert_eq!(value, 1);
        deque = new_deque;

        let (value, new_deque) = deque.pop_back().unwrap();
        assert_eq!(value, 2);
        deque = new_deque;

        let (value, new_deque) = deque.pop_back().unwrap();
        assert_eq!(value, 3);
        deque = new_deque;

        assert!(deque.is_empty());
    }

    #[test]
    fn test_push_back_pop_front() {
        let mut deque = ListDeque::empty();
        deque = deque.push_back(1);
        deque = deque.push_back(2);
        deque = deque.push_back(3);

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

    #[test]
    fn test_peek() {
        let deque = ListDeque::empty().push_front(1).push_back(2);

        assert_eq!(deque.peek_front().unwrap(), 1);
        assert_eq!(deque.peek_back().unwrap(), 2);

        let (_, deque) = deque.pop_front().unwrap();
        assert_eq!(deque.peek_front().unwrap(), 2);
        assert_eq!(deque.peek_back().unwrap(), 2);
    }

    #[test]
    fn test_from_iter() {
        let mut deque = ListDeque::from_iter(vec![1, 2, 3]);

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

    #[test]
    fn test_mixed_operations() {
        let mut deque = ListDeque::empty();

        // Push elements from both ends
        deque = deque.push_front(1);
        deque = deque.push_back(2);
        deque = deque.push_front(3);
        deque = deque.push_back(4);

        // Expected order: [3, 1, 2, 4]
        assert_eq!(deque.size(), 4);

        // Check peek operations
        assert_eq!(deque.peek_front().unwrap(), 3);
        assert_eq!(deque.peek_back().unwrap(), 4);

        // Pop from front
        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 3);
        deque = new_deque;

        // Pop from back
        let (value, new_deque) = deque.pop_back().unwrap();
        assert_eq!(value, 4);
        deque = new_deque;

        // Expected order: [1, 2]
        assert_eq!(deque.size(), 2);
        assert_eq!(deque.peek_front().unwrap(), 1);
        assert_eq!(deque.peek_back().unwrap(), 2);
    }

    #[test]
    fn test_balance() {
        // Create a deque with elements only in the front
        let mut deque = ListDeque::empty();
        for i in 1..=10 {
            deque = deque.push_front(i);
        }

        // Pop all elements from the front
        for _ in 0..4 {
            let (_, new_deque) = deque.pop_front().unwrap();
            deque = new_deque;
        }

        // The deque should have 6 elements left
        assert_eq!(deque.size(), 6);

        // Pop the remaining elements and check their order
        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 6);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 5);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 4);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 3);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 2);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 1);
        deque = new_deque;

        assert!(deque.is_empty());
    }
}
