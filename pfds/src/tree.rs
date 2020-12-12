use std::rc::Rc;

use crate::Set;
use rust_fp_categories::Empty;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Tree<A> {
    Empty,
    Cons(Rc<Self>, A, Rc<Self>),
}

impl<A> Tree<A> {
    pub fn single(value: A) -> Self {
        Self::cons(Tree::Empty, value, Tree::Empty)
    }

    pub fn cons(left: Self, value: A, right: Self) -> Self {
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
        fn insert_to<A: Clone + PartialEq + PartialOrd>(x: A, s: &Tree<A>) -> Option<Tree<A>> {
            match s {
                &Tree::Empty => Some(Tree::cons(Tree::Empty, x, Tree::Empty)),
                &Tree::Cons(ref a, ref y, ref b) => {
                    if x < *y {
                        insert_to(x, a)
                            .map(|a: Tree<A>| Tree::Cons(Rc::new(a), y.clone(), b.clone()))
                    } else if *y < x {
                        insert_to(x, b)
                            .map(|b: Tree<A>| Tree::Cons(a.clone(), y.clone(), Rc::new(b)))
                    } else {
                        None
                    }
                }
            }
        }
        insert_to(value, &self).unwrap_or(self)
    }

    fn member(&self, value: A) -> bool {
        fn member1<A: Clone + PartialEq + PartialOrd>(x: A, last: Option<A>, ss: &Tree<A>) -> bool {
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

    fn size(&self) -> usize {
        match self {
            &Tree::Empty => 0,
            &Tree::Cons(ref a, _, ref b) => 1 + a.size() + b.size(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Set, StackError, Tree};

    #[test]
    fn test_size() -> Result<(), StackError> {
        assert_eq!(Tree::single(1).size(), 1);
        Ok(())
    }
}
