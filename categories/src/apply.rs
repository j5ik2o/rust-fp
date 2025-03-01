use std::rc::Rc;

/// Applyは、関数を含むコンテナを適用するための型クラスです。
///
/// # 型クラス階層における位置
///
/// ApplyはFunctorを拡張した型クラスで、Applicativeの一部となります：
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
/// * `M<B>` - 変換後のコンテナの型（Bは新しい要素の型）
///
/// # メソッド
///
/// * `ap` - 関数を含むコンテナを値を含むコンテナに適用し、新しいコンテナを返す
pub trait Apply {
    type Elm;
    type M<B: Clone>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B;
}

// ---

use crate::impl_apply_for_numeric;

impl_apply_for_numeric!();

impl<A> Apply for Rc<A> {
    type Elm = A;
    type M<U: Clone> = Rc<U>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        crate::common::rc::ap(self, fs)
    }
}

impl<A> Apply for Box<A> {
    type Elm = A;
    type M<U: Clone> = Box<U>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        crate::common::boxed::ap(self, fs)
    }
}

// ---

impl<A> Apply for Option<A> {
    type Elm = A;
    type M<U: Clone> = Option<U>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        crate::common::option::ap(self, fs)
    }
}

impl<A, E> Apply for Result<A, E> {
    type Elm = A;
    type M<U: Clone> = Result<U, E>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        crate::common::result::ap(self, fs)
    }
}

impl<A> Apply for Vec<A> {
    type Elm = A;
    type M<U: Clone> = Vec<U>;

    fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        crate::common::vec::ap(self, fs)
    }
}
