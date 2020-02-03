use std::sync::Arc;

use rust_fp_categories::apply::Apply;
use rust_fp_categories::empty::Empty;
use rust_fp_categories::foldable::Foldable;
use rust_fp_categories::functor::Functor;
use rust_fp_categories::hkt::HKT;
use rust_fp_categories::monoid::Monoid;
use rust_fp_categories::pure::Pure;
use rust_fp_categories::semigroup::Semigroup;
use rust_fp_categories::applicative::Applicative;
use stack::{Stack, StackError};

#[derive(Debug, Clone)]
pub enum List<A> {
    Nil,
    Cons {
        head: A,
        tail: Arc<List<A>>,
    },
}

impl<A: Clone> List<A> {
    fn reverse(&self) -> Self {
        self.fold_left(List::empty(), |acc, h| acc.cons(h.clone()))
    }
}


impl<T, U> HKT<U> for List<T> {
    type C = T;
    type T = List<U>;
}

impl<A> Empty for List<A> {
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
    fn combine(&self, other: Self) -> Self {
        match self {
            &List::Nil => other,
            &List::Cons { head: ref h, tail: ref t } =>
                List::Cons { head: h.clone(), tail: Arc::new(t.combine(other)) },
        }
    }
}

impl<A: Clone> Monoid for List<A> {}

// ---

impl<A, B: Clone> Functor<B> for List<A> {
    fn fmap<F>(&self, f: F) -> List<B>
        where
            F: Fn(&A) -> B,
            List<B>: Stack<B> {
        if self.is_empty() {
            List::Nil
        } else {
            let mut result: List<B> = List::empty();
            let mut cur: &List<A> = self;
            while let List::Cons { ref head, ref tail } = *cur {
                result = result.cons(f(head));
                cur = tail
            }
            result
        }
    }
}

impl<A: Clone> Pure<A> for List<A> {
    fn pure(value: A) -> Self::T {
        List::empty().cons(value)
    }
}

impl<A: Clone, B> Foldable<B> for List<A> {
    fn fold_left<F>(&self, b: B, f: F) -> B where
        F: Fn(B, &<Self as HKT<B>>::C) -> B {
        match self {
            &List::Nil => b,
            &List::Cons { ref head, ref tail } =>
                tail.fold_left(f(b, head), f),
        }
    }

    fn fold_right<F>(&self, b: B, f: F) -> B
        where F: Fn(&<Self as HKT<A>>::C, B) -> B, {
        self.reverse().fold_left(b, |b, a| f(a, b))
    }
}

impl<A: Clone> Stack<A> for List<A> {
    fn cons(&self, value: A) -> Self {
        List::Cons {
            head: value,
            tail: Arc::new(self.clone()),
        }
    }

    fn head(&self) -> Result<A, StackError> {
        match *self {
            List::Nil => Err(StackError::NoSuchElementError),
            List::Cons { head: ref value, .. } => Ok(value.clone()),
        }
    }

    fn tail(&self) -> Arc<Self> {
        match *self {
            List::Nil => Arc::new(List::Nil),
            List::Cons { ref tail, .. } => tail.clone(),
        }
    }

    fn size(&self) -> usize {
        match *self {
            List::Nil => 0,
            List::Cons { ref tail, .. } => 1 + tail.size(),
        }
    }

    fn update(&self, index: u32, new_value: A) -> Result<Self, StackError> where Self: Sized {
        match self {
            &List::Nil => Err(StackError::IndexOutOfRange),
            &List::Cons { head: ref value, ref tail } => match index {
                0 => Ok(tail.clone().cons(new_value)),
                _ => {
                    let updated_tail = tail.update(index - 1, new_value)?;
                    Ok(updated_tail.cons(value.clone()))
                }
            },
        }
    }

    fn get(&self, index: u32) -> Result<A, StackError> {
        match self {
            &List::Nil => Err(StackError::NoSuchElementError),
            &List::Cons { head: ref value, ref tail } => match index {
                0 => Ok(value.clone()),
                _ => tail.get(index - 1)
            },
        }
    }
}

