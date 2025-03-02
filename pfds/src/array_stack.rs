use crate::stack::Stack;
use crate::StackError;
use rust_fp_categories::{Applicative, Apply, Bind, Empty, Foldable, Functor, Monad, Pure};
use std::rc::Rc;

/// ArrayStack is a stack implementation that uses a vector as the underlying data structure.
///
/// This implementation provides better performance for certain operations compared to List.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayStack<A> {
    elements: Vec<A>,
}

impl<A> ArrayStack<A> {
    /// Creates a new empty ArrayStack.
    pub fn new() -> Self {
        ArrayStack {
            elements: Vec::new(),
        }
    }
}

impl<A> Empty for ArrayStack<A> {
    fn empty() -> Self {
        ArrayStack::new()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<A: Clone> Stack<A> for ArrayStack<A> {
    fn cons(mut self, value: A) -> Self {
        self.elements.push(value);
        self
    }

    fn head(&self) -> Result<&A, StackError> {
        self.elements.last().ok_or(StackError::NoSuchElementError)
    }

    fn peek(&self) -> Result<&A, StackError> {
        self.head()
    }

    fn tail(&self) -> Rc<Self> {
        if rust_fp_categories::Empty::is_empty(self) {
            return Rc::new(ArrayStack::empty());
        }

        let mut new_stack = ArrayStack::empty();
        for i in 0..self.elements.len() - 1 {
            new_stack.elements.push(self.elements[i].clone());
        }
        Rc::new(new_stack)
    }

    fn size(&self) -> usize {
        self.elements.len()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn update(mut self, index: u32, new_value: A) -> Result<Self, StackError>
    where
        Self: Sized,
    {
        let idx = self
            .size()
            .checked_sub(1 + index as usize)
            .ok_or(StackError::IndexOutOfRangeError)?;

        if idx >= self.elements.len() {
            return Err(StackError::IndexOutOfRangeError);
        }

        self.elements[idx] = new_value;
        Ok(self)
    }

    fn get(&self, index: u32) -> Result<&A, StackError> {
        let idx = self
            .size()
            .checked_sub(1 + index as usize)
            .ok_or(StackError::IndexOutOfRangeError)?;

        self.elements.get(idx).ok_or(StackError::NoSuchElementError)
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut stack = ArrayStack::empty();
        for item in iter {
            stack = stack.cons(item);
        }
        stack
    }

    fn uncons(self) -> Result<(A, Self), StackError> {
        if rust_fp_categories::Empty::is_empty(&self) {
            Err(StackError::NoSuchElementError)
        } else {
            let mut new_elements = Vec::with_capacity(self.elements.len() - 1);
            new_elements.extend(self.elements.iter().skip(1).cloned());
            Ok((
                self.elements[0].clone(),
                ArrayStack {
                    elements: new_elements,
                },
            ))
        }
    }
}

impl<A: Clone> From<Vec<A>> for ArrayStack<A> {
    fn from(vec: Vec<A>) -> Self {
        ArrayStack { elements: vec }
    }
}

impl<A: Clone> Into<Vec<A>> for ArrayStack<A> {
    fn into(self) -> Vec<A> {
        self.elements
    }
}

// Implement Functor for ArrayStack
impl<A: Clone> Functor for ArrayStack<A> {
    type Elm = A;
    type M<U: Clone> = ArrayStack<U>;

    fn fmap<B: Clone, F>(self, f: F) -> ArrayStack<B>
    where
        F: Fn(&A) -> B,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            ArrayStack::empty()
        } else {
            let mut result = ArrayStack::empty();
            for item in self.elements.iter() {
                result = result.cons(f(item));
            }
            result
        }
    }
}

// Implement Pure for ArrayStack
impl<A: Clone> Pure for ArrayStack<A> {
    type Elm = A;
    type M<U: Clone> = ArrayStack<U>;

    fn pure(value: A) -> ArrayStack<A> {
        ArrayStack::empty().cons(value)
    }

