use std::rc::Rc;

use crate::{Deque, DequeError};
use rust_fp_categories::Empty;

/// A fixed-size array-based implementation of a double-ended queue (deque).
///
/// This implementation uses a circular buffer to efficiently implement a double-ended queue.
/// The circular buffer allows for efficient operations on both ends of the deque.
///
/// 固定サイズの配列ベースの両端キュー（deque）の実装。
///
/// この実装では、循環バッファを使用して両端キューを効率的に実装しています。
/// 循環バッファにより、dequeの両端での効率的な操作が可能になります。
/// 配列ベースの実装は、メモリ効率が良く、キャッシュ局所性に優れています。
///
/// Time complexity:
/// - push_front: O(1) amortized
/// - push_back: O(1) amortized
/// - pop_front: O(1)
/// - pop_back: O(1)
/// - peek_front: O(1)
/// - peek_back: O(1)
/// - size: O(1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayDeque<A> {
    buffer: Rc<Vec<Option<A>>>,
    front: usize,
    back: usize,
    size: usize,
    capacity: usize,
}

impl<A: Clone> ArrayDeque<A> {
    /// Creates a new empty deque with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        // Ensure capacity is at least 1
        let capacity = capacity.max(1);
        
        // Create a buffer with capacity + 1 to distinguish between empty and full
        let mut buffer = Vec::with_capacity(capacity + 1);
        for _ in 0..=capacity {
            buffer.push(None);
        }
        
        ArrayDeque {
            buffer: Rc::new(buffer),
            front: 0,
            back: 0,
            size: 0,
            capacity: capacity + 1,
        }
    }

    /// Creates a new empty deque with the default capacity.
    pub fn new() -> Self {
        Self::with_capacity(7) // Default capacity of 7 (resulting in a buffer of size 8)
    }

    /// Returns the index after the given index, wrapping around if necessary.
    fn next_index(&self, index: usize) -> usize {
        (index + 1) % self.capacity
    }

    /// Returns the index before the given index, wrapping around if necessary.
    fn prev_index(&self, index: usize) -> usize {
        (index + self.capacity - 1) % self.capacity
    }

    /// Checks if the deque is full.
    fn is_full(&self) -> bool {
        self.next_index(self.back) == self.front
    }

    /// Grows the capacity of the deque.
    fn grow(self) -> Self {
        let new_capacity = self.capacity * 2 - 1;
        let mut new_buffer = Vec::with_capacity(new_capacity);
        
        // Initialize new buffer with None
        for _ in 0..new_capacity {
            new_buffer.push(None);
        }
        
        // Copy elements from old buffer to new buffer
        let mut index = self.front;
        let mut new_index = 0;
        
        while index != self.back {
            new_buffer[new_index] = self.buffer[index].clone();
            index = self.next_index(index);
            new_index += 1;
        }
        
        ArrayDeque {
            buffer: Rc::new(new_buffer),
            front: 0,
            back: self.size,
            size: self.size,
            capacity: new_capacity,
        }
    }
}

