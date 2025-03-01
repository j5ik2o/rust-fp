use std::rc::Rc;

/// Pureは、値をコンテナにリフトするための型クラスです。
///
/// # 型クラス階層における位置
///
/// PureはApplicativeの一部となる型クラスです：
/// ```text
///                   Functor
///                     |
///                     v
///                    Apply
///                   /    \
///                  v      v
///                Pure    Bind
///                 \      /
///                  v    v
///               Applicative
///                     |
///                     v
///                   Monad
/// ```
///
/// # 型パラメータ
///
/// * `Elm` - コンテナ内の要素の型
/// * `M<U>` - 変換後のコンテナの型（Uは新しい要素の型）
///
/// # メソッド
///
/// * `pure` - 値をコンテナにリフトする
/// * `unit` - 単位値をコンテナにリフトする
pub trait Pure {
    type Elm;
    type M<U: Clone>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> where Self::Elm: Clone;

    fn unit() -> Self::M<()>;
}

use crate::impl_pure_for_numeric;

impl_pure_for_numeric!();

impl<A: Clone> Pure for Rc<A> {
    type Elm = A;
    type M<U: Clone> = Rc<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        crate::common::rc::pure(value)
    }

    fn unit() -> Self::M<()> {
        crate::common::rc::unit()
    }
}

impl<A: Clone> Pure for Box<A> {
    type Elm = A;
    type M<U: Clone> = Box<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        crate::common::boxed::pure(value)
    }

    fn unit() -> Self::M<()> {
        crate::common::boxed::unit()
    }
}

impl<A: Clone> Pure for Option<A> {
    type Elm = A;
    type M<U: Clone> = Option<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        crate::common::option::pure(value)
    }

    fn unit() -> Self::M<()> {
        crate::common::option::unit()
    }
}

impl<A: Clone, E> Pure for Result<A, E> {
    type Elm = A;
    type M<U: Clone> = Result<U, E>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        crate::common::result::pure(value)
    }

    fn unit() -> Self::M<()> {
        crate::common::result::unit()
    }
}

impl<A: Clone> Pure for Vec<A> {
    type Elm = A;
    type M<U: Clone> = Vec<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        crate::common::vec::pure(value)
    }

    fn unit() -> Self::M<()> {
        crate::common::vec::unit()
    }
}
