use std::sync::Arc;

use rust_fp_categories::*;
use rust_fp_categories::empty::Empty;
use rust_fp_categories::hkt::HKT;
use set::Set;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Tree<A> {
    Empty,
    Cons(Arc<Self>, A, Arc<Self>),
}

derive_hkt!(Tree);

impl<A> Tree<A> {
    fn cons(left: Self, value: A, right: Self) -> Self {
        Tree::Cons(Arc::new(left), value, Arc::new(right))
    }
}

impl<A> Empty for Tree<A> {
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

impl<A: Clone + PartialEq + PartialOrd> Set<A> for Tree<A> {
    fn insert(&self, value: A) -> Self {
        fn insert_to<A: Clone + PartialEq + PartialOrd>(x: &A, s_arc: Arc<Tree<A>>) -> Option<Arc<Tree<A>>> {
            let s = Arc::try_unwrap(s_arc).unwrap_or(Tree::Empty);
            match s {
                Tree::Empty =>
                    Some(Arc::new(Tree::cons(Tree::Empty, x.clone(), Tree::Empty))),
                Tree::Cons(a, y, b) =>
                    if *x < y {
                        insert_to(x, a).map(|a| Arc::new(Tree::Cons(a, y, b)))
                    } else if y < *x {
                        insert_to(x, b).map(|b| Arc::new(Tree::Cons(a, y, b)))
                    } else {
                        None
                    }
            }
        }
        let result = insert_to(&value, Arc::new(self.clone())).unwrap_or(Arc::new(self.clone()));
        Arc::try_unwrap(result).unwrap_or(Tree::Empty)
    }

    fn member(&self, value: A) -> bool {
        fn member1<A: Clone + PartialEq + PartialOrd>(x: &A, last: Option<A>, ss_arc: Arc<Tree<A>>) -> bool {
            let ss = Arc::try_unwrap(ss_arc).unwrap_or(Tree::Empty);
            match ss {
                Tree::Empty =>
                    last.iter().any(|y| *x == *y),
                Tree::Cons(a, y, b) =>
                    if *x < y {
                        member1(x, last, a)
                    } else {
                        member1(x, Some(y), b)
                    }
            }
        }
        member1(&value, None, Arc::new(self.clone()))
    }
}
