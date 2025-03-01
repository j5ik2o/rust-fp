use std::fmt::Debug;

use rust_fp_categories::Empty;

use crate::{Deque, DequeError, List, Stack};

/// An optimized implementation of a persistent double-ended queue (deque).
///
/// This implementation uses two lists (front and back) to represent the deque,
/// with optimized balancing to ensure efficient operations from both ends.
/// All operations create a new deque instance, preserving the original.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizedDeque<A: Clone> {
    front: List<A>,
    back: List<A>,
    size: usize,
}

impl<A: Clone> OptimizedDeque<A> {
    /// Creates a new empty OptimizedDeque.
    pub fn new() -> Self {
        Self {
            front: List::empty(),
            back: List::empty(),
            size: 0,
        }
    }

    /// Balances the deque to ensure efficient operations.
    ///
    /// This method redistributes elements between the front and back lists
    /// when one list becomes significantly larger than the other.
    /// The balancing is performed to maintain amortized O(1) operations.
    fn balance(self) -> Self {
        let front_size = Stack::size(&self.front);
        let back_size = Stack::size(&self.back);

        // Check if balancing is needed
        if front_size > 3 * back_size + 1 || back_size > 3 * front_size + 1 {
            let total_size = front_size + back_size;
            let target_size = total_size / 2;

            // Collect all elements
            let mut all_elements = Vec::with_capacity(total_size);

            // Add front elements in reverse order (to maintain correct order)
            let mut current_front = self.front;
            while !Stack::is_empty(&current_front) {
                let (value, rest) = current_front.uncons().unwrap();
                all_elements.push(value);
                current_front = rest;
            }

            // Reverse to get correct order
            all_elements.reverse();

            // Add back elements
            let mut current_back = self.back;
            while !Stack::is_empty(&current_back) {
                let (value, rest) = current_back.uncons().unwrap();
                all_elements.push(value);
                current_back = rest;
            }

            // Create new front list (first half of elements, in reverse order)
            let mut new_front = List::empty();
            for i in (0..target_size).rev() {
                new_front = new_front.cons(all_elements[i].clone());
            }

            // Create new back list (second half of elements, in normal order)
            let mut new_back = List::empty();
            for i in (target_size..total_size).rev() {
                new_back = new_back.cons(all_elements[i].clone());
            }

            Self {
                front: new_front,
                back: new_back,
                size: total_size,
            }
        } else {
            // No balancing needed
            self
        }
    }

    /// Checks if the deque needs to be balanced after an operation.
    ///
    /// This is an optimization to avoid unnecessary balancing.
    fn needs_balance(&self) -> bool {
        let front_size = Stack::size(&self.front);
        let back_size = Stack::size(&self.back);

        front_size > 3 * back_size + 1 || back_size > 3 * front_size + 1
    }
}

