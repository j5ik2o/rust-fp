use std::future::Future;
use std::pin::Pin;

/// AsyncEmptyは、非同期的な空の状態を表現する型クラスです。
///
/// # 型クラス階層における位置
///
/// AsyncEmptyは非同期カテゴリの基本となる型クラスです。
///
/// # メソッド
///
/// * `empty` - 空のインスタンスを非同期的に生成する
/// * `is_empty` - インスタンスが空かどうかを非同期的に判定する
pub trait AsyncEmpty {
    fn empty<'a>() -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        Self: Sized + 'a;

    fn is_empty<'a>(&'a self) -> Pin<Box<dyn Future<Output = bool> + 'a>>;
}