impl<A: Clone> Empty for ArrayDeque<A> {
    fn empty() -> Self {
        ArrayDeque::new()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<A: Clone> Deque<A> for ArrayDeque<A> {
    fn push_front(self, value: A) -> Self {
        // If the deque is full, grow it
        if self.is_full() {
            return self.grow().push_front(value);
        }
        
        // Calculate the new front index
        let new_front = self.prev_index(self.front);
        
        // Create a new buffer with the value at the new front
        let buffer = match Rc::try_unwrap(self.buffer) {
            Ok(mut vec) => {
                vec[new_front] = Some(value);
                Rc::new(vec)
            }
            Err(rc) => {
                let mut new_buffer = (*rc).clone();
                new_buffer[new_front] = Some(value);
                Rc::new(new_buffer)
            }
        };
        
        ArrayDeque {
            buffer,
            front: new_front,
            back: self.back,
            size: self.size + 1,
            capacity: self.capacity,
        }
    }

    fn push_back(self, value: A) -> Self {
        // If the deque is full, grow it
        if self.is_full() {
            return self.grow().push_back(value);
        }
        
        // Calculate the new back index before modifying the buffer
        let new_back = self.next_index(self.back);
        
        // Create a new buffer with the value at the back
        let buffer = match Rc::try_unwrap(self.buffer) {
            Ok(mut vec) => {
                vec[self.back] = Some(value);
                Rc::new(vec)
            }
            Err(rc) => {
                let mut new_buffer = (*rc).clone();
                new_buffer[self.back] = Some(value);
                Rc::new(new_buffer)
            }
        };
        
        ArrayDeque {
            buffer,
            front: self.front,
            back: new_back,
            size: self.size + 1,
            capacity: self.capacity,
        }
    }

    fn pop_front(self) -> Result<(A, Self), DequeError> {
        if rust_fp_categories::Empty::is_empty(&self) {
            return Err(DequeError::EmptyDequeError);
        }
        
        // Get the value at the front
        let value = match &self.buffer[self.front] {
            Some(v) => v.clone(),
            None => return Err(DequeError::EmptyDequeError), // This should never happen
        };
        
        // Calculate the new front index before modifying the buffer
        let new_front = self.next_index(self.front);
        
        // Create a new buffer with None at the front
        let buffer = match Rc::try_unwrap(self.buffer) {
            Ok(mut vec) => {
                vec[self.front] = None;
                Rc::new(vec)
            }
            Err(rc) => {
                let mut new_buffer = (*rc).clone();
                new_buffer[self.front] = None;
                Rc::new(new_buffer)
            }
        };
        
        Ok((
            value,
            ArrayDeque {
                buffer,
                front: new_front,
                back: self.back,
                size: self.size - 1,
                capacity: self.capacity,
            },
        ))
    }

    fn pop_back(self) -> Result<(A, Self), DequeError> {
        if rust_fp_categories::Empty::is_empty(&self) {
            return Err(DequeError::EmptyDequeError);
        }
        
        // Calculate the index of the last element
        let last_index = self.prev_index(self.back);
        
        // Get the value at the back
        let value = match &self.buffer[last_index] {
            Some(v) => v.clone(),
            None => return Err(DequeError::EmptyDequeError), // This should never happen
        };
        
        // Create a new buffer with None at the back
        let buffer = match Rc::try_unwrap(self.buffer) {
            Ok(mut vec) => {
                vec[last_index] = None;
                Rc::new(vec)
            }
            Err(rc) => {
                let mut new_buffer = (*rc).clone();
                new_buffer[last_index] = None;
                Rc::new(new_buffer)
            }
        };
        
        Ok((
            value,
            ArrayDeque {
                buffer,
                front: self.front,
                back: last_index,
                size: self.size - 1,
                capacity: self.capacity,
            },
        ))
    }

    fn peek_front(&self) -> Result<A, DequeError> {
        if rust_fp_categories::Empty::is_empty(self) {
            return Err(DequeError::EmptyDequeError);
        }
        
        match &self.buffer[self.front] {
            Some(v) => Ok(v.clone()),
            None => Err(DequeError::EmptyDequeError), // This should never happen
        }
    }

    fn peek_back(&self) -> Result<A, DequeError> {
        if rust_fp_categories::Empty::is_empty(self) {
            return Err(DequeError::EmptyDequeError);
        }
        
        // Calculate the index of the last element
        let last_index = self.prev_index(self.back);
        
        match &self.buffer[last_index] {
            Some(v) => Ok(v.clone()),
            None => Err(DequeError::EmptyDequeError), // This should never happen
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let items: Vec<A> = iter.into_iter().collect();
        let size = items.len();
        
        if size == 0 {
            return ArrayDeque::empty();
        }
        
        // Create a deque with enough capacity for all items
        let mut deque = ArrayDeque::with_capacity(size);
        
        // Add all items to the deque
        for item in items {
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
        let deque: ArrayDeque<i32> = ArrayDeque::empty();
        assert!(deque.is_empty());
        assert_eq!(deque.size(), 0);
        assert!(deque.peek_front().is_err());
        assert!(deque.peek_back().is_err());
    }

    #[test]
    fn test_push_front_pop_front() {
        let mut deque = ArrayDeque::empty();
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
        let mut deque = ArrayDeque::empty();
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
        let mut deque = ArrayDeque::empty();
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
        let mut deque = ArrayDeque::empty();
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
        let deque = ArrayDeque::empty().push_front(1).push_back(2);

        assert_eq!(deque.peek_front().unwrap(), 1);
        assert_eq!(deque.peek_back().unwrap(), 2);

        let (_, deque) = deque.pop_front().unwrap();
        assert_eq!(deque.peek_front().unwrap(), 2);
        assert_eq!(deque.peek_back().unwrap(), 2);
    }

    #[test]
    fn test_from_iter() {
        let mut deque = ArrayDeque::from_iter(vec![1, 2, 3]);

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
        let mut deque = ArrayDeque::empty();

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
    fn test_grow() {
        // Create a deque with small capacity
        let mut deque = ArrayDeque::with_capacity(3);
        
        // Add elements until it needs to grow
        deque = deque.push_back(1);
        deque = deque.push_back(2);
        deque = deque.push_back(3);
        
        // This should trigger a grow
        deque = deque.push_back(4);
        
        // Check that all elements are still there
        assert_eq!(deque.size(), 4);
        
        // Pop all elements and check their values
        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 1);
        deque = new_deque;
        
        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 2);
        deque = new_deque;
        
        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 3);
        deque = new_deque;
        
        let (value, new_deque) = deque.pop_front().unwrap();
        assert_eq!(value, 4);
        deque = new_deque;
        
        assert!(deque.is_empty());
    }

    #[test]
    fn test_large_deque() {
        let mut deque = ArrayDeque::empty();

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
