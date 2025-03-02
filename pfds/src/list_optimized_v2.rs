use crate::stack::Stack;
use crate::StackError;
use rust_fp_categories::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum List<A> {
    Nil,
    Cons { head: A, tail: Rc<List<A>> },
}

impl<A: Clone> From<Vec<A>> for List<A> {
    fn from(vec: Vec<A>) -> Self {
        let mut result = List::empty();
        for item in vec.iter().rev() {
            result = result.cons(item.clone());
        }
        result
    }
}

impl<A: Clone> Into<Vec<A>> for List<A> {
    fn into(self) -> Vec<A> {
        let size = self.size();
        let mut result = Vec::with_capacity(size);
        let mut current = &self;
        while let List::Cons { ref head, ref tail } = *current {
            result.push(head.clone());
            current = tail;
        }
        result
    }
}

impl<A: Clone> List<A> {
    // Optimized drop implementation using iteration instead of recursion
    pub fn drop(self, n: u32) -> Self {
        if n == 0 {
            return self;
        }

        let mut current = self;
        let mut count = 0;

        while count < n {
            match current {
                List::Nil => return List::Nil,
                List::Cons { tail, .. } => {
                    // We can't use Rc::try_unwrap here because it would consume the Rc
                    // Instead, we need to clone the tail and continue
                    current = (*tail).clone();
                    count += 1;
                }
            }
        }

        current
    }

    pub fn reverse(&self) -> Self {
        let mut result = List::empty();
        let mut current = self;
        while let List::Cons { ref head, ref tail } = *current {
            result = result.cons(head.clone());
            current = tail;
        }
        result
    }
}

// --- Monoid

impl<A: Clone> Empty for List<A> {
    fn empty() -> List<A> {
        List::Nil
    }
    fn is_empty(&self) -> bool {
        match self {
            &List::Nil => true,
            _ => false,
        }
    }
}

impl<A: Clone> Semigroup for List<A> {
    fn combine(self, other: Self) -> Self {
        match self {
            List::Nil => other,
            List::Cons { head: h, tail: t } => List::Cons {
                head: h,
                tail: Rc::new(Rc::try_unwrap(t).unwrap_or(List::Nil).combine(other)),
            },
        }
    }
}

impl<A: Clone> Monoid for List<A> {}

// --- Functor

impl<A: Clone> Functor for List<A> {
    type Elm = A;
    type M<U: Clone> = List<U>;

    fn fmap<B: Clone, F>(self, f: F) -> List<B>
    where
        F: Fn(&A) -> B,
        List<B>: Stack<B>,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            List::Nil
        } else {
            self.fold_right(List::<B>::empty(), |v, acc| acc.cons(f(&v)))
        }
    }
}

// --- Applicative

impl<A: Clone> Pure for List<A> {
    type Elm = A;
    type M<U: Clone> = List<U>;

    fn pure(value: A) -> List<A> {
        List::empty().cons(value)
    }

    fn unit() -> Self::M<()> {
        List::empty().cons(())
    }
}

impl<A: Clone> Apply for List<A> {
    type Elm = A;
    type M<U: Clone> = List<U>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&A) -> B,
        List<B>: Stack<B>,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            List::Nil
        } else {
            let mut result: List<B> = List::empty();
            let mut cur1: &List<A> = &self;
            let mut cur2: &List<F> = &fs;
            while let List::Cons { ref head, ref tail } = *cur1 {
                if let List::Cons {
                    head: ref hf,
                    tail: ref tf,
                } = *cur2
                {
                    result = result.cons((*hf)(head));
                    cur1 = tail;
                    cur2 = tf;
                }
            }
            result
        }
    }
}

impl<A: Clone> Applicative for List<A> {}

// --- Bind

impl<A: Clone> Bind for List<A> {
    type Elm = A;
    type M<U: Clone> = List<U>;

    fn bind<B: Clone, F>(self, f: F) -> List<B>
    where
        F: Fn(&A) -> List<B>,
    {
        if rust_fp_categories::Empty::is_empty(&self) {
            List::Nil
        } else {
            self.fold_left(List::<B>::empty(), |acc, v| acc.combine(f(&v)))
        }
    }
}

impl<A: Clone> Monad for List<A> {}

// --- Foldable

impl<A: Clone> Foldable for List<A> {
    type Elm = A;

