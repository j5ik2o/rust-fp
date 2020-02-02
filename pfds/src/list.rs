use std::sync::Arc;
use rust_fp_categories::empty::Empty;
use rust_fp_categories::semigroup::Semigroup;
use rust_fp_categories::monoid::Monoid;

#[derive(Debug, Clone)]
pub enum List<A> {
    Nil,
    Cons {
        head: A,
        tail: Arc<List<A>>,
    },
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