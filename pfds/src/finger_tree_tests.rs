use crate::{FingerTree, FingerTreeError, SimpleFingerTree};
use rust_fp_categories::Empty;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let tree: SimpleFingerTree<i32> = SimpleFingerTree::empty();
        assert!(tree.is_empty());
        assert_eq!(tree.size(), 0);
    }

    #[test]
    fn test_single() {
        let tree = SimpleFingerTree::single(42);
        assert!(!tree.is_empty());
        assert_eq!(tree.size(), 1);
        assert_eq!(tree.peek_front().unwrap(), 42);
        assert_eq!(tree.peek_back().unwrap(), 42);
    }

    #[test]
    fn test_push_front() {
        let tree = SimpleFingerTree::<i32>::empty()
            .push_front(3)
            .push_front(2)
            .push_front(1);

        assert_eq!(tree.size(), 3);
        assert_eq!(tree.peek_front().unwrap(), 1);
        assert_eq!(tree.peek_back().unwrap(), 3);
    }

    #[test]
    fn test_push_back() {
        let tree = SimpleFingerTree::<i32>::empty()
            .push_back(1)
            .push_back(2)
            .push_back(3);

        assert_eq!(tree.size(), 3);
        assert_eq!(tree.peek_front().unwrap(), 1);
        assert_eq!(tree.peek_back().unwrap(), 3);
    }

    #[test]
    fn test_pop_front() {
        let tree = SimpleFingerTree::<i32>::empty()
            .push_back(1)
            .push_back(2)
            .push_back(3);

        let (value, tree) = tree.pop_front().unwrap();
        assert_eq!(value, 1);
        assert_eq!(tree.size(), 2);

        let (value, tree) = tree.pop_front().unwrap();
        assert_eq!(value, 2);
        assert_eq!(tree.size(), 1);

        let (value, tree) = tree.pop_front().unwrap();
        assert_eq!(value, 3);
        assert_eq!(tree.size(), 0);

        assert!(tree.pop_front().is_err());
    }

    #[test]
    fn test_pop_back() {
        let tree = SimpleFingerTree::<i32>::empty()
            .push_back(1)
            .push_back(2)
            .push_back(3);

        let (value, tree) = tree.pop_back().unwrap();
        assert_eq!(value, 3);
        assert_eq!(tree.size(), 2);

        let (value, tree) = tree.pop_back().unwrap();
        assert_eq!(value, 2);
        assert_eq!(tree.size(), 1);

        let (value, tree) = tree.pop_back().unwrap();
        assert_eq!(value, 1);
        assert_eq!(tree.size(), 0);

        assert!(tree.pop_back().is_err());
    }

    #[test]
    fn test_peek_front() {
        let empty_tree = SimpleFingerTree::<i32>::empty();
        assert!(empty_tree.peek_front().is_err());

        let tree = empty_tree.push_back(1).push_back(2);
        assert_eq!(tree.peek_front().unwrap(), 1);
    }

    #[test]
    fn test_peek_back() {
        let empty_tree = SimpleFingerTree::<i32>::empty();
        assert!(empty_tree.peek_back().is_err());

        let tree = empty_tree.push_back(1).push_back(2);
        assert_eq!(tree.peek_back().unwrap(), 2);
    }

    #[test]
    fn test_concat() {
        let tree1 = SimpleFingerTree::<i32>::empty().push_back(1).push_back(2);

        let tree2 = SimpleFingerTree::<i32>::empty().push_back(3).push_back(4);

        let combined = tree1.concat(tree2);
        assert_eq!(combined.size(), 4);

        let (value, combined) = combined.pop_front().unwrap();
        assert_eq!(value, 1);

        let (value, combined) = combined.pop_front().unwrap();
        assert_eq!(value, 2);

        let (value, combined) = combined.pop_front().unwrap();
        assert_eq!(value, 3);

        let (value, _) = combined.pop_front().unwrap();
        assert_eq!(value, 4);
    }

    #[test]
    fn test_split() {
        let tree = SimpleFingerTree::<i32>::empty()
            .push_back(1)
            .push_back(2)
            .push_back(3)
            .push_back(4);

        // Split at index 0
        let (left, right) = tree.clone().split(0);
        assert_eq!(left.size(), 0);
        assert_eq!(right.size(), 4);

        // Split at index 2
        let (left, right) = tree.clone().split(2);
        assert_eq!(left.size(), 2);
        assert_eq!(right.size(), 2);
        assert_eq!(left.peek_back().unwrap(), 2);
        assert_eq!(right.peek_front().unwrap(), 3);

        // Split at end
        let (left, right) = tree.clone().split(4);
        assert_eq!(left.size(), 4);
        assert_eq!(right.size(), 0);
    }

    #[test]
    fn test_from_iter() {
        let vec = vec![1, 2, 3, 4, 5];
        let tree = SimpleFingerTree::from_iter(vec.clone());

        assert_eq!(tree.size(), 5);

        let mut result = Vec::new();
        let mut current_tree = tree;

        while let Ok((value, new_tree)) = current_tree.pop_front() {
            result.push(value);
            current_tree = new_tree;
        }

        assert_eq!(result, vec);
    }

    #[test]
    fn test_large_tree() {
        let mut tree = SimpleFingerTree::<i32>::empty();

        // Add 1000 elements
        for i in 0..1000 {
            tree = tree.push_back(i);
        }

        assert_eq!(tree.size(), 1000);
        assert_eq!(tree.peek_front().unwrap(), 0);
        assert_eq!(tree.peek_back().unwrap(), 999);

        // Split the tree
        let (left, right) = tree.split(500);
        assert_eq!(left.size(), 500);
        assert_eq!(right.size(), 500);

        // Check elements in left tree
        let mut left_values = Vec::new();
        let mut current_tree = left.clone();
        while let Ok((value, new_tree)) = current_tree.pop_front() {
            left_values.push(value);
            current_tree = new_tree;
        }
        assert_eq!(left_values.len(), 500);
        assert_eq!(left_values[0], 0);
        assert_eq!(left_values[499], 499);

        // Check elements in right tree
        let mut right_values = Vec::new();
        let mut current_tree = right.clone();
        while let Ok((value, new_tree)) = current_tree.pop_front() {
            right_values.push(value);
            current_tree = new_tree;
        }
        assert_eq!(right_values.len(), 500);
        assert_eq!(right_values[0], 500);
        assert_eq!(right_values[499], 999);
    }

    #[test]
    fn test_error_handling() {
        let empty_tree = SimpleFingerTree::<i32>::empty();

        match empty_tree.clone().pop_front() {
            Err(FingerTreeError::EmptyTreeError) => (),
            _ => panic!("Expected EmptyTreeError"),
        }

        match empty_tree.clone().pop_back() {
            Err(FingerTreeError::EmptyTreeError) => (),
            _ => panic!("Expected EmptyTreeError"),
        }

        match empty_tree.clone().peek_front() {
            Err(FingerTreeError::EmptyTreeError) => (),
            _ => panic!("Expected EmptyTreeError"),
        }

        match empty_tree.peek_back() {
            Err(FingerTreeError::EmptyTreeError) => (),
            _ => panic!("Expected EmptyTreeError"),
        }
    }

    #[test]
    fn test_mixed_operations() {
        let mut tree = SimpleFingerTree::<i32>::empty();

        // Mix of push_front and push_back
        tree = tree.push_front(3);
        tree = tree.push_back(4);
        tree = tree.push_front(2);
        tree = tree.push_back(5);
        tree = tree.push_front(1);

        assert_eq!(tree.size(), 5);
        assert_eq!(tree.peek_front().unwrap(), 1);
        assert_eq!(tree.peek_back().unwrap(), 5);

        // Mix of pop_front and pop_back
        let (value, tree) = tree.pop_front().unwrap();
        assert_eq!(value, 1);

        let (value, tree) = tree.pop_back().unwrap();
        assert_eq!(value, 5);

        let (value, tree) = tree.pop_front().unwrap();
        assert_eq!(value, 2);

        let (value, tree) = tree.pop_back().unwrap();
        assert_eq!(value, 4);

        let (value, tree) = tree.pop_front().unwrap();
        assert_eq!(value, 3);

        assert!(tree.is_empty());
    }
}