    fn fold_left<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(B, &Self::Elm) -> B,
    {
        match self {
            &List::Nil => b,
            &List::Cons { ref head, ref tail } => tail.fold_left(f(b, head), f),
        }
    }

    fn fold_right<B, F>(&self, b: B, f: F) -> B
    where
        F: Fn(&Self::Elm, B) -> B,
    {
        self.reverse().fold_left(b, |b, a| f(a, b))
    }
}

impl<A: Clone> Stack<A> for List<A> {
    fn cons(self, value: A) -> Self {
        List::Cons {
            head: value,
            tail: Rc::new(self),
        }
    }

    fn head(&self) -> Result<&A, StackError> {
        match self {
            &List::Nil => Err(StackError::NoSuchElementError),
            &List::Cons {
                head: ref value, ..
            } => Ok(value),
        }
    }

    fn peek(&self) -> Result<&A, StackError> {
        self.head()
    }

    fn tail(&self) -> Rc<Self> {
        match self {
            List::Nil => Rc::new(List::Nil),
            List::Cons { tail, .. } => Rc::clone(tail),
        }
    }

    fn size(&self) -> usize {
        match self {
            &List::Nil => 0,
            &List::Cons { ref tail, .. } => 1 + tail.size(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            &List::Nil => true,
            _ => false,
        }
    }

    // Optimized update implementation to reduce unnecessary clones
    fn update(self, index: u32, new_value: A) -> Result<Self, StackError>
    where
        Self: Sized,
    {
        if index == 0 {
            match self {
                List::Nil => Err(StackError::IndexOutOfRangeError),
                List::Cons { tail, .. } => Ok(List::Cons {
                    head: new_value,
                    tail: tail,
                }),
            }
        } else {
            match self {
                List::Nil => Err(StackError::IndexOutOfRangeError),
                List::Cons { head, tail } => {
                    match Rc::try_unwrap(tail) {
                        Ok(t) => {
                            // Successfully unwrapped, update the tail
                            let updated_tail = t.update(index - 1, new_value)?;
                            Ok(List::Cons {
                                head,
                                tail: Rc::new(updated_tail),
                            })
                        }
                        Err(tail_rc) => {
                            // Could not unwrap, clone and update
                            let t = (*tail_rc).clone();
                            let updated_tail = t.update(index - 1, new_value)?;
                            Ok(List::Cons {
                                head,
                                tail: Rc::new(updated_tail),
                            })
                        }
                    }
                }
            }
        }
    }

    // Optimized get implementation using iteration instead of recursion
    fn get(&self, index: u32) -> Result<&A, StackError> {
        let mut current = self;
        let mut current_index = index;

        while current_index > 0 {
            match current {
                List::Nil => return Err(StackError::NoSuchElementError),
                List::Cons { tail, .. } => {
                    current = tail;
                    current_index -= 1;
                }
            }
        }

        match current {
            List::Nil => Err(StackError::NoSuchElementError),
            List::Cons { head, .. } => Ok(head),
        }
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut result = List::empty();
        for item in iter.into_iter().collect::<Vec<_>>().into_iter().rev() {
            result = result.cons(item);
        }
        result
    }

    fn uncons(self) -> Result<(A, Self), StackError> {
        match self {
            List::Nil => Err(StackError::NoSuchElementError),
            List::Cons { head, tail } => Ok((head, (*tail).clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list_optimized_v2::List;
    use crate::stack::StackError;
    use crate::Stack;
    use rust_fp_categories::Bind;
    use rust_fp_categories::Empty;
    use rust_fp_categories::Functor;
    use rust_fp_categories::Semigroup;

    #[test]
    fn test_from_vec_to_vec() -> Result<(), StackError> {
        let v1 = vec![1, 2, 3];
        let expected1 = v1.clone();
        let l1 = List::from(v1);
        let v2: Vec<i32> = l1.into();
        assert_eq!(v2, expected1);
        Ok(())
    }

    #[test]
    fn test_empty_cons() -> Result<(), StackError> {
        let list1 = List::empty().cons(1);
        assert_eq!(*list1.head()?, 1);
        assert_eq!(*list1.tail(), List::empty());
        Ok(())
    }

    #[test]
    fn test_is_empty() -> Result<(), StackError> {
        let list1 = List::empty().cons(1);
        assert_eq!(rust_fp_categories::Empty::is_empty(&list1), false);
        assert_eq!(
            rust_fp_categories::Empty::is_empty(&List::<i32>::empty()),
            true
        );
        Ok(())
    }

    #[test]
    fn test_combine() -> Result<(), StackError> {
        let list1 = List::empty().cons(1);
        let list2 = List::empty().cons(1);
        let list3 = list1.combine(list2);
        let vec1: Vec<i32> = list3.into();
        assert_eq!(vec1, vec![1, 1]);
        Ok(())
    }

    #[test]
    fn test_fmap() -> Result<(), StackError> {
        let list1: List<i32> = List::from(vec![1, 2, 3, 4, 5]);
        let list2: List<i32> = list1.fmap(|v| v * 2);
        let vec1: Vec<i32> = list2.into();
        assert_eq!(vec1, vec![2, 4, 6, 8, 10]);
        Ok(())
    }

    #[test]
    fn test_bind() -> Result<(), StackError> {
        let list1: List<i32> = List::from(vec![1, 2, 3, 4, 5]);
        let list2 = list1.clone();
        let list3 = list1.bind(|_| List::<i32>::empty());
        let vec1: Vec<i32> = list3.into();
        assert_eq!(vec1, Vec::<i32>::empty());
        let list4 = list2.bind(|v| List::<i32>::empty().cons(*v * 2));
        let vec2: Vec<i32> = list4.into();
        assert_eq!(vec2, vec![2, 4, 6, 8, 10]);
        Ok(())
    }

    #[test]
    fn test_head_tail() -> Result<(), StackError> {
        let list1: List<i32> = List::from(vec![1, 2, 3, 4, 5]);
        let head = list1.head()?;
        let tail = list1.tail();
        assert_eq!(*head, 1);
        assert_eq!(*tail.as_ref(), List::from(vec![2, 3, 4, 5]));
        let vec1: Vec<i32> = tail.as_ref().clone().into();
        assert_eq!(vec1, vec![2, 3, 4, 5]);
        Ok(())
    }

    #[test]
    fn test_get() -> Result<(), StackError> {
        let list1: List<i32> = List::empty().cons(5).cons(4).cons(3).cons(2).cons(1);
        let chr = list1.get((list1.size() - 1) as u32)?;
        assert_eq!(*chr, 5);
        Ok(())
    }

    #[test]
    fn test_eq() -> Result<(), StackError> {
        let list1: List<i32> = List::from(vec![1, 2, 3, 4, 5]);
        let list2: List<i32> = List::from(vec![2, 2, 3, 4, 5]);
        assert_ne!(list1, list2);
        assert_ne!(*list1.head()?, *list2.head()?);
        assert_eq!(list1.tail(), list2.tail());
        Ok(())
    }

    #[test]
    fn test_drop() -> Result<(), StackError> {
        let list1: List<i32> = List::from(vec![1, 2, 3, 4, 5]);
        let list2 = list1.clone().drop(2);
        let vec1: Vec<i32> = list2.into();
        assert_eq!(vec1, vec![3, 4, 5]);

        // Test dropping more elements than the list contains
        let list3 = list1.clone().drop(10);
        assert!(rust_fp_categories::Empty::is_empty(&list3));

        // Test dropping zero elements
        let list4 = list1.clone().drop(0);
        let vec2: Vec<i32> = list4.into();
        assert_eq!(vec2, vec![1, 2, 3, 4, 5]);

        Ok(())
    }

    #[test]
    fn test_update() -> Result<(), StackError> {
        let list1: List<i32> = List::from(vec![1, 2, 3, 4, 5]);

        // Update first element
        let list2 = list1.clone().update(0, 10)?;
        let vec1: Vec<i32> = list2.into();
        assert_eq!(vec1, vec![10, 2, 3, 4, 5]);

        // Update middle element
        let list3 = list1.clone().update(2, 30)?;
        let vec2: Vec<i32> = list3.into();
        assert_eq!(vec2, vec![1, 2, 30, 4, 5]);

        // Update last element
        let list4 = list1.clone().update(4, 50)?;
        let vec3: Vec<i32> = list4.into();
        assert_eq!(vec3, vec![1, 2, 3, 4, 50]);

        // Test updating out of bounds
        let result = list1.clone().update(10, 100);
        assert!(result.is_err());

        Ok(())
    }
}
