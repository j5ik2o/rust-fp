use std::future::Future;
use std::pin::Pin;

/// AsyncPureは、値を非同期的にコンテナにリフトする機能を提供する型クラスです。
///
/// # 型クラス階層における位置
///
/// AsyncPureはAsyncApplicativeの一部となる型クラスです：
/// ```text
///                   AsyncFunctor
///                        |
///                        v
///                    AsyncApply
///                      /    \
///                     v      v
///                AsyncPure  AsyncBind
///                    \      /
///                     v    v
///                AsyncApplicative
///                        |
///                        v
///                    AsyncMonad
/// ```
///
/// # 型パラメータ
///
/// * `Elm` - コンテナ内の要素の型
///
/// # メソッド
///
/// * `pure` - 値を非同期的にコンテナにリフトする
pub trait AsyncPure {
    type Elm: Clone;

    fn pure<'a>(value: Self::Elm) -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        Self: Sized + 'a;
}
