use std::rc::Rc;

use crate::{Deque, DequeError};
use rust_fp_categories::Empty;

/// An optimized list-based implementation of a double-ended queue (deque).
///
/// This implementation uses two vectors to represent the front and back of the deque,
/// with optimizations for better performance compared to the basic ListDeque.
/// Elements are stored in reverse order in the front list and in normal order in the back list.
/// This allows for efficient operations on both ends of the deque.
///
/// 最適化されたリストベースの両端キュー（deque）の実装。
///
/// この実装では、dequeの前部と後部を表現するために2つのベクターを使用し、
/// 基本的なListDequeと比較してパフォーマンスを向上させるための最適化が施されています。
/// 要素は前部リストでは逆順に、後部リストでは通常の順序で格納されます。
/// これにより、dequeの両端での効率的な操作が可能になります。
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
pub struct OptimizedDeque<A> {
    front: Rc<Vec<A>>,
    back: Rc<Vec<A>>,
    size: usize,
}

impl<A: Clone> OptimizedDeque<A> {
    /// Creates a new empty deque.
    pub fn new() -> Self {
        OptimizedDeque {
            front: Rc::new(Vec::new()),
            back: Rc::new(Vec::new()),
            size: 0,
        }
    }

    /// Balances the deque by redistributing elements between the front and back lists.
    ///
    /// This is called when one of the lists becomes empty, to ensure efficient operations.
    /// The front list stores elements in reverse order, and the back list stores elements in normal order.
    /// This implementation is optimized to minimize cloning and memory allocations.
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
            return OptimizedDeque::empty();
        }

        // Optimization: Pre-allocate capacity for the new vectors
        let mid = total_len / 2;
        let mut new_front = Vec::with_capacity(mid);
        let mut new_back = Vec::with_capacity(total_len - mid);

        // If only front has elements
        if !self.front.is_empty() {
            // Split front elements between new front and new back
            // Front elements are already in reverse order
            for i in 0..mid {
                if i < self.front.len() {
                    new_front.push(self.front[i].clone());
                }
            }
            
            // Remaining elements go to back (in correct order)
            for i in (mid..self.front.len()).rev() {
                new_back.push(self.front[i].clone());
            }
        } else {
            // Only back has elements
            // Take first half of back elements for front (in reverse order)
            for i in (0..mid).rev() {
                if i < self.back.len() {
                    new_front.push(self.back[i].clone());
                }
            }
            
            // Remaining elements stay in back
            for i in mid..self.back.len() {
                new_back.push(self.back[i].clone());
            }
        }

        OptimizedDeque {
            front: Rc::new(new_front),
            back: Rc::new(new_back),
            size: self.size,
        }
    }
}

