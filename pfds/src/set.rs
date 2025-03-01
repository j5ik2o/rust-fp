use rust_fp_categories::Empty;

/// Set trait represents a set data structure.
///
/// A set is a collection of distinct elements.
pub trait Set<A>: Empty {
    /// Inserts a value into the set.
    ///
    /// If the value already exists in the set, the set is returned unchanged.
    fn insert(self, value: A) -> Self;

    /// Checks if a value is a member of the set.
    fn member(&self, value: A) -> bool;

    /// Returns the number of elements in the set.
    fn size(&self) -> usize;

    /// Returns the union of this set and another set.
    ///
    /// The union of two sets contains all elements that are in either set.
    fn union(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone;

    /// Returns the intersection of this set and another set.
    ///
    /// The intersection of two sets contains only the elements that are in both sets.
    fn intersection(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone;

    /// Returns the difference of this set and another set.
    ///
    /// The difference of two sets contains elements that are in the first set but not in the second set.
    fn difference(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone;

    /// Checks if this set is a subset of another set.
    ///
    /// A set is a subset of another set if all elements of the first set are also elements of the second set.
    fn is_subset_of(&self, other: &Self) -> bool
    where
        A: Clone;
}
