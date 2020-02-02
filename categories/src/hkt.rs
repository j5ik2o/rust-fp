use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;
use std::rc::Rc;

pub trait HKT<U> {
    type C;
    // Current type
    type T; // Type with C swapped with U
}

#[macro_export]
macro_rules! derive_hkt {
    ($t:ident) => {
        impl<A, B> HKT<B> for $t<A> {
            type C = A;
            type T = $t<B>;
        }
    };
}

derive_hkt!(Rc);
derive_hkt!(Box);

derive_hkt!(Vec);
derive_hkt!(VecDeque);
derive_hkt!(Option);

impl<A, B, E> HKT<B> for Result<A, E> {
    type C = A;
    type T = Result<B, E>;
}

pub trait HKT3<U1, U2> {
    type C1;
    type C2;
    type T;
}

#[macro_export]
macro_rules! derive_hkt3 {
    ($t:ident) => {
        impl<A1, A2, B1, B2> HKT3<B1, B2> for $t<A1, A2> {
            // The currently contained types
            type C1 = B1;
            type C2 = B2;
            // How the U's get filled in.
            type T = $t<A1, B2>;
        }
    };
}

derive_hkt3!(HashMap);
