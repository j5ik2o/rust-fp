use std::rc::Rc;

/// Pureは、値をコンテナにリフトするための型クラスです。
///
/// # 型クラス階層における位置
///
/// PureはApplicativeの一部となる型クラスです：
/// ```rust,ignore
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
    type M<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm>;

    fn unit() -> Self::M<()>;
}

use crate::impl_pure_for_numeric;

impl_pure_for_numeric!();

impl<A> Pure for Rc<A> {
    type Elm = A;
    type M<U> = Rc<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        crate::common::rc::pure(value)
    }

    fn unit() -> Self::M<()> {
        crate::common::rc::unit()
    }
}

impl<A> Pure for Box<A> {
    type Elm = A;
    type M<U> = Box<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        crate::common::boxed::pure(value)
    }

    fn unit() -> Self::M<()> {
        crate::common::boxed::unit()
    }
}

impl<A> Pure for Option<A> {
    type Elm = A;
    type M<U> = Option<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        crate::common::option::pure(value)
    }

    fn unit() -> Self::M<()> {
        crate::common::option::unit()
    }
}

impl<A, E> Pure for Result<A, E> {
    type Elm = A;
    type M<U> = Result<U, E>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        crate::common::result::pure(value)
    }

    fn unit() -> Self::M<()> {
        crate::common::result::unit()
    }
}

impl<A> Pure for Vec<A> {
    type Elm = A;
    type M<U> = Vec<U>;

    fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
        crate::common::vec::pure(value)
    }

    fn unit() -> Self::M<()> {
        crate::common::vec::unit()
    }
}