    fn unit() -> Self::M<()> {
        ArrayStack::empty().cons(())
    }
}

// Implement Apply for ArrayStack
impl<A: Clone> Apply for ArrayStack<A> {
    type Elm = A;
    type M<U: Clone> = ArrayStack<U>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&A) -> B,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            ArrayStack::empty()
        } else {
            let mut result = ArrayStack::empty();
            for f in fs.elements.iter() {
                for a in self.elements.iter() {
                    result = result.cons(f(a));
                }
            }
            result
        }
    }
}

// Implement Applicative for ArrayStack
impl<A: Clone> Applicative for ArrayStack<A> {}

// Implement Bind for ArrayStack
impl<A: Clone> Bind for ArrayStack<A> {
    type Elm = A;
    type M<U: Clone> = ArrayStack<U>;

    fn bind<B: Clone, F>(self, f: F) -> ArrayStack<B>
    where
        F: Fn(&A) -> ArrayStack<B>,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            ArrayStack::empty()
        } else {
            let mut result = ArrayStack::empty();
            for item in self.elements.iter() {
                let inner_stack = f(item);
                for inner_item in inner_stack.elements.iter() {
                    result = result.cons(inner_item.clone());
                }
            }
            result
        }
    }
}

// Implement Monad for ArrayStack
impl<A: Clone> Monad for ArrayStack<A> {}

// Implement Foldable for ArrayStack
impl<A: Clone> Foldable for ArrayStack<A> {
    type Elm = A;

    fn fold_left<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(B, &Self::Elm) -> B,
    {
        let mut result = b;
        for item in self.elements.iter() {
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
    use crate::Stack;

    #[test]
    fn test_empty_cons() -> Result<(), StackError> {
        let stack = ArrayStack::empty().cons(1);
        assert_eq!(*stack.head()?, 1);
        assert_eq!(stack.size(), 1);
        Ok(())
    }

    #[test]
    fn test_peek() -> Result<(), StackError> {
        let stack = ArrayStack::empty().cons(1).cons(2);
        assert_eq!(*stack.peek()?, 2);
        Ok(())
    }

    #[test]
    fn test_is_empty() {
        let stack1 = ArrayStack::<i32>::empty();
        let stack2 = stack1.clone().cons(1);
        assert!(rust_fp_categories::Empty::is_empty(&stack1));
        assert!(!rust_fp_categories::Empty::is_empty(&stack2));
    }

    #[test]
    fn test_tail() -> Result<(), StackError> {
        let stack = ArrayStack::empty().cons(1).cons(2);
        let tail = stack.tail();
        assert_eq!(*tail.head()?, 1);
        assert_eq!(tail.size(), 1);
        Ok(())
    }

    #[test]
    fn test_update() -> Result<(), StackError> {
        let stack = ArrayStack::empty().cons(1).cons(2).cons(3);
        let updated = stack.update(1, 5)?;
        assert_eq!(*updated.head()?, 3);
        assert_eq!(*updated.get(1)?, 5);
        assert_eq!(*updated.get(2)?, 1);
        Ok(())
    }

    #[test]
    fn test_get() -> Result<(), StackError> {
        let stack = ArrayStack::empty().cons(1).cons(2).cons(3);
        assert_eq!(*stack.get(0)?, 3);
        assert_eq!(*stack.get(1)?, 2);
        assert_eq!(*stack.get(2)?, 1);
        Ok(())
    }

    #[test]
    fn test_from_iter() {
        let stack = ArrayStack::from_iter(vec![1, 2, 3]);
        assert_eq!(stack.size(), 3);
        assert_eq!(stack.elements, vec![1, 2, 3]);
    }

    #[test]
    fn test_from_vec_to_vec() {
        let vec = vec![1, 2, 3];
        let stack = ArrayStack::from(vec.clone());
        let vec2: Vec<i32> = stack.into();
        assert_eq!(vec, vec2);
    }
}
