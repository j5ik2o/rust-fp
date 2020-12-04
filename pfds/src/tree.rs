use std::rc::Rc;

use rust_fp_categories::*;
use rust_fp_categories::empty::Empty;
use rust_fp_categories::hkt::HKT;
use set::Set;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Tree<A> {
    Empty,
    Cons(Rc<Self>, A, Rc<Self>),
}

derive_hkt!(Tree);

impl<A> Tree<A> {

    fn cons(left: Self, value: A, right: Self) -> Self {
        Tree::Cons(Rc::new(left), value, Rc::new(right))
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

    fn insert(self, value: A) -> Self {
        fn insert_to<A: Clone + PartialEq + PartialOrd>(
            x: A,
            s: &Tree<A>,
        ) -> Option<Tree<A>> {
            match s {
                &Tree::Empty => Some(Tree::cons(Tree::Empty, x, Tree::Empty)),
                &Tree::Cons(ref a, ref y, ref b) => {
                    if x < *y {
                        insert_to(x, a).map(|a| Tree::Cons(Rc::new(a), y.clone(), b.clone()))
                    } else if *y < x {
                        insert_to(x, b).map(|b| Tree::Cons(a.clone(), y.clone(), Rc::new(b)))
                    } else {
                        None
                    }
                }
            }
        }
        insert_to(value, &self).unwrap_or(self)
    }

    fn member(&self, value: A) -> bool {
        fn member1<A: Clone + PartialEq + PartialOrd>(
            x: A,
            last: Option<A>,
            ss: &Tree<A>,
        ) -> bool {
            match ss {
                &Tree::Empty => last.iter().any(|y| x == *y),
                &Tree::Cons(ref a, ref y, ref b) => {
                    if x < *y {
                        member1(x, last, a.as_ref())
                    } else {
                        member1(x, Some(y.clone()), b.as_ref())
                    }
                }
            }
        }
        member1(value, None, self)
    }
}

#[cfg(test)]
mod tests {
    use stack::StackError;

    #[test]
    fn test() -> Result<(), StackError> {
        Ok(())
    }
}