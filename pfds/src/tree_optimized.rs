use std::boxed::Box;

use crate::Set;
use rust_fp_categories::Empty;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum Tree<A>
where
    A: Eq,
{
    Empty,
    Cons(Box<Self>, A, Box<Self>),
}

impl<A: Eq> Tree<A> {
    pub fn single(value: A) -> Self {
        Self::cons(Tree::Empty, value, Tree::Empty)
    }

    pub fn cons(left: Self, value: A, right: Self) -> Self {
        Tree::Cons(Box::new(left), value, Box::new(right))
    }
}

impl<A: Eq> Empty for Tree<A> {
    fn empty() -> Self {
        Tree::Empty
    }

    fn is_empty(&self) -> bool {
        match self {
            &Tree::Empty => true,
            &Tree::Cons(..) => false,
        }
    }
}

// Category trait implementations are provided by TreeWrapper
// See tree_wrapper.rs for implementations

impl<A: Clone + PartialEq + PartialOrd + Eq> Set<A> for Tree<A> {
    // Optimized insert implementation to reduce unnecessary clones
    fn insert(self, value: A) -> Self {
        // Helper function to check if a value exists in the tree
        fn value_exists<A: Clone + PartialEq + PartialOrd + Eq>(tree: &Tree<A>, value: &A) -> bool {
            match tree {
                Tree::Empty => false,
                Tree::Cons(left, y, right) => {
                    if value < y {
                        value_exists(left, value)
                    } else if y < value {
                        value_exists(right, value)
                    } else {
                        true // Value found
                    }
                }
            }
        }

        // If the value already exists in the tree, return the original tree
        if value_exists(&self, &value) {
            return self;
        }

        // Iterative implementation using a stack to track the path
        let mut stack = Vec::new();
        let mut current = self;

        // First phase: traverse the tree and build a path stack
        loop {
            match current {
                Tree::Empty => {
                    // Found insertion point
                    current = Tree::cons(Tree::Empty, value.clone(), Tree::Empty);
                    break;
                }
                Tree::Cons(left, y, right) => {
                    if value < y {
                        // Go left
                        stack.push((false, y, right));
                        current = *left;
                    } else if y < value {
                        // Go right
                        stack.push((true, y, left));
                        current = *right;
                    } else {
                        // This should not happen as we already checked for existence
                        unreachable!();
                    }
                }
            }
        }

        // Second phase: rebuild the tree from the bottom up
        while let Some((went_right, y, other)) = stack.pop() {
            if went_right {
                // We went right, so 'other' is the left subtree
                current = Tree::Cons(other, y, Box::new(current));
            } else {
                // We went left, so 'other' is the right subtree
                current = Tree::Cons(Box::new(current), y, other);
            }
        }

        current
    }

    // Optimized member implementation to reduce recursion
    fn member(&self, value: A) -> bool {
        let mut current = self;
        let mut last_value: Option<&A> = None;

        // Iterative implementation
        loop {
            match current {
                Tree::Empty => {
                    // Check if the last value matches
                    return last_value.map_or(false, |y| value == *y);
                }
                Tree::Cons(left, y, right) => {
                    if value < *y {
                        // Go left
                        current = left;
                    } else {
                        // Go right or found
                        last_value = Some(y);
                        current = right;
                    }
                }
            }
        }
    }

    fn size(&self) -> usize {
        match self {
            &Tree::Empty => 0,
            &Tree::Cons(ref a, _, ref b) => 1 + a.size() + b.size(),
        }
    }

    fn union(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        fn fold_insert<A: Clone + PartialEq + PartialOrd + Eq>(acc: Tree<A>, value: A) -> Tree<A> {
            acc.insert(value)
        }

        fn fold_tree<A: Clone + PartialEq + PartialOrd + Eq>(
            acc: Tree<A>,
            tree: &Tree<A>,
        ) -> Tree<A> {
            match tree {
                Tree::Empty => acc,
                Tree::Cons(left, value, right) => {
                    let acc = fold_tree(acc, left);
                    let acc = fold_insert(acc, value.clone());
                    fold_tree(acc, right)
                }
            }
        }

        fold_tree(self, &other)
    }

