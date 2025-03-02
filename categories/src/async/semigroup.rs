use std::future::Future;
use std::pin::Pin;

/// AsyncSemigroupは、非同期的な結合操作を提供する型クラスです。
///
/// # 型クラス階層における位置
///
/// AsyncSemigroupは非同期カテゴリの基本となる型クラスです。
///
/// # メソッド
///
/// * `combine` - 2つの値を非同期的に結合する
pub trait AsyncSemigroup {
    fn combine<'a>(&'a self, other: &'a Self) -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        Self: Sized + Clone;
}
