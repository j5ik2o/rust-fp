use std::future::Future;
use std::pin::Pin;

/// AsyncFunctorは、非同期的なマッピング操作を可能にする型クラスです。
///
/// # 型クラス階層における位置
///
/// AsyncFunctorは非同期カテゴリの基本となる型クラスです：
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
/// * `M<B>` - 変換後のコンテナの型（Bは新しい要素の型）
///
/// # メソッド
///
/// * `async_fmap` - コンテナ内の各要素に関数を非同期的に適用し、新しいコンテナを返す
pub trait AsyncFunctor {
    type Elm: Clone;
    type M<B: Clone + Send + Sync + 'static>: AsyncFunctor<Elm = B>;

    fn fmap<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        f: F,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> B + Send + Sync + 'a;
}
