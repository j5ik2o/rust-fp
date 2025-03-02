use std::future::Future;
use std::pin::Pin;

use super::functor::AsyncFunctor;

/// AsyncApplyは、関数を含むコンテナを値を含むコンテナに非同期的に適用する型クラスです。
///
/// # 型クラス階層における位置
///
/// AsyncApplyはAsyncFunctorを拡張した型クラスで、AsyncApplicativeの一部となります：
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
/// * `ap` - 関数を含むコンテナを値を含むコンテナに非同期的に適用する
pub trait AsyncApply: AsyncFunctor {
    fn ap<'a, B: Clone + Send + Sync + 'static, F: Clone + Send + Sync + 'static>(
        &'a self,
        fs: &'a Self::M<F>,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> B + Send + Sync + 'a;
}
