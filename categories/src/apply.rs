use std::rc::Rc;

use hkt::HKT;

pub trait Apply<A>: HKT<A> {
    fn ap<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<A>>::T
    where
        F: Fn(&<Self as HKT<A>>::C) -> A,
        Self: HKT<F>;
}

impl<A, B> Apply<A> for Rc<B> {
    fn ap<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<A>>::T
    where
        F: Fn(&<Self as HKT<A>>::C) -> A,
    {
        let v = fs(self);
        Rc::new(v)
    }
}

impl<A, B> Apply<A> for Box<B> {
    fn ap<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<A>>::T
    where
        F: Fn(&<Self as HKT<A>>::C) -> A,
    {
        let v = fs(self);
        Box::new(v)
    }
}

// ---

impl<A, B> Apply<A> for Option<B> {
    fn ap<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<A>>::T
    where
        F: Fn(&<Self as HKT<A>>::C) -> A,
    {
        match self {
            &Some(ref value) => match fs {
                Some(f) => Some(f(value)),
                None => None,
            },
            &None => None,
        }
    }
}

impl<A, B, E: Clone> Apply<A> for Result<B, E> {
    fn ap<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<A>>::T
    where
        F: Fn(&<Self as HKT<A>>::C) -> A,
    {
        match self {
            &Ok(ref x) => match fs {
                Ok(fs) => Ok(fs(x)),
                Err(ref e) => Err(e.clone()),
            },
            &Err(ref e) => Err(e.clone()),
        }
    }
}

impl<A, B> Apply<A> for Vec<B> {
    fn ap<F>(&self, fs: <Self as HKT<F>>::T) -> <Self as HKT<A>>::T
    where
        F: Fn(&<Self as HKT<A>>::C) -> A,
    {
        let zipped = self.iter().zip(fs.iter());
        zipped.map(|(x, f)| f(x)).collect::<Vec<A>>()
    }
}
