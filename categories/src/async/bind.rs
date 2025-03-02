use std::future::Future;
use std::pin::Pin;

/// AsyncBindは、モナド的な連鎖操作を非同期的に可能にする型クラスです。
///
/// # 型クラス階層における位置
///
/// AsyncBindはAsyncFunctorを拡張した型クラスで、AsyncMonadの一部となります：
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
/// * `async_bind` - コンテナ内の値に関数を非同期的に適用し、その結果を平坦化して新しいコンテナを返す
pub trait AsyncBind {
    type Elm: Clone;
    type M<B: Clone + Send + Sync + 'static>: AsyncBind<Elm = B>;

    fn bind<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        f: F,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>> + Send + Sync + 'a;
}
