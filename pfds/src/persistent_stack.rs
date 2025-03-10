use crate::stack::Stack;
use crate::StackError;
use rust_fp_categories::{Applicative, Apply, Bind, Empty, Foldable, Functor, Monad, Pure};
use std::rc::Rc;

/// PersistentStack is a fully persistent stack implementation.
///
/// This implementation ensures that all operations preserve the original stack,
/// making it suitable for functional programming patterns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistentStack<A> {
    Empty,
    Node(A, Rc<PersistentStack<A>>),
}

impl<A> PersistentStack<A> {
    /// Creates a new empty PersistentStack.
    pub fn new() -> Self {
        PersistentStack::Empty
    }
}

impl<A> Empty for PersistentStack<A> {
    fn empty() -> Self {
        PersistentStack::Empty
    }

    fn is_empty(&self) -> bool {
        match self {
            PersistentStack::Empty => true,
            _ => false,
        }
    }
}

impl<A: Clone> Stack<A> for PersistentStack<A> {
    fn cons(self, value: A) -> Self {
        PersistentStack::Node(value, Rc::new(self))
    }

    fn head(&self) -> Result<&A, StackError> {
        match self {
            PersistentStack::Empty => Err(StackError::NoSuchElementError),
            PersistentStack::Node(value, _) => Ok(value),
        }
    }

    fn peek(&self) -> Result<&A, StackError> {
        self.head()
    }

    fn tail(&self) -> Rc<Self> {
        match self {
            PersistentStack::Empty => Rc::new(PersistentStack::Empty),
            PersistentStack::Node(_, tail) => Rc::clone(tail),
        }
    }

    fn size(&self) -> usize {
        match self {
            PersistentStack::Empty => 0,
            PersistentStack::Node(_, tail) => 1 + tail.size(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            PersistentStack::Empty => true,
            _ => false,
        }
    }

    fn update(self, index: u32, new_value: A) -> Result<Self, StackError>
    where
        Self: Sized,
    {
        match self {
            PersistentStack::Empty => Err(StackError::IndexOutOfRangeError),
            PersistentStack::Node(value, tail) => match index {
                0 => Ok(PersistentStack::Node(new_value, tail)),
                _ => {
                    let updated_tail = tail.as_ref().clone().update(index - 1, new_value)?;
                    Ok(PersistentStack::Node(value, Rc::new(updated_tail)))
                }
            },
        }
    }

    fn get(&self, index: u32) -> Result<&A, StackError> {
        match self {
            PersistentStack::Empty => Err(StackError::NoSuchElementError),
            PersistentStack::Node(value, tail) => match index {
                0 => Ok(value),
                _ => tail.get(index - 1),
            },
        }
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut stack = PersistentStack::empty();
        for item in iter {
            stack = stack.cons(item);
        }
        stack
    }

    fn uncons(self) -> Result<(A, Self), StackError> {
        match self {
            PersistentStack::Empty => Err(StackError::NoSuchElementError),
            PersistentStack::Node(value, tail) => Ok((value, (*tail).clone())),
        }
    }
}

impl<A: Clone> From<Vec<A>> for PersistentStack<A> {
    fn from(vec: Vec<A>) -> Self {
        vec.into_iter()
            .rev()
            .fold(PersistentStack::empty(), |acc, item| acc.cons(item))
    }
}

impl<A: Clone> Into<Vec<A>> for PersistentStack<A> {
    fn into(self) -> Vec<A> {
        let mut result = Vec::with_capacity(self.size());
        let mut current = self;
        while let PersistentStack::Node(value, tail) = current {
            result.push(value);
            current = tail.as_ref().clone();
        }
        result
    }
}

// Implement Functor for PersistentStack
impl<A: Clone> Functor for PersistentStack<A> {
    type Elm = A;
    type M<U: Clone> = PersistentStack<U>;

    fn fmap<B: Clone, F>(self, f: F) -> PersistentStack<B>
    where
        F: Fn(&A) -> B,
    {
        match self {
            PersistentStack::Empty => PersistentStack::Empty,
            PersistentStack::Node(value, tail) => {
                // Create a new function that captures f by reference
                let f_ref = &f;
                let mapped_tail = tail.as_ref().clone().fmap(|x| f_ref(x));
                PersistentStack::Node(f(&value), Rc::new(mapped_tail))
            }
        }
    }
}

// Implement Pure for PersistentStack
impl<A: Clone> Pure for PersistentStack<A> {
    type Elm = A;
    type M<U: Clone> = PersistentStack<U>;

    fn pure(value: A) -> PersistentStack<A> {
        PersistentStack::empty().cons(value)
    }

    fn unit() -> Self::M<()> {
        PersistentStack::empty().cons(())
    }
}

// Implement Apply for PersistentStack
impl<A: Clone> Apply for PersistentStack<A> {
    type Elm = A;
    type M<U: Clone> = PersistentStack<U>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&A) -> B,
    {
        match (self, fs) {
            (PersistentStack::Empty, _) => PersistentStack::Empty,
            (_, PersistentStack::Empty) => PersistentStack::Empty,
            (stack, PersistentStack::Node(f, fs_tail)) => {
                let mapped = stack.clone().fmap(f);
                let rest = stack.ap(fs_tail.as_ref().clone());
                mapped.combine(rest)
            }
        }
    }
}

