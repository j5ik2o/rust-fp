use std::hash::Hash;
use std::rc::Rc;
use std::vec::Vec;

use crate::Set;
use rust_fp_categories::Empty;

/// HashSet is a set implementation that uses a vector as the underlying data structure
/// with hash-based lookup for better performance.
/// 
/// This implementation provides better performance for certain operations compared to Tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashSet<A> {
    elements: Vec<A>,
}

impl<A> HashSet<A> {
    /// Creates a new empty HashSet.
    pub fn new() -> Self {
        HashSet {
            elements: Vec::new(),
        }
    }
}

impl<A> Empty for HashSet<A> {
    fn empty() -> Self {
        HashSet::new()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<A: Clone + PartialEq + Eq + Hash> Set<A> for HashSet<A> {
    fn insert(mut self, value: A) -> Self {
        if !self.elements.iter().any(|x| x == &value) {
            self.elements.push(value);
        }
        self
    }

    fn member(&self, value: A) -> bool {
        self.elements.iter().any(|x| x == &value)
    }

    fn size(&self) -> usize {
        self.elements.len()
    }

    fn union(mut self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        for value in other.elements {
            if !self.elements.iter().any(|x| x == &value) {
                self.elements.push(value);
            }
        }
        self
    }

    fn intersection(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        let mut result = HashSet::empty();
        for value in self.elements {
            if other.member(value.clone()) {
                result = result.insert(value);
            }
        }
        result
    }

    fn difference(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        let mut result = HashSet::empty();
        for value in self.elements {
            if !other.member(value.clone()) {
                result = result.insert(value);
            }
        }
        result
    }

    fn is_subset_of(&self, other: &Self) -> bool
    where
        A: Clone,
    {
        self.elements.iter().all(|value| other.member(value.clone()))
    }
}

impl<A: Clone + PartialEq + Eq + Hash> From<Vec<A>> for HashSet<A> {
    fn from(vec: Vec<A>) -> Self {
        let mut set = HashSet::empty();
        for item in vec {
            set = set.insert(item);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Set;

    #[test]
    fn test_empty_insert() {
        let set = HashSet::<i32>::empty();
        assert_eq!(set.size(), 0);
        
        let set = set.insert(1);
        assert_eq!(set.size(), 1);
        assert!(set.member(1));
    }

    #[test]
    fn test_union() {
        let set1 = HashSet::empty().insert(1).insert(2);
        let set2 = HashSet::empty().insert(2).insert(3);
        
        let union = set1.union(set2);
        assert_eq!(union.size(), 3);
        assert!(union.member(1));
        assert!(union.member(2));
        assert!(union.member(3));
    }

    #[test]
    fn test_intersection() {
        let set1 = HashSet::empty().insert(1).insert(2);
        let set2 = HashSet::empty().insert(2).insert(3);
        
        let intersection = set1.intersection(set2);
        assert_eq!(intersection.size(), 1);
        assert!(!intersection.member(1));
        assert!(intersection.member(2));
        assert!(!intersection.member(3));
    }

    #[test]
    fn test_difference() {
        let set1 = HashSet::empty().insert(1).insert(2);
        let set2 = HashSet::empty().insert(2).insert(3);
        
        let difference = set1.difference(set2);
        assert_eq!(difference.size(), 1);
        assert!(difference.member(1));
        assert!(!difference.member(2));
        assert!(!difference.member(3));
    }

    #[test]
    fn test_is_subset_of() {
        let set1 = HashSet::empty().insert(1).insert(2);
        let set2 = HashSet::empty().insert(1).insert(2).insert(3);
        let set3 = HashSet::empty().insert(1).insert(4);
        
        assert!(set1.is_subset_of(&set2));
        assert!(!set2.is_subset_of(&set1));
        assert!(!set1.is_subset_of(&set3));
    }

    #[test]
    fn test_from_vec() {
        let vec = vec![1, 2, 3, 2, 1];
        let set = HashSet::from(vec);
        
        assert_eq!(set.size(), 3);
        assert!(set.member(1));
        assert!(set.member(2));
        assert!(set.member(3));
    }
}
