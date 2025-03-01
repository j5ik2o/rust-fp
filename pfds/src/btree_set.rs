use std::rc::Rc;
use std::vec::Vec;

use crate::Set;
use crate::Tree;
use rust_fp_categories::Empty;

/// BTreeSet is a set implementation that uses a balanced binary tree as the underlying data structure.
///
/// This implementation provides ordered iteration and efficient range queries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BTreeSet<A> {
    tree: Tree<A>,
}

impl<A> BTreeSet<A> {
    /// Creates a new empty BTreeSet.
    pub fn new() -> Self {
        BTreeSet {
            tree: Tree::empty(),
        }
    }
}

impl<A> Empty for BTreeSet<A> {
    fn empty() -> Self {
        BTreeSet::new()
    }

    fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
}

impl<A: Clone + PartialEq + PartialOrd> Set<A> for BTreeSet<A> {
    fn insert(self, value: A) -> Self {
        BTreeSet {
            tree: self.tree.insert(value),
        }
    }

    fn member(&self, value: A) -> bool {
        self.tree.member(value)
    }

    fn size(&self) -> usize {
        self.tree.size()
    }

    fn union(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        BTreeSet {
            tree: self.tree.union(other.tree),
        }
    }

    fn intersection(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        BTreeSet {
            tree: self.tree.intersection(other.tree),
        }
    }

    fn difference(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        BTreeSet {
            tree: self.tree.difference(other.tree),
        }
    }

    fn is_subset_of(&self, other: &Self) -> bool
    where
        A: Clone,
    {
        self.tree.is_subset_of(&other.tree)
    }
}

impl<A: Clone + PartialEq + PartialOrd> From<Vec<A>> for BTreeSet<A> {
    fn from(vec: Vec<A>) -> Self {
        let mut set = BTreeSet::empty();
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
        let set = BTreeSet::<i32>::empty();
        assert_eq!(set.size(), 0);

        let set = set.insert(1);
        assert_eq!(set.size(), 1);
        assert!(set.member(1));
    }

    #[test]
    fn test_union() {
        let set1 = BTreeSet::empty().insert(1).insert(2);
        let set2 = BTreeSet::empty().insert(2).insert(3);

        let union = set1.union(set2);
        assert_eq!(union.size(), 3);
        assert!(union.member(1));
        assert!(union.member(2));
        assert!(union.member(3));
    }

    #[test]
    fn test_intersection() {
        let set1 = BTreeSet::empty().insert(1).insert(2);
        let set2 = BTreeSet::empty().insert(2).insert(3);

        let intersection = set1.intersection(set2);
        assert_eq!(intersection.size(), 1);
        assert!(!intersection.member(1));
        assert!(intersection.member(2));
        assert!(!intersection.member(3));
    }

    #[test]
    fn test_difference() {
        let set1 = BTreeSet::empty().insert(1).insert(2);
        let set2 = BTreeSet::empty().insert(2).insert(3);

        let difference = set1.difference(set2);
        assert_eq!(difference.size(), 1);
        assert!(difference.member(1));
        assert!(!difference.member(2));
        assert!(!difference.member(3));
    }

    #[test]
    fn test_is_subset_of() {
        let set1 = BTreeSet::empty().insert(1).insert(2);
        let set2 = BTreeSet::empty().insert(1).insert(2).insert(3);
        let set3 = BTreeSet::empty().insert(1).insert(4);

        assert!(set1.is_subset_of(&set2));
        assert!(!set2.is_subset_of(&set1));
        assert!(!set1.is_subset_of(&set3));
    }

    #[test]
    fn test_from_vec() {
        let vec = vec![1, 2, 3, 2, 1];
        let set = BTreeSet::from(vec);

        assert_eq!(set.size(), 3);
        assert!(set.member(1));
        assert!(set.member(2));
        assert!(set.member(3));
    }
}
