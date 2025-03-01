use std::boxed::Box;

use crate::Set;
use rust_fp_categories::Empty;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum Tree<A>
where
    A: Eq,
{
    Empty,
    Cons(Box<Self>, A, Box<Self>),
}

impl<A: Eq> Tree<A> {
    pub fn single(value: A) -> Self {
        Self::cons(Tree::Empty, value, Tree::Empty)
    }

    pub fn cons(left: Self, value: A, right: Self) -> Self {
        Tree::Cons(Box::new(left), value, Box::new(right))
    }
}

impl<A: Eq> Empty for Tree<A> {
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

impl<A: Clone + PartialEq + PartialOrd + Eq> Set<A> for Tree<A> {
    fn insert(self, value: A) -> Self {
        fn insert_to<A: Clone + PartialEq + PartialOrd + Eq>(x: A, s: &Tree<A>) -> Option<Tree<A>> {
            match s {
                &Tree::Empty => Some(Tree::cons(Tree::Empty, x, Tree::Empty)),
                &Tree::Cons(ref a, ref y, ref b) => {
                    if x < *y {
                        insert_to(x, a)
                            .map(|a: Tree<A>| Tree::Cons(Box::new(a), y.clone(), b.clone()))
                    } else if *y < x {
                        insert_to(x, b)
                            .map(|b: Tree<A>| Tree::Cons(a.clone(), y.clone(), Box::new(b)))
                    } else {
                        None
                    }
                }
            }
        }
        insert_to(value, &self).unwrap_or(self)
    }

    fn member(&self, value: A) -> bool {
        fn member1<A: Clone + PartialEq + PartialOrd + Eq>(
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

    fn size(&self) -> usize {
        match self {
            &Tree::Empty => 0,
            &Tree::Cons(ref a, _, ref b) => 1 + a.size() + b.size(),
        }
    }

    fn union(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        fn fold_insert<A: Clone + PartialEq + PartialOrd + Eq>(acc: Tree<A>, value: A) -> Tree<A> {
            acc.insert(value)
        }

        fn fold_tree<A: Clone + PartialEq + PartialOrd + Eq>(
            acc: Tree<A>,
            tree: &Tree<A>,
        ) -> Tree<A> {
            match tree {
                Tree::Empty => acc,
                Tree::Cons(left, value, right) => {
                    let acc = fold_tree(acc, left);
                    let acc = fold_insert(acc, value.clone());
                    fold_tree(acc, right)
                }
            }
        }

        fold_tree(self, &other)
    }

    fn intersection(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        fn fold_intersect<A: Clone + PartialEq + PartialOrd + Eq>(
            acc: Tree<A>,
            value: A,
            other: &Tree<A>,
        ) -> Tree<A> {
            if other.member(value.clone()) {
                acc.insert(value)
            } else {
                acc
            }
        }

        fn fold_tree<A: Clone + PartialEq + PartialOrd + Eq>(
            acc: Tree<A>,
            tree: &Tree<A>,
            other: &Tree<A>,
        ) -> Tree<A> {
            match tree {
                Tree::Empty => acc,
                Tree::Cons(left, value, right) => {
                    let acc = fold_tree(acc, left, other);
                    let acc = fold_intersect(acc, value.clone(), other);
                    fold_tree(acc, right, other)
                }
            }
        }

        fold_tree(Tree::empty(), &self, &other)
    }

    fn difference(self, other: Self) -> Self
    where
        Self: Sized,
        A: Clone,
    {
        fn fold_difference<A: Clone + PartialEq + PartialOrd + Eq>(
            acc: Tree<A>,
            value: A,
            other: &Tree<A>,
        ) -> Tree<A> {
            if !other.member(value.clone()) {
                acc.insert(value)
            } else {
                acc
            }
        }

        fn fold_tree<A: Clone + PartialEq + PartialOrd + Eq>(
            acc: Tree<A>,
            tree: &Tree<A>,
            other: &Tree<A>,
        ) -> Tree<A> {
            match tree {
                Tree::Empty => acc,
                Tree::Cons(left, value, right) => {
                    let acc = fold_tree(acc, left, other);
                    let acc = fold_difference(acc, value.clone(), other);
                    fold_tree(acc, right, other)
                }
            }
        }

        fold_tree(Tree::empty(), &self, &other)
    }

    fn is_subset_of(&self, other: &Self) -> bool
    where
        A: Clone,
    {
        fn check_subset<A: Clone + PartialEq + PartialOrd + Eq>(
            tree: &Tree<A>,
            other: &Tree<A>,
        ) -> bool {
            match tree {
                Tree::Empty => true,
                Tree::Cons(left, value, right) => {
                    other.member(value.clone())
                        && check_subset(left, other)
                        && check_subset(right, other)
                }
            }
        }

        check_subset(self, other)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Set, StackError, Tree};
    use rust_fp_categories::Empty;

    #[test]
    fn test_size() -> Result<(), StackError> {
        assert_eq!(Tree::single(1).size(), 1);
        Ok(())
    }

    #[test]
    fn test_empty_insert() {
        let set = Tree::<i32>::empty();
        assert_eq!(set.size(), 0);

        let set = set.insert(1);
        assert_eq!(set.size(), 1);
        assert!(set.member(1));
    }

    #[test]
    fn test_union() {
        let set1 = Tree::<i32>::empty().insert(1).insert(2);
        let set2 = Tree::<i32>::empty().insert(2).insert(3);

        let union = set1.union(set2);
        assert_eq!(union.size(), 3);
        assert!(union.member(1));
        assert!(union.member(2));
        assert!(union.member(3));
    }

    #[test]
    fn test_intersection() {
        let set1 = Tree::<i32>::empty().insert(1).insert(2);
        let set2 = Tree::<i32>::empty().insert(2).insert(3);

        let intersection = set1.intersection(set2);
        assert_eq!(intersection.size(), 1);
        assert!(!intersection.member(1));
        assert!(intersection.member(2));
        assert!(!intersection.member(3));
    }

    #[test]
    fn test_difference() {
        let set1 = Tree::<i32>::empty().insert(1).insert(2);
        let set2 = Tree::<i32>::empty().insert(2).insert(3);

        let difference = set1.difference(set2);
        assert_eq!(difference.size(), 1);
        assert!(difference.member(1));
        assert!(!difference.member(2));
        assert!(!difference.member(3));
    }

    #[test]
    fn test_is_subset_of() {
        let set1 = Tree::<i32>::empty().insert(1).insert(2);
        let set2 = Tree::<i32>::empty().insert(1).insert(2).insert(3);
        let set3 = Tree::<i32>::empty().insert(1).insert(4);

        assert!(set1.is_subset_of(&set2));
        assert!(!set2.is_subset_of(&set1));
        assert!(!set1.is_subset_of(&set3));
    }
}
