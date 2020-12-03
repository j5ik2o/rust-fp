use std::rc::Rc;

use hkt::HKT;

pub trait Apply<A>: HKT<A> {
    fn ap<F>(self, fs: <Self as HKT<F>>::T) -> <Self as HKT<A>>::T
    where
        F: Fn(&<Self as HKT<A>>::C) -> A,
        Self: HKT<F>;
}

// ---

impl<A, B> Apply<B> for Rc<A> {
    fn ap<F>(self, fs: <Self as HKT<F>>::T) -> <Self as HKT<B>>::T
    where
        F: Fn(&A) -> B,
    {
        let v = fs(&self);
        Rc::new(v)
    }
}

impl<A, B> Apply<B> for Box<A> {
    fn ap<F>(self, fs: <Self as HKT<F>>::T) -> <Self as HKT<B>>::T
    where
        F: Fn(&A) -> B,
    {
        let v = fs(&self);
        Box::new(v)
    }
}

// ---

impl<A, B> Apply<B> for Option<A> {
    fn ap<F>(self, fs: <Self as HKT<F>>::T) -> <Self as HKT<B>>::T
    where
        F: Fn(&A) -> B,
    {
        match self {
            Some(ref value) => match fs {
                Some(f) => Some(f(value)),
                None => None,
            },
            None => None,
        }
    }
}

impl<A, B, E: Clone> Apply<B> for Result<A, E> {
    fn ap<F>(self, fs: <Self as HKT<F>>::T) -> <Self as HKT<B>>::T
    where
        F: Fn(&A) -> B,
    {
        match self {
            Ok(x) => match fs {
                Ok(fs) => Ok(fs(&x)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}

impl<A, B> Apply<B> for Vec<A> {
    fn ap<F>(self, fs: <Self as HKT<F>>::T) -> <Self as HKT<B>>::T
    where
        F: Fn(&A) -> B,
    {
        let zipped = self.iter().zip(fs.iter());
        zipped.map(|(x, f)| f(x)).collect::<Vec<B>>()
    }
}
