use rust_fp_categories::Empty;
use std::fmt::Debug;

/// Error type for FingerTree operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FingerTreeError {
    /// Error when trying to access an element from an empty tree
    EmptyTreeError,
}

/// A trait for persistent finger tree data structures.
///
/// A finger tree is a functional data structure that provides efficient
/// sequence operations, particularly concatenation.
/// All operations create a new tree instance, preserving the original.
pub trait FingerTree<A: Clone + Debug>: Empty {
    /// Adds an element to the front of the tree.
    ///
    /// Returns a new tree with the element added.
    fn push_front(self, value: A) -> Self;

    /// Adds an element to the back of the tree.
    ///
    /// Returns a new tree with the element added.
    fn push_back(self, value: A) -> Self;

    /// Removes an element from the front of the tree.
    ///
    /// Returns a tuple containing the removed element and the new tree,
    /// or an error if the tree is empty.
    fn pop_front(self) -> Result<(A, Self), FingerTreeError>
    where
        Self: Sized;

    /// Removes an element from the back of the tree.
    ///
    /// Returns a tuple containing the removed element and the new tree,
    /// or an error if the tree is empty.
    fn pop_back(self) -> Result<(A, Self), FingerTreeError>
    where
        Self: Sized;

    /// Returns the element at the front of the tree without removing it.
    ///
    /// Returns an error if the tree is empty.
    fn peek_front(&self) -> Result<A, FingerTreeError>;

    /// Returns the element at the back of the tree without removing it.
    ///
    /// Returns an error if the tree is empty.
    fn peek_back(&self) -> Result<A, FingerTreeError>;

    /// Concatenates two trees.
    ///
    /// Returns a new tree containing all elements from both trees.
    fn concat(self, other: Self) -> Self;

    /// Splits the tree at the specified index.
    ///
    /// Returns a tuple containing two new trees.
    fn split(self, index: usize) -> (Self, Self)
    where
        Self: Sized;

    /// Returns the number of elements in the tree.
    fn size(&self) -> usize;

    /// Creates a tree from an iterator.
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
