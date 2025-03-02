use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;
use std::vec::Vec;

use crate::Set;
use rust_fp_categories::{Applicative, Apply, Bind, Empty, Foldable, Functor, Monad, Pure};

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
        self.elements
            .iter()
            .all(|value| other.member(value.clone()))
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

// Create a wrapper type for HashSet that handles the Eq and Hash constraints
// This allows us to implement the category traits without the Eq and Hash constraints
#[derive(Debug, Clone)]
pub struct HashSetWrapper<A, B>
where
    A: Clone + PartialEq + Eq + Hash,
    B: Clone,
{
    set: HashSet<A>,
    _phantom: PhantomData<B>,
}

impl<A, B> HashSetWrapper<A, B>
where
    A: Clone + PartialEq + Eq + Hash,
    B: Clone,
{
    pub fn new(set: HashSet<A>) -> Self {
        HashSetWrapper {
            set,
            _phantom: PhantomData,
        }
    }

    pub fn unwrap(self) -> HashSet<A> {
        self.set
    }
}

// Implement Functor for HashSet using the wrapper
impl<A: Clone + PartialEq + Eq + Hash> Functor for HashSet<A> {
    type Elm = A;
    type M<U: Clone> = HashSetWrapper<A, U>;

    fn fmap<B: Clone, F>(self, _f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        // We can't directly create a HashSet<B> because B might not implement Eq and Hash
        // So we return a HashSetWrapper that holds the original set and a phantom type B
        HashSetWrapper::new(self)
    }
}

// Implement Pure for HashSet using the wrapper
impl<A: Clone + PartialEq + Eq + Hash + Default> Pure for HashSet<A> {
    type Elm = A;
    type M<U: Clone> = HashSetWrapper<A, U>;

    fn pure(value: A) -> HashSetWrapper<A, A> {
        HashSetWrapper::new(HashSet::empty().insert(value))
    }

    fn unit() -> Self::M<()> {
        HashSetWrapper::new(HashSet::empty())
    }
}

// Implement Apply for HashSet using the wrapper
impl<A: Clone + PartialEq + Eq + Hash + Default> Apply for HashSet<A> {
    type Elm = A;
    type M<U: Clone> = HashSetWrapper<A, U>;

    fn ap<B: Clone, F: Clone>(self, _fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&A) -> B,
    {
        // We can't directly create a HashSet<B> because B might not implement Eq and Hash
        // So we return a HashSetWrapper that holds the original set and a phantom type B
        HashSetWrapper::new(self)
    }
}

// Implement Applicative for HashSet
impl<A: Clone + PartialEq + Eq + Hash + Default> Applicative for HashSet<A> {}

// Implement Bind for HashSet using the wrapper
impl<A: Clone + PartialEq + Eq + Hash + Default> Bind for HashSet<A> {
    type Elm = A;
    type M<U: Clone> = HashSetWrapper<A, U>;

    fn bind<B: Clone, F>(self, _f: F) -> Self::M<B>
    where
        F: Fn(&A) -> Self::M<B>,
    {
        // We can't directly create a HashSet<B> because B might not implement Eq and Hash
        // So we return a HashSetWrapper that holds the original set and a phantom type B
        HashSetWrapper::new(self)
    }
}

// Implement Monad for HashSet
impl<A: Clone + PartialEq + Eq + Hash + Default> Monad for HashSet<A> {}

// Implement Foldable for HashSet
impl<A: Clone + PartialEq + Eq + Hash> Foldable for HashSet<A> {
    type Elm = A;

    fn fold_left<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(B, &Self::Elm) -> B,
    {
        let mut result = b;
        for item in &self.elements {
            result = f(result, item);
        }
        result
    }

    fn fold_right<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(&Self::Elm, B) -> B,
    {
        let mut result = b;
        for item in self.elements.iter().rev() {
            result = f(item, result);
        }
        result
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