impl<A: Clone> Empty for OptimizedDeque<A> {
    fn empty() -> Self {
        OptimizedDeque::new()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<A: Clone> Deque<A> for OptimizedDeque<A> {
    fn push_front(self, value: A) -> Self {
        // Optimization: Check if we can reuse the existing vector
        let front = match Rc::try_unwrap(self.front) {
            Ok(mut vec) => {
                vec.push(value);
                Rc::new(vec)
            }
            Err(rc) => {
                // If we can't get exclusive ownership, clone the vector
                let mut new_front = (*rc).clone();
                new_front.push(value);
                Rc::new(new_front)
            }
        };

        OptimizedDeque {
            front,
            back: self.back,
            size: self.size + 1,
        }
    }

    fn push_back(self, value: A) -> Self {
        // Optimization: Check if we can reuse the existing vector
        let back = match Rc::try_unwrap(self.back) {
            Ok(mut vec) => {
                vec.push(value);
                Rc::new(vec)
            }
            Err(rc) => {
                // If we can't get exclusive ownership, clone the vector
                let mut new_back = (*rc).clone();
                new_back.push(value);
                Rc::new(new_back)
            }
        };

        OptimizedDeque {
            front: self.front,
            back,
            size: self.size + 1,
        }
    }

    fn pop_front(self) -> Result<(A, Self), DequeError> {
        if rust_fp_categories::Empty::is_empty(&self) {
            return Err(DequeError::EmptyDequeError);
        }

        if !self.front.is_empty() {
            // If front is not empty, pop from front
            let front_len = self.front.len();
            let value = self.front[front_len - 1].clone();

            // Optimization: Try to reuse the existing vector
            let front = match Rc::try_unwrap(self.front) {
                Ok(mut vec) => {
                    vec.pop();
                    Rc::new(vec)
                }
                Err(rc) => {
                    // If we can't get exclusive ownership, clone the vector
                    let mut new_front = (*rc).clone();
                    new_front.pop();
                    Rc::new(new_front)
                }
            };

            // Check if we need to balance
            let need_balance = front.is_empty() && !self.back.is_empty();

            let new_deque = OptimizedDeque {
                front,
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
            let value = self.back[0].clone();

            // Optimization: Try to reuse the existing vector
            let back = match Rc::try_unwrap(self.back) {
                Ok(mut vec) => {
                    vec.remove(0);
                    Rc::new(vec)
                }
                Err(rc) => {
                    // If we can't get exclusive ownership, clone the vector
                    let mut new_back = (*rc).clone();
                    new_back.remove(0);
                    Rc::new(new_back)
                }
            };

            // Check if we need to balance
            let need_balance = back.is_empty() && !self.front.is_empty();

            let new_deque = OptimizedDeque {
                front: self.front,
                back,
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
            let back_len = self.back.len();
            let value = self.back[back_len - 1].clone();

            // Optimization: Try to reuse the existing vector
            let back = match Rc::try_unwrap(self.back) {
                Ok(mut vec) => {
                    vec.pop();
                    Rc::new(vec)
                }
                Err(rc) => {
                    // If we can't get exclusive ownership, clone the vector
                    let mut new_back = (*rc).clone();
                    new_back.pop();
                    Rc::new(new_back)
                }
            };

            // Check if we need to balance
            let need_balance = back.is_empty() && !self.front.is_empty();

            let new_deque = OptimizedDeque {
                front: self.front,
                back,
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
            let value = self.front[0].clone();

            // Optimization: Try to reuse the existing vector
            let front = match Rc::try_unwrap(self.front) {
                Ok(mut vec) => {
                    vec.remove(0);
                    Rc::new(vec)
                }
                Err(rc) => {
                    // If we can't get exclusive ownership, clone the vector
                    let mut new_front = (*rc).clone();
                    new_front.remove(0);
                    Rc::new(new_front)
                }
            };

            // Check if we need to balance
            let need_balance = front.is_empty() && !self.back.is_empty();

            let new_deque = OptimizedDeque {
                front,
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
        // Optimization: Collect items into a vector first to determine size
        let items: Vec<A> = iter.into_iter().collect();
        let size = items.len();
        
        if size == 0 {
            return OptimizedDeque::empty();
        }
        
        // Split items between front and back for better balance
        let mid = size / 2;
        let mut front = Vec::with_capacity(mid);
        let mut back = Vec::with_capacity(size - mid);
        
        // Front elements are stored in reverse order
        for i in (0..mid).rev() {
            front.push(items[i].clone());
        }
        
        // Back elements are stored in normal order
        for i in mid..size {
            back.push(items[i].clone());
        }
        
        OptimizedDeque {
            front: Rc::new(front),
            back: Rc::new(back),
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Deque;

    #[test]
    fn test_empty_deque() {
        let deque: OptimizedDeque<i32> = OptimizedDeque::empty();
        assert!(deque.is_empty());
        assert_eq!(deque.size(), 0);
        assert!(deque.peek_front().is_err());
        assert!(deque.peek_back().is_err());
    }

    #[test]
    fn test_push_front_pop_front() {
        let mut deque = OptimizedDeque::empty();
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
        let mut deque = OptimizedDeque::empty();
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
        let mut deque = OptimizedDeque::empty();
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
        let mut deque = OptimizedDeque::empty();
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
        let deque = OptimizedDeque::empty().push_front(1).push_back(2);

        assert_eq!(deque.peek_front().unwrap(), 1);
        assert_eq!(deque.peek_back().unwrap(), 2);

        let (_, deque) = deque.pop_front().unwrap();
        assert_eq!(deque.peek_front().unwrap(), 2);
        assert_eq!(deque.peek_back().unwrap(), 2);
    }

    #[test]
    fn test_from_iter() {
        let mut deque = OptimizedDeque::from_iter(vec![1, 2, 3]);

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
        let mut deque = OptimizedDeque::empty();

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
        let mut deque = OptimizedDeque::empty();
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

    #[test]
    fn test_large_deque() {
        let mut deque = OptimizedDeque::empty();

        // Push a large number of elements
        for i in 0..100 {
            if i % 2 == 0 {
                deque = deque.push_front(i);
            } else {
                deque = deque.push_back(i);
            }
        }

        assert_eq!(deque.size(), 100);

        // Pop half from front, half from back
        for _ in 0..50 {
            let (_, new_deque) = deque.pop_front().unwrap();
            deque = new_deque;
        }

        assert_eq!(deque.size(), 50);

        for _ in 0..50 {
            let (_, new_deque) = deque.pop_back().unwrap();
            deque = new_deque;
        }

        assert!(deque.is_empty());
    }
}
