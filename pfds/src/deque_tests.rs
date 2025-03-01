//! Common tests for Deque implementations.
//!
//! This module provides a set of tests that can be used to verify
//! the correctness of any implementation of the Deque trait.

use crate::Deque;
use rust_fp_categories::Empty;

/// Tests an empty deque.
pub fn test_empty_deque<D, A>(empty: D)
where
    D: Deque<A>,
    A: Clone + PartialEq + std::fmt::Debug,
{
    assert!(rust_fp_categories::Empty::is_empty(&empty));
    assert_eq!(empty.size(), 0);
    assert!(empty.peek_front().is_err());
    assert!(empty.peek_back().is_err());
}

/// Tests push_front and pop_front operations.
pub fn test_push_front_pop_front<D, A>(empty: D, values: Vec<A>)
where
    D: Deque<A>,
    A: Clone + PartialEq + std::fmt::Debug,
{
    let mut deque = empty;

    // Push all values to the front
    for value in values.iter().rev() {
        deque = deque.push_front(value.clone());
    }

    assert_eq!(deque.size(), values.len());
    assert!(!rust_fp_categories::Empty::is_empty(&deque));

    // Pop all values from the front and check order
    for value in values.iter() {
        let (popped, new_deque) = deque.pop_front().unwrap();
        assert_eq!(popped, *value);
        deque = new_deque;
    }

    assert!(rust_fp_categories::Empty::is_empty(&deque));
    assert!(deque.pop_front().is_err());
}

/// Tests push_back and pop_back operations.
pub fn test_push_back_pop_back<D, A>(empty: D, values: Vec<A>)
where
    D: Deque<A>,
    A: Clone + PartialEq + std::fmt::Debug,
{
    let mut deque = empty;

    // Push all values to the back
    for value in values.iter() {
        deque = deque.push_back(value.clone());
    }

    assert_eq!(deque.size(), values.len());
    assert!(!rust_fp_categories::Empty::is_empty(&deque));

    // Pop all values from the back and check order
    for value in values.iter().rev() {
        let (popped, new_deque) = deque.pop_back().unwrap();
        assert_eq!(popped, *value);
        deque = new_deque;
    }

    assert!(rust_fp_categories::Empty::is_empty(&deque));
    assert!(deque.pop_back().is_err());
}

/// Tests push_front and pop_back operations.
pub fn test_push_front_pop_back<D, A>(empty: D, values: Vec<A>)
where
    D: Deque<A>,
    A: Clone + PartialEq + std::fmt::Debug,
{
    let mut deque = empty;

    // Push all values to the front
    for value in values.iter() {
        deque = deque.push_front(value.clone());
    }

    // Pop all values from the back and check order
    for value in values.iter() {
        let (popped, new_deque) = deque.pop_back().unwrap();
        assert_eq!(popped, *value);
        deque = new_deque;
    }

    assert!(rust_fp_categories::Empty::is_empty(&deque));
}

/// Tests push_back and pop_front operations.
pub fn test_push_back_pop_front<D, A>(empty: D, values: Vec<A>)
where
    D: Deque<A>,
    A: Clone + PartialEq + std::fmt::Debug,
{
    let mut deque = empty;

    // Push all values to the back
    for value in values.iter() {
        deque = deque.push_back(value.clone());
    }

    // Pop all values from the front and check order
    for value in values.iter() {
        let (popped, new_deque) = deque.pop_front().unwrap();
        assert_eq!(popped, *value);
        deque = new_deque;
    }

    assert!(rust_fp_categories::Empty::is_empty(&deque));
}

/// Tests peek_front and peek_back operations.
pub fn test_peek<D, A>(empty: D, values: Vec<A>)
where
    D: Deque<A>,
    A: Clone + PartialEq + std::fmt::Debug,
{
    let mut deque = empty;

    // Push values from both ends
    deque = deque.push_front(values[0].clone());
    deque = deque.push_back(values[1].clone());

    // Check peek operations
    assert_eq!(deque.peek_front().unwrap(), values[0]);
    assert_eq!(deque.peek_back().unwrap(), values[1]);

    // Pop from front and check peek again
    let (_, new_deque) = deque.pop_front().unwrap();
    deque = new_deque;

    assert_eq!(deque.peek_front().unwrap(), values[1]);
    assert_eq!(deque.peek_back().unwrap(), values[1]);
}

/// Tests from_iter operation.
pub fn test_from_iter<D, A>(values: Vec<A>)
where
    D: Deque<A>,
    A: Clone + PartialEq + std::fmt::Debug,
{
    let deque = D::from_iter(values.clone());

    assert_eq!(deque.size(), values.len());

    // Pop all values from the front and check order
    let mut deque = deque;
    for value in values.iter() {
        let (popped, new_deque) = deque.pop_front().unwrap();
        assert_eq!(popped, *value);
        deque = new_deque;
    }

    assert!(rust_fp_categories::Empty::is_empty(&deque));
}

/// Tests mixed operations.
pub fn test_mixed_operations<D, A>(empty: D, values: Vec<A>)
where
    D: Deque<A>,
    A: Clone + PartialEq + std::fmt::Debug,
{
    let mut deque = empty;

    // Push elements from both ends
    deque = deque.push_front(values[0].clone());
    deque = deque.push_back(values[1].clone());
    deque = deque.push_front(values[2].clone());
    deque = deque.push_back(values[3].clone());

    // Expected order: [values[2], values[0], values[1], values[3]]
    assert_eq!(deque.size(), 4);

    // Check peek operations
    assert_eq!(deque.peek_front().unwrap(), values[2]);
    assert_eq!(deque.peek_back().unwrap(), values[3]);

    // Pop from front
    let (value, new_deque) = deque.pop_front().unwrap();
    assert_eq!(value, values[2]);
    deque = new_deque;

    // Pop from back
    let (value, new_deque) = deque.pop_back().unwrap();
    assert_eq!(value, values[3]);
    deque = new_deque;

    // Expected order: [values[0], values[1]]
    assert_eq!(deque.size(), 2);
    assert_eq!(deque.peek_front().unwrap(), values[0]);
    assert_eq!(deque.peek_back().unwrap(), values[1]);
}

/// Tests large deque operations.
pub fn test_large_deque<D>(empty: D)
where
    D: Deque<i32>,
{
    let mut deque = empty;

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

    assert!(rust_fp_categories::Empty::is_empty(&deque));
}
