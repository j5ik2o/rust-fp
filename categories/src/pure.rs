use std::rc::Rc;

pub trait Pure {
    type Elm;
    type M<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm>;

    fn unit() -> Self::M<()>;
}

macro_rules! pure_numeric_impl {
    ($($t:ty)*) => ($(
        impl Pure for $t {
          type Elm = $t;
          type M<U> = U;

          fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
            value
          }

          fn unit() -> Self::M<()> {
            ()
          }
        }
    )*)
}

pure_numeric_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl<A> Pure for Rc<A> {
    type Elm = A;
    type M<U> = Rc<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        Rc::new(value)
    }

    fn unit() -> Self::M<()> {
        Rc::new(())
    }
}

impl<A> Pure for Box<A> {
    type Elm = A;
    type M<U> = Box<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        Box::new(value)
    }

    fn unit() -> Self::M<()> {
        Box::new(())
    }
}

impl<A> Pure for Option<A> {
    type Elm = A;
    type M<U> = Option<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        Some(value)
    }

    fn unit() -> Self::M<()> {
        Some(())
    }
}

impl<A, E> Pure for Result<A, E> {
    type Elm = A;
    type M<U> = Result<U, E>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        Ok(value)
    }

    fn unit() -> Self::M<()> {
        Ok(())
    }
}

impl<A> Pure for Vec<A> {
    type Elm = A;
    type M<U> = Vec<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        vec![value]
    }

    fn unit() -> Self::M<()> {
        vec![()]
    }
}
