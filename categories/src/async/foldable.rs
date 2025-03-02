use std::future::Future;
use std::pin::Pin;

/// AsyncFoldableは、コンテナの要素を非同期的に畳み込む機能を提供する型クラスです。
///
/// # メソッド
///
/// * `fold_left` - 左から右へ非同期的に畳み込む
/// * `fold_right` - 右から左へ非同期的に畳み込む
pub trait AsyncFoldable: Sized {
    type Elm: Clone;

    fn fold_left<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        b: B,
        f: F,
    ) -> Pin<Box<dyn Future<Output = B> + 'a>>
    where
        F: Fn(B, &Self::Elm) -> Pin<Box<dyn Future<Output = B> + 'a>> + Send + Sync + 'a;

    fn fold_right<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        b: B,
        f: F,
    ) -> Pin<Box<dyn Future<Output = B> + 'a>>
    where
        F: Fn(&Self::Elm, B) -> Pin<Box<dyn Future<Output = B> + 'a>> + Send + Sync + 'a;
}
