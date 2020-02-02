use hkt::HKT;

pub trait Foldable<A>: HKT<A> + Sized {
    fn fold_left<F>(&self, b: A, ba: F) -> A
    where
        F: Fn(A, &<Self as HKT<A>>::C) -> A;

    fn fold_right<F>(&self, b: A, f: F) -> A
    where
        F: Fn(&<Self as HKT<A>>::C, A) -> A;
}

impl<A, B> Foldable<B> for Vec<A> {
    fn fold_left<F>(&self, b: B, f: F) -> B
    where
        F: Fn(B, &<Self as HKT<B>>::C) -> B,
    {
        self.iter().fold(b, f)
    }

    fn fold_right<F>(&self, b: B, f: F) -> B
    where
        F: Fn(&<Self as HKT<B>>::C, B) -> B,
    {
        self.iter().rev().fold(b, |x, y| f(y, x))
    }
}
