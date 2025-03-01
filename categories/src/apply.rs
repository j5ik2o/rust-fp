use std::rc::Rc;

pub trait Apply {
    type Elm;
    type M<B>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B;
}

// ---

macro_rules! apply_numeric_impl {
    ($($t:ty)*) => ($(
        impl Apply for $t {
          type Elm = $t;
          type M<U> = U;

          fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
          where
            F: Fn(&Self::Elm) -> B,
            {
                fs(&self)
            }
        }
    )*)
}

apply_numeric_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl<A> Apply for Rc<A> {
    type Elm = A;
    type M<U> = Rc<U>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        Rc::new(fs(&self))
    }
}

impl<A> Apply for Box<A> {
    type Elm = A;
    type M<U> = Box<U>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        Box::new(fs(&self))
    }
}

// ---

impl<A> Apply for Option<A> {
    type Elm = A;
    type M<U> = Option<U>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        Some(fs?(&self?))
    }
}

impl<A, E> Apply for Result<A, E> {
    type Elm = A;
    type M<U> = Result<U, E>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        let x = self?;
        let fs = fs?;
        Ok(fs(&x))
    }
}

impl<A> Apply for Vec<A> {
    type Elm = A;
    type M<U> = Vec<U>;

    fn ap<B, F>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        let zipped = self.iter().zip(fs.iter());
        zipped.map(|(x, f)| f(x)).collect::<Vec<B>>()
    }
}