// Helper method to combine two stacks
impl<A: Clone> PersistentStack<A> {
    fn combine(self, other: Self) -> Self {
        match self {
            PersistentStack::Empty => other,
            PersistentStack::Node(value, tail) => {
                PersistentStack::Node(value, Rc::new(tail.as_ref().clone().combine(other)))
            }
        }
    }
}

// Implement Applicative for PersistentStack
impl<A: Clone> Applicative for PersistentStack<A> {}

// Implement Bind for PersistentStack
impl<A: Clone> Bind for PersistentStack<A> {
    type Elm = A;
    type M<U: Clone> = PersistentStack<U>;

    fn bind<B: Clone, F>(self, f: F) -> PersistentStack<B>
    where
        F: Fn(&A) -> PersistentStack<B>,
    {
        match self {
            PersistentStack::Empty => PersistentStack::Empty,
            PersistentStack::Node(value, tail) => {
                let result = f(&value);
                // Create a new function that captures f by reference
                let f_ref = &f;
                let rest = tail.as_ref().clone().bind(|x| f_ref(x));
                result.combine(rest)
            }
        }
    }
}

// Implement Monad for PersistentStack
impl<A: Clone> Monad for PersistentStack<A> {}

// Implement Foldable for PersistentStack
impl<A: Clone> Foldable for PersistentStack<A> {
    type Elm = A;

    fn fold_left<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(B, &Self::Elm) -> B,
    {
        match self {
            PersistentStack::Empty => b,
            PersistentStack::Node(value, tail) => {
                let b1 = f(b, value);
                tail.fold_left(b1, f)
            }
        }
    }

    fn fold_right<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(&Self::Elm, B) -> B,
    {
        match self {
            PersistentStack::Empty => b,
            PersistentStack::Node(value, tail) => {
                let b1 = tail.fold_right(b, &f);
                f(value, b1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Stack;

    #[test]
    fn test_empty_cons() -> Result<(), StackError> {
        let stack = PersistentStack::empty().cons(1);
        assert_eq!(*stack.head()?, 1);
        assert_eq!(stack.size(), 1);
        Ok(())
    }

    #[test]
    fn test_peek() -> Result<(), StackError> {
        let stack = PersistentStack::empty().cons(1).cons(2);
        assert_eq!(*stack.peek()?, 2);
        Ok(())
    }

    #[test]
    fn test_is_empty() {
        let stack1 = PersistentStack::<i32>::empty();
        let stack2 = stack1.clone().cons(1);
        assert!(rust_fp_categories::Empty::is_empty(&stack1));
        assert!(!rust_fp_categories::Empty::is_empty(&stack2));
    }

    #[test]
    fn test_tail() -> Result<(), StackError> {
        let stack = PersistentStack::empty().cons(1).cons(2);
        let tail = stack.tail();
        assert_eq!(*tail.head()?, 1);
        assert_eq!(tail.size(), 1);
        Ok(())
    }

    #[test]
    fn test_update() -> Result<(), StackError> {
        let stack = PersistentStack::empty().cons(1).cons(2).cons(3);
        let updated = stack.clone().update(1, 5)?;

        // Original stack should remain unchanged
        assert_eq!(*stack.head()?, 3);
        assert_eq!(*stack.get(1)?, 2);

        // Updated stack should have the new value
        assert_eq!(*updated.head()?, 3);
        assert_eq!(*updated.get(1)?, 5);
        assert_eq!(*updated.get(2)?, 1);
        Ok(())
    }

    #[test]
    fn test_get() -> Result<(), StackError> {
        let stack = PersistentStack::empty().cons(1).cons(2).cons(3);
        assert_eq!(*stack.get(0)?, 3);
        assert_eq!(*stack.get(1)?, 2);
        assert_eq!(*stack.get(2)?, 1);
        Ok(())
    }

    #[test]
    fn test_from_iter() {
        let stack = PersistentStack::from_iter(vec![1, 2, 3]);
        assert_eq!(stack.size(), 3);
        let vec: Vec<i32> = stack.into();
        assert_eq!(vec, vec![3, 2, 1]);
    }

    #[test]
    fn test_from_vec_to_vec() {
        let vec = vec![1, 2, 3];
        let stack = PersistentStack::from(vec.clone());
        let vec2: Vec<i32> = stack.into();
        assert_eq!(vec, vec2);
    }

    #[test]
    fn test_persistence() -> Result<(), StackError> {
        let stack1 = PersistentStack::empty().cons(1).cons(2);
        let stack2 = stack1.clone().cons(3);

        // stack1 should remain unchanged
        assert_eq!(*stack1.head()?, 2);
        assert_eq!(stack1.size(), 2);

        // stack2 should have the new value
        assert_eq!(*stack2.head()?, 3);
        assert_eq!(stack2.size(), 3);

        Ok(())
    }
}
