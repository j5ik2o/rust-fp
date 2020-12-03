use std::rc::Rc;

use rust_fp_categories::*;
use rust_fp_categories::empty::Empty;
use rust_fp_categories::hkt::HKT;
use set::Set;

#[derive(Debug, PartialEq, PartialOrd)]
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

impl<A: PartialEq + PartialOrd> Set<A> for Tree<A> {

    fn insert(self, value: A) -> Self {
        fn insert_to<A: PartialEq + PartialOrd>(
            x: A,
            s_arc: Rc<Tree<A>>,
        ) -> Option<Rc<Tree<A>>> {
            let s = Rc::try_unwrap(s_arc).unwrap_or(Tree::Empty);
            match s {
                Tree::Empty => Some(Rc::new(Tree::cons(Tree::Empty, x, Tree::Empty))),
                Tree::Cons(a, y, b) => {
                    if x < y {
                        insert_to(x, a).map(|a| Rc::new(Tree::Cons(a, y, b)))
                    } else if y < x {
                        insert_to(x, b).map(|b| Rc::new(Tree::Cons(a, y, b)))
                    } else {
                        None
                    }
                }
            }
        }
        let target = Rc::new(self);
        let default = Rc::clone(&target);
        let result = insert_to(value, target).unwrap_or(default);
        Rc::try_unwrap(result).unwrap_or(Tree::Empty)
    }

    fn member(self, value: A) -> bool {
        fn member1<A: PartialEq + PartialOrd>(
            x: A,
            last: Option<A>,
            ss_arc: Rc<Tree<A>>,
        ) -> bool {
            let ss = Rc::try_unwrap(ss_arc).unwrap_or(Tree::Empty);
            match ss {
                Tree::Empty => last.iter().any(|y| x == *y),
                Tree::Cons(a, y, b) => {
                    if x < y {
                        member1(x, last, a)
                    } else {
                        member1(x, Some(y), b)
                    }
                }
            }
        }
        member1(value, None, Rc::new(self))
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