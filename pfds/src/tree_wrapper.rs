use crate::Tree;
use rust_fp_categories::{Applicative, Apply, Bind, Empty, Foldable, Functor, Monad, Pure};
use std::marker::PhantomData;

/// A wrapper around Tree<A> that allows implementing category traits
/// This wrapper is specifically designed to implement category traits
/// for Tree<A> where A: Clone + Eq + PartialOrd
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct TreeWrapper<A>(pub Tree<A>)
where
    A: Clone + Eq + PartialOrd;

impl<A> TreeWrapper<A>
where
    A: Clone + Eq + PartialOrd,
{
    pub fn new(tree: Tree<A>) -> Self {
        TreeWrapper(tree)
    }

    pub fn unwrap(self) -> Tree<A> {
        self.0
    }

    // Helper method to create a TreeWrapper from a value
    // This is used by the Pure trait implementation
    fn create_single(value: A) -> Self {
        TreeWrapper(Tree::cons(Tree::Empty, value, Tree::Empty))
    }
}

impl<A> From<Tree<A>> for TreeWrapper<A>
where
    A: Clone + Eq + PartialOrd,
{
    fn from(tree: Tree<A>) -> Self {
        TreeWrapper(tree)
    }
}

impl<A> From<TreeWrapper<A>> for Tree<A>
where
    A: Clone + Eq + PartialOrd,
{
    fn from(wrapper: TreeWrapper<A>) -> Self {
        wrapper.0
    }
}

impl<A> Empty for TreeWrapper<A>
where
    A: Clone + Eq + PartialOrd,
{
    fn empty() -> Self {
        TreeWrapper(Tree::Empty)
    }

    fn is_empty(&self) -> bool {
        match self.0 {
            Tree::Empty => true,
            _ => false,
        }
    }
}

// Define a wrapper type for TreeWrapper that can be used with any type B
// This allows us to implement the category traits without adding extra bounds
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct TreeWrapperOption<B: Clone>(pub Option<B>);

impl<A> Functor for TreeWrapper<A>
where
    A: Clone + Eq + PartialOrd,
{
    type Elm = A;
    type M<B: Clone> = TreeWrapperOption<B>;

    fn fmap<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
        B: Clone,
    {
        match self.0 {
            Tree::Empty => TreeWrapperOption(None),
            Tree::Cons(_, value, _) => TreeWrapperOption(Some(f(&value))),
        }
    }
}

impl<A> Pure for TreeWrapper<A>
where
    A: Clone + Eq + PartialOrd,
{
    type Elm = A;
    type M<B: Clone> = TreeWrapperOption<B>;

    fn pure(value: A) -> Self::M<A> {
        TreeWrapperOption(Some(value))
    }

    fn unit() -> Self::M<()> {
        TreeWrapperOption(Some(()))
    }
}

impl<A> Apply for TreeWrapper<A>
where
    A: Clone + Eq + PartialOrd,
{
    type Elm = A;
    type M<B: Clone> = TreeWrapperOption<B>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&A) -> B + Clone,
        B: Clone,
    {
        match fs.0 {
            None => TreeWrapperOption(None),
            Some(f) => match self.0 {
                Tree::Empty => TreeWrapperOption(None),
                Tree::Cons(_, value, _) => TreeWrapperOption(Some(f(&value))),
            },
        }
    }
}

impl<A> Applicative for TreeWrapper<A> where A: Clone + Eq + PartialOrd {}

impl<A> Bind for TreeWrapper<A>
where
    A: Clone + Eq + PartialOrd,
{
    type Elm = A;
    type M<B: Clone> = TreeWrapperOption<B>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> Self::M<B>,
        B: Clone,
    {
        match self.0 {
            Tree::Empty => TreeWrapperOption(None),
            Tree::Cons(_, value, _) => f(&value),
        }
    }
}

impl<A> Monad for TreeWrapper<A> where A: Clone + Eq + PartialOrd {}

