use std::rc::Rc;

pub trait Pure {
    type Elm;
    type M<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm>;
}

impl<A> Pure for Rc<A> {
    type Elm = A;
    type M<U> = Rc<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        Rc::new(value)
    }
}

impl<A> Pure for Box<A> {
    type Elm = A;
    type M<U> = Box<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        Box::new(value)
    }
}

// ---

impl<A> Pure for Option<A> {
    type Elm = A;
    type M<U> = Option<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        Some(value)
    }
}

impl<A, E> Pure for Result<A, E> {
    type Elm = A;
    type M<U> = Result<U, E>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        Ok(value)
    }
}

impl<A> Pure for Vec<A> {
    type Elm = A;
    type M<U> = Vec<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        vec![value]
    }
}
