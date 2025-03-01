use std::rc::Rc;

/// Bindは、モナド的な連鎖操作を可能にする型クラスです。
///
/// # 型クラス階層における位置
///
/// BindはFunctorを拡張した型クラスで、Monadの一部となります：
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
/// * `M<B>` - 変換後のコンテナの型（Bは新しい要素の型）
///
/// # メソッド
///
/// * `bind` - コンテナ内の値に関数を適用し、その結果を平坦化して新しいコンテナを返す
pub trait Bind {
    type Elm;
    type M<B>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> Self::M<B>;
}

use crate::impl_bind_for_numeric;

impl_bind_for_numeric!();

impl<A> Bind for Rc<A> {
    type Elm = A;
    type M<U> = Rc<U>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> Self::M<B>,
    {
        crate::common::rc::bind(self, f)
    }
}

impl<A> Bind for Box<A> {
    type Elm = A;
    type M<U> = Box<U>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> Self::M<B>,
    {
        crate::common::boxed::bind(self, f)
    }
}

// ---

impl<A> Bind for Option<A> {
    type Elm = A;
    type M<U> = Option<U>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> Self::M<B>,
    {
        crate::common::option::bind(self, f)
    }
}

impl<A, E> Bind for Result<A, E> {
    type Elm = A;
    type M<U> = Result<U, E>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnOnce(&Self::Elm) -> Self::M<B>,
    {
        crate::common::result::bind(self, f)
    }
}

impl<A> Bind for Vec<A> {
    type Elm = A;
    type M<U> = Vec<U>;

    fn bind<B, F>(self, f: F) -> Self::M<B>
    where
        F: FnMut(&Self::Elm) -> Self::M<B>,
    {
        crate::common::vec::bind(self, f)
    }
}
