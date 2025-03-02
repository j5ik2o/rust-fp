use std::fmt::Display;
use std::marker::PhantomData;

/// HList型クラスは、異なる型の要素を持つヘテロジニアスリスト（異種リスト）を表します。
///
/// HListは、コンパイル時に型安全な方法で異なる型の要素を格納するためのデータ構造です。
/// これにより、実行時の型チェックなしに、異なる型の要素を含むリストを操作することができます。
///
/// # 型クラス階層における位置
///
/// HList型クラスは独立した型クラスであり、他の型クラスとの直接的な階層関係はありません。
/// しかし、他の型クラスと組み合わせて使用することができます。
///
/// # HList型クラスの構造
///
/// HListは以下の2つの型で構成されます：
///
/// 1. `HNil` - 空のHListを表す型
/// 2. `HCons<H, T>` - 先頭要素の型がHで、残りの要素がT型のHListである型
///
/// # 例
///
/// ```
/// use rust_fp_categories::{HList, HNil, HCons};
/// use rust_fp_categories::hlist;
///
/// // 空のHListを作成
/// let empty = HNil;
///
/// // 要素を追加
/// let list = empty.prepend(42).prepend("hello");
/// assert_eq!(list.head, "hello");
/// assert_eq!(list.tail.head, 42);
///
/// // マクロを使用して作成
/// let list = hlist!["hello", 42, true];
/// assert_eq!(list.head, "hello");
/// assert_eq!(list.tail.head, 42);
/// assert_eq!(list.tail.tail.head, true);
/// ```
pub trait HList: Sized {
    /// 新しい要素をHListの先頭に追加します。
    ///
    /// # 引数
    ///
    /// * `h` - 追加する要素
    ///
    /// # 戻り値
    ///
    /// 新しい要素を先頭に持つHList
    fn prepend<H>(self, h: H) -> HCons<H, Self> {
        HCons {
            head: h,
            tail: self,
        }
    }
}

/// 空のHListを表す型です。
///
/// HNilはHListの終端を表し、要素を持ちません。
///
/// # 例
///
/// ```
/// use rust_fp_categories::{HList, HNil};
///
/// let empty = HNil;
/// let list = empty.prepend(42);
/// assert_eq!(list.head, 42);
/// assert_eq!(list.tail, HNil);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HNil;

impl HList for HNil {}

/// HListの要素を表す型です。
///
/// HConsは先頭要素（head）と残りの要素（tail）を持ちます。
///
/// # 型パラメータ
///
/// * `H` - 先頭要素の型
/// * `T` - 残りの要素のHList型
///
/// # 例
///
/// ```
/// use rust_fp_categories::{HList, HNil, HCons};
///
/// let list = HNil.prepend(42).prepend("hello");
/// assert_eq!(list.head, "hello");
/// assert_eq!(list.tail.head, 42);
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HCons<H, T: HList> {
    /// 先頭要素
    pub head: H,
    /// 残りの要素
    pub tail: T,
}

impl<H, T: HList> HList for HCons<H, T> {}

impl<H: Display, T: HList + Display> Display for HCons<H, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} :: {}", self.head, self.tail)
    }
}

impl Display for HNil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HNil")
    }
}

/// HListの型を表すマクロです。
///
/// このマクロは、HListの型を簡潔に記述するために使用します。
///
/// # 例
///
/// ```
/// use rust_fp_categories::{HList, HNil, HCons, Hlist};
///
/// // 型の定義
/// type MyList = Hlist!(String, i32, bool);
///
/// // 関数の引数の型として使用
/// fn process_list(list: Hlist!(String, i32, bool)) -> String {
///     list.head
/// }
///
/// let list = HNil.prepend(true).prepend(42).prepend("hello".to_string());
/// assert_eq!(process_list(list), "hello".to_string());
/// ```
#[macro_export]
macro_rules! Hlist {
    () => { $crate::HNil };
    ($x:ty) => { $crate::HCons<$x, $crate::HNil> };
    ($x:ty, $($y:ty),*) => { $crate::HCons<$x, Hlist!($($y),*)> };
}

/// HListを作成するマクロです。
///
/// このマクロは、HListのインスタンスを簡潔に作成するために使用します。
///
/// # 例
///
/// ```
/// use rust_fp_categories::{HList, HNil, HCons, hlist};
///
/// let list = hlist!["hello", 42, true];
/// assert_eq!(list.head, "hello");
/// assert_eq!(list.tail.head, 42);
/// assert_eq!(list.tail.tail.head, true);
/// ```
#[macro_export]
macro_rules! hlist {
    () => { $crate::HNil };
    ($x:expr) => { $crate::HCons { head: $x, tail: $crate::HNil } };
    ($x:expr, $($y:expr),*) => { $crate::HCons { head: $x, tail: hlist!($($y),*) } };
}

/// HListに対するShow型クラスの実装
///
/// この実装により、HListの内容を文字列表現に変換することができます。
///
/// # 例
///
/// ```
/// use rust_fp_categories::{HList, HNil, HCons, hlist, Show};
///
/// let list = hlist!["hello", 42, true];
/// let result = list.show();
/// assert_eq!(result, "hello :: 42 :: true :: HNil");
/// ```
impl<H: Display, T: HList + Display> crate::Show for HCons<H, T> {
    type Elm = HCons<H, T>;

    fn show(self) -> String {
        format!("{}", self)
    }
}

/// 空のHListに対するShow型クラスの実装
///
/// この実装により、空のHListを文字列表現に変換することができます。
///
/// # 例
///
/// ```
/// use rust_fp_categories::{HList, HNil, Show};
///
/// let empty = HNil;
/// let result = empty.show();
/// assert_eq!(result, "HNil");
/// ```
impl crate::Show for HNil {
    type Elm = HNil;

    fn show(self) -> String {
        "HNil".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Show;

    #[test]
    fn test_prepend() {
        let list = HNil.prepend(42).prepend("hello");
        assert_eq!(list.head, "hello");
        assert_eq!(list.tail.head, 42);
        assert_eq!(list.tail.tail, HNil);
    }

    #[test]
    fn test_hlist_macro() {
        let list = hlist!["hello", 42, true];
        assert_eq!(list.head, "hello");
        assert_eq!(list.tail.head, 42);
        assert_eq!(list.tail.tail.head, true);
        assert_eq!(list.tail.tail.tail, HNil);
    }

    #[test]
    fn test_hlist_type_macro() {
        fn process_list(list: Hlist!(String, i32, bool)) -> String {
            list.head
        }

        let list = HNil.prepend(true).prepend(42).prepend("hello".to_string());
        assert_eq!(process_list(list), "hello".to_string());
    }

    #[test]
    fn test_display() {
        let list = hlist!["hello", 42, true];
        assert_eq!(format!("{}", list), "hello :: 42 :: true :: HNil");
    }

    #[test]
    fn test_show() {
        let list = hlist!["hello", 42, true];
        assert_eq!(list.show(), "hello :: 42 :: true :: HNil");

        let empty = HNil;
        assert_eq!(empty.show(), "HNil");
    }
}