    fn intersection(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        fn fold_intersect<A: Clone + PartialEq + PartialOrd + Eq>(
            acc: Tree<A>,
            value: A,
            other: &Tree<A>,
        ) -> Tree<A> {
            if other.member(value.clone()) {
                acc.insert(value)
            } else {
                acc
            }
        }

        fn fold_tree<A: Clone + PartialEq + PartialOrd + Eq>(
            acc: Tree<A>,
            tree: &Tree<A>,
            other: &Tree<A>,
        ) -> Tree<A> {
            match tree {
                Tree::Empty => acc,
                Tree::Cons(left, value, right) => {
                    let acc = fold_tree(acc, left, other);
                    let acc = fold_intersect(acc, value.clone(), other);
                    fold_tree(acc, right, other)
                }
            }
        }

        fold_tree(Tree::empty(), &self, &other)
    }

    fn difference(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        fn fold_difference<A: Clone + PartialEq + PartialOrd + Eq>(
            acc: Tree<A>,
            value: A,
            other: &Tree<A>,
        ) -> Tree<A> {
            if !other.member(value.clone()) {
                acc.insert(value)
            } else {
                acc
            }
        }

        fn fold_tree<A: Clone + PartialEq + PartialOrd + Eq>(
            acc: Tree<A>,
            tree: &Tree<A>,
            other: &Tree<A>,
        ) -> Tree<A> {
            match tree {
                Tree::Empty => acc,
                Tree::Cons(left, value, right) => {
                    let acc = fold_tree(acc, left, other);
                    let acc = fold_difference(acc, value.clone(), other);
                    fold_tree(acc, right, other)
                }
            }
        }

        fold_tree(Tree::empty(), &self, &other)
    }

    fn is_subset_of(&self, other: &Self) -> bool
    where
        A: Clone,
    {
        fn check_subset<A: Clone + PartialEq + PartialOrd + Eq>(
            tree: &Tree<A>,
            other: &Tree<A>,
        ) -> bool {
            match tree {
                Tree::Empty => true,
                Tree::Cons(left, value, right) => {
                    other.member(value.clone())
                        && check_subset(left, other)
                        && check_subset(right, other)
                }
            }
        }

        check_subset(self, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Set, StackError, TreeOptimized as Tree};
    use rust_fp_categories::Empty;

    #[test]
    fn test_size() -> Result<(), StackError> {
        assert_eq!(Tree::single(1).size(), 1);
        Ok(())
    }

    #[test]
    fn test_empty_insert() {
        let set = Tree::<i32>::empty();
        assert_eq!(set.size(), 0);

        let set = set.insert(1);
        assert_eq!(set.size(), 1);
        assert!(set.member(1));
    }

    #[test]
    fn test_union() {
        let set1 = Tree::<i32>::empty().insert(1).insert(2);
        let set2 = Tree::<i32>::empty().insert(2).insert(3);

        let union = set1.union(set2);
        assert_eq!(union.size(), 3);
        assert!(union.member(1));
        assert!(union.member(2));
        assert!(union.member(3));
    }

    #[test]
    fn test_intersection() {
        let set1 = Tree::<i32>::empty().insert(1).insert(2);
        let set2 = Tree::<i32>::empty().insert(2).insert(3);

        let intersection = set1.intersection(set2);
        assert_eq!(intersection.size(), 1);
        assert!(!intersection.member(1));
        assert!(intersection.member(2));
        assert!(!intersection.member(3));
    }

    #[test]
    fn test_difference() {
        let set1 = Tree::<i32>::empty().insert(1).insert(2);
        let set2 = Tree::<i32>::empty().insert(2).insert(3);

        let difference = set1.difference(set2);
        assert_eq!(difference.size(), 1);
        assert!(difference.member(1));
        assert!(!difference.member(2));
        assert!(!difference.member(3));
    }

    #[test]
    fn test_is_subset_of() {
        let set1 = Tree::<i32>::empty().insert(1).insert(2);
        let set2 = Tree::<i32>::empty().insert(1).insert(2).insert(3);
        let set3 = Tree::<i32>::empty().insert(1).insert(4);

        assert!(set1.is_subset_of(&set2));
        assert!(!set2.is_subset_of(&set1));
        assert!(!set1.is_subset_of(&set3));
    }

    #[test]
    fn test_insert_existing() {
        let set = Tree::<i32>::empty().insert(1).insert(2);
        let set2 = set.clone().insert(2);

        // Inserting an existing element should not change the tree
        assert_eq!(set, set2);
        assert_eq!(set2.size(), 2);
    }

    #[test]
    fn test_member_not_found() {
        let set = Tree::<i32>::empty().insert(1).insert(3);

        // Test member with a value that doesn't exist
        assert!(!set.member(2));
        assert!(!set.member(4));
    }

    // Category trait tests are in TreeWrapper
}