impl<A: Clone> Empty for OptimizedDeque<A> {
    fn empty() -> Self {
        Self::new()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<A: Clone + Debug> Deque<A> for OptimizedDeque<A> {
    fn push_front(self, value: A) -> Self {
        let new_front = Stack::cons(self.front, value);
        let new_size = self.size + 1;

        let result = Self {
            front: new_front,
            back: self.back,
            size: new_size,
        };

        // Only balance if necessary
        if result.needs_balance() {
            result.balance()
        } else {
            result
        }
    }

    fn push_back(self, value: A) -> Self {
        let new_back = Stack::cons(self.back, value);
        let new_size = self.size + 1;

        let result = Self {
            front: self.front,
            back: new_back,
            size: new_size,
        };

        // Only balance if necessary
        if result.needs_balance() {
            result.balance()
        } else {
            result
        }
    }

    fn pop_front(self) -> Result<(A, Self), DequeError> {
        if self.is_empty() {
            return Err(DequeError::EmptyDequeError);
        }

        if !Stack::is_empty(&self.front) {
            // Pop from front
            let (value, new_front) = self.front.uncons().unwrap();
            let new_size = self.size - 1;

            let result = Self {
                front: new_front,
                back: self.back,
                size: new_size,
            };

            // Only balance if necessary
            let balanced_result = if result.needs_balance() {
                result.balance()
            } else {
                result
            };

            Ok((value, balanced_result))
        } else {
            // Front is empty, pop from back and reverse
            let mut elements = Vec::new();
            let mut current_back = self.back;

            while !Stack::is_empty(&current_back) {
                let (value, rest) = current_back.uncons().unwrap();
                elements.push(value);
                current_back = rest;
            }

            // First element is the one to return
            let value = elements[0].clone();

            // Create new front list from the rest of the elements
            let mut new_front = List::empty();
            for i in (1..elements.len()).rev() {
                new_front = Stack::cons(new_front, elements[i].clone());
            }

            let result = Self {
                front: new_front,
                back: List::empty(),
                size: self.size - 1,
            };

            Ok((value, result))
        }
    }

    fn pop_back(self) -> Result<(A, Self), DequeError> {
        if self.is_empty() {
            return Err(DequeError::EmptyDequeError);
        }

        if !Stack::is_empty(&self.back) {
            // Pop from back
            let (value, new_back) = self.back.uncons().unwrap();
            let new_size = self.size - 1;

            let result = Self {
                front: self.front,
                back: new_back,
                size: new_size,
            };

            // Only balance if necessary
            let balanced_result = if result.needs_balance() {
                result.balance()
            } else {
                result
            };

            Ok((value, balanced_result))
        } else {
            // Back is empty, pop from front and reverse
            let mut elements = Vec::new();
            let mut current_front = self.front;

            while !Stack::is_empty(&current_front) {
                let (value, rest) = current_front.uncons().unwrap();
                elements.push(value);
                current_front = rest;
            }

            // First element is the one to return
            let value = elements[0].clone();

            // Create new back list from the rest of the elements
            let mut new_back = List::empty();
            for i in (1..elements.len()).rev() {
                new_back = Stack::cons(new_back, elements[i].clone());
            }

            let result = Self {
                front: List::empty(),
                back: new_back,
                size: self.size - 1,
            };

            Ok((value, result))
        }
    }

    fn peek_front(&self) -> Result<A, DequeError> {
        if self.is_empty() {
            return Err(DequeError::EmptyDequeError);
        }

        if !Stack::is_empty(&self.front) {
            // Peek from front
            Ok(Stack::head(&self.front).unwrap().clone())
        } else {
            // Front is empty, peek from back (last element)
            let mut elements = Vec::new();
            let mut current_back = self.back.clone();

            while !Stack::is_empty(&current_back) {
                let (value, rest) = current_back.uncons().unwrap();
                elements.push(value);
                current_back = rest;
            }

            Ok(elements[elements.len() - 1].clone())
        }
    }

    fn peek_back(&self) -> Result<A, DequeError> {
        if self.is_empty() {
            return Err(DequeError::EmptyDequeError);
        }

        if !Stack::is_empty(&self.back) {
            // Peek from back
            Ok(Stack::head(&self.back).unwrap().clone())
        } else {
            // Back is empty, peek from front (last element)
            let mut elements = Vec::new();
            let mut current_front = self.front.clone();

            while !Stack::is_empty(&current_front) {
                let (value, rest) = current_front.uncons().unwrap();
                elements.push(value);
                current_front = rest;
            }

            Ok(elements[elements.len() - 1].clone())
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut deque = Self::empty();
        for item in iter {
            deque = deque.push_back(item);
        }
        deque
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        // Push elements to the front
        for i in 1..=5 {
            deque = deque.push_front(i);
        }

        assert_eq!(deque.size(), 5);
        assert!(!deque.is_empty());

        // Pop all elements from the front and check order
        for i in 1..=5 {
            let (value, new_deque) = deque.pop_front().unwrap();
            assert_eq!(value, 6 - i);
            deque = new_deque;
        }

        assert!(deque.is_empty());
        assert!(deque.pop_front().is_err());
    }

    #[test]
    fn test_push_back_pop_back() {
        let mut deque = OptimizedDeque::empty();

        // Push elements to the back
        for i in 1..=5 {
            deque = deque.push_back(i);
        }

        assert_eq!(deque.size(), 5);
        assert!(!deque.is_empty());

        // Pop all elements from the back and check order
        for i in 1..=5 {
            let (value, new_deque) = deque.pop_back().unwrap();
            assert_eq!(value, 6 - i);
            deque = new_deque;
        }

        assert!(deque.is_empty());
        assert!(deque.pop_back().is_err());
    }

    #[test]
    fn test_push_front_pop_back() {
        let mut deque = OptimizedDeque::empty();

        // Push elements to the front
        for i in 1..=5 {
            deque = deque.push_front(i);
        }

        // Pop all elements from the back and check order
        for i in 1..=5 {
            let (value, new_deque) = deque.pop_back().unwrap();
            assert_eq!(value, i);
            deque = new_deque;
        }

        assert!(deque.is_empty());
    }

    #[test]
    fn test_push_back_pop_front() {
        let mut deque = OptimizedDeque::empty();

        // Push elements to the back
        for i in 1..=5 {
            deque = deque.push_back(i);
        }

        // Pop all elements from the front and check order
        for i in 1..=5 {
            let (value, new_deque) = deque.pop_front().unwrap();
            assert_eq!(value, i);
            deque = new_deque;
        }

        assert!(deque.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut deque = OptimizedDeque::empty();
        deque = deque.push_front(1);
        deque = deque.push_back(2);

        assert_eq!(deque.peek_front().unwrap(), 1);
        assert_eq!(deque.peek_back().unwrap(), 2);
    }

    #[test]
    fn test_from_iter() {
        let values = vec![1, 2, 3, 4, 5];
        let deque = OptimizedDeque::from_iter(values.clone());

        assert_eq!(deque.size(), 5);

        // Pop all elements from the front and check order
        let mut deque = deque;
        for value in values.iter() {
            let (popped, new_deque) = deque.pop_front().unwrap();
            assert_eq!(popped, *value);
            deque = new_deque;
        }

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
        assert_eq!(value, 5);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 6);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 7);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 8);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 9);
        deque = new_deque;

        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 10);
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