impl<A> Foldable for TreeWrapper<A>
where
    A: Clone + Eq + PartialOrd,
{
    type Elm = A;

    fn fold_left<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(B, &Self::Elm) -> B,
    {
        match &self.0 {
            Tree::Empty => b,
            Tree::Cons(left, value, right) => {
                // We need to avoid creating new TreeWrapper instances to prevent recursion limit issues
                // Instead, we'll implement a direct fold on the Tree structure
                let b1 = fold_tree_left(&**left, b, &f);
                let b2 = f(b1, value);
                fold_tree_left(&**right, b2, &f)
            }
        }
    }

    fn fold_right<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(&Self::Elm, B) -> B,
    {
        match &self.0 {
            Tree::Empty => b,
            Tree::Cons(left, value, right) => {
                // We need to avoid creating new TreeWrapper instances to prevent recursion limit issues
                // Instead, we'll implement a direct fold on the Tree structure
                let b1 = fold_tree_right(&**right, b, &f);
                let b2 = f(value, b1);
                fold_tree_right(&**left, b2, &f)
            }
        }
    }
}

// Helper function to fold a Tree from left to right without creating TreeWrapper instances
fn fold_tree_left<A, B, F>(tree: &Tree<A>, b: B, f: &F) -> B
where
    A: Clone + Eq + PartialOrd,
    F: Fn(B, &A) -> B,
{
    match tree {
        Tree::Empty => b,
        Tree::Cons(left, value, right) => {
            let b1 = fold_tree_left(&**left, b, f);
            let b2 = f(b1, value);
            fold_tree_left(&**right, b2, f)
        }
    }
}

// Helper function to fold a Tree from right to left without creating TreeWrapper instances
fn fold_tree_right<A, B, F>(tree: &Tree<A>, b: B, f: &F) -> B
where
    A: Clone + Eq + PartialOrd,
    F: Fn(&A, B) -> B,
{
    match tree {
        Tree::Empty => b,
        Tree::Cons(left, value, right) => {
            let b1 = fold_tree_right(&**right, b, f);
            let b2 = f(value, b1);
            fold_tree_right(&**left, b2, f)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Set, Tree};
    use rust_fp_categories::{Bind, Empty, Foldable, Functor, Pure};

    #[test]
    fn test_empty() {
        let empty_wrapper = TreeWrapper::<i32>::empty();
        assert!(empty_wrapper.is_empty());
    }

    #[test]
    fn test_functor() {
        let tree = Tree::empty().insert(1).insert(2).insert(3);
        let wrapper = TreeWrapper::new(tree.clone());

        // Since fmap returns TreeWrapperOption<B>, we need to test differently
        let result = wrapper.fmap(|x| x * 2);
        assert!(result.0.is_some());
        assert_eq!(result.0.unwrap(), 2); // Only gets the first value

        // We can still test the original tree
        let tree2 = Tree::empty().insert(1).insert(2).insert(3);
        assert_eq!(tree, tree2);
    }

    #[test]
    fn test_pure() {
        let result = TreeWrapper::<i32>::pure(5);
        assert!(result.0.is_some());
        assert_eq!(result.0.unwrap(), 5);
    }

    #[test]
    fn test_bind() {
        let tree = Tree::empty().insert(1).insert(2);
        let wrapper = TreeWrapper::new(tree);

        // Since bind returns TreeWrapperOption<B>, we need to test differently
        let result = wrapper.bind(|x| TreeWrapperOption(Some(x * 10)));
        assert!(result.0.is_some());
        assert_eq!(result.0.unwrap(), 10); // Only gets the first value
    }

    #[test]
    fn test_fold_left() {
        let tree = Tree::empty().insert(1).insert(2).insert(3);
        let wrapper = TreeWrapper::new(tree);
        let sum = wrapper.fold_left(0, |acc, x| acc + x);

        assert_eq!(sum, 6);
    }

    #[test]
    fn test_fold_right() {
        let tree = Tree::empty().insert(1).insert(2).insert(3);
        let wrapper = TreeWrapper::new(tree);
        let sum = wrapper.fold_right(0, |x, acc| x + acc);

        assert_eq!(sum, 6);
    }
}
