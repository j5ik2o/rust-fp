#[allow(dead_code)]
use std::rc::Rc;

/// Functorは、ある型から別の型への写像を表す型クラスです。
///
/// Functorは、コンテナ型（Option、Result、Vecなど）に対して、
/// その内部の値を変換する機能を提供します。これにより、コンテナの構造を
/// 保ちながら、内部の値だけを変換することができます。
///
/// # 型クラス階層における位置
///
/// Functorは型クラス階層の基本となる型クラスです。他の多くの型クラスはFunctorを拡張しています：
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
/// # Functorの法則
///
/// Functorは以下の2つの法則を満たす必要があります：
///
/// 1. 恒等関数を適用しても値は変わらない（恒等法則）
///    ```rust,ignore
///    x.fmap(|a| a) == x
///    ```
///
/// 2. 関数の合成は、個別に適用した結果と同じ（合成法則）
///    ```rust,ignore
///    x.fmap(|a| f(g(a))) == x.fmap(g).fmap(f)
///    ```
///
/// # 型パラメータ
///
/// * `Elm` - コンテナ内の要素の型
/// * `M<B>` - 変換後のコンテナの型（Bは新しい要素の型）
///
/// # メソッド
///
/// * `fmap` - コンテナ内の各要素に関数を適用し、新しいコンテナを返す
///
/// # 注意
///
/// `fmap`メソッドの関数パラメータは`Fn`トレイトを実装している必要があります。
/// これは、関数が複数回呼び出される可能性があるためです。以前は`FnOnce`を
/// 使用していましたが、一貫性のために`Fn`に統一されました。
pub trait Functor {
    type Elm;
    type M<B: Clone>;

    fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B;
}

use crate::impl_functor_for_numeric;

impl_functor_for_numeric!();

/// `Rc<A>`に対するFunctorの実装
///
/// この実装により、参照カウント型のコンテナ内の値を変換することができます。
///
/// # 例
///
/// ```
/// use std::rc::Rc;
/// use rust_fp_categories::Functor;
///
/// let value = Rc::new(5);
/// let result = value.fmap(|x| x * 2);
/// assert_eq!(*result, 10);
/// ```
impl<A> Functor for Rc<A> {
    type Elm = A;
    type M<U: Clone> = Rc<U>;

    fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        crate::common::rc::fmap(self, f)
    }
}

/// `Box<A>`に対するFunctorの実装
///
/// この実装により、ヒープ割り当て型のコンテナ内の値を変換することができます。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Functor;
///
/// let value = Box::new(5);
/// let result = value.fmap(|x| x * 2);
/// assert_eq!(*result, 10);
/// ```
impl<A> Functor for Box<A> {
    type Elm = A;
    type M<U: Clone> = Box<U>;

    fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        crate::common::boxed::fmap(self, f)
    }
}

// ---

/// `Option<A>`に対するFunctorの実装
///
/// この実装により、Optionコンテナ内の値を変換することができます。
/// Noneの場合は変換されず、Noneのままです。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Functor;
///
/// let some_value = Some(5);
/// let result = some_value.fmap(|x| x * 2);
/// assert_eq!(result, Some(10));
///
/// let none_value: Option<i32> = None;
/// let result = none_value.fmap(|x| x * 2);
/// assert_eq!(result, None);
/// ```
impl<A> Functor for Option<A> {
    type Elm = A;
    type M<B: Clone> = Option<B>;

    fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        crate::common::option::fmap(self, f)
    }
}

/// `Result<A, E>`に対するFunctorの実装
///
/// この実装により、Resultコンテナ内の値を変換することができます。
/// Errの場合は変換されず、元のエラー値を保持したままです。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Functor;
///
/// let ok_value: Result<i32, &str> = Ok(5);
/// let result = ok_value.fmap(|x| x * 2);
/// assert_eq!(result, Ok(10));
///
/// let err_value: Result<i32, &str> = Err("エラー");
/// let result = err_value.fmap(|x| x * 2);
/// assert_eq!(result, Err("エラー"));
/// ```
impl<A, E> Functor for Result<A, E> {
    type Elm = A;
    type M<B: Clone> = Result<B, E>;

    fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        crate::common::result::fmap(self, f)
    }
}

/// `Vec<A>`に対するFunctorの実装
///
/// この実装により、ベクトル内の各要素を変換することができます。
/// 空のベクトルの場合は、空のベクトルが返されます。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Functor;
///
/// let values = vec![1, 2, 3, 4, 5];
/// let result = values.fmap(|x| x * 2);
/// assert_eq!(result, vec![2, 4, 6, 8, 10]);
///
/// let empty: Vec<i32> = vec![];
/// let result = empty.fmap(|x| x * 2);
/// assert_eq!(result, Vec::<i32>::new());
/// ```
impl<A> Functor for Vec<A> {
    type Elm = A;
    type M<B: Clone> = Vec<B>;

    fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
    where
        F: Fn(&Self::Elm) -> B,
    {
        crate::common::vec::fmap(self, f)
    }
}

#[cfg(test)]
mod laws {
    mod option {
        use crate::Functor;
        use std::convert::identity;

        #[quickcheck]
        fn covariant_identity(n: Option<i32>) -> bool {
            n.fmap(|x| identity(*x)) == n
        }

        #[quickcheck]
        fn covariant_composition(n: Option<i32>) -> bool {
            let f1: fn(&i32) -> i32 = |x| *x * 2;
            let f2: fn(&i32) -> i32 = |x| *x + 4;
            n.fmap(f1).fmap(f2) == n.fmap(|x| f2(&f1(x)))
        }
    }

    mod result {
        use crate::Functor;
        use std::convert::identity;

        #[quickcheck]
        fn covariant_identity(n: Result<i32, String>) -> bool {
            let expected = n.clone();
            n.fmap(|x| identity(*x)) == expected
        }

        #[quickcheck]
        fn covariant_composition(n: Result<i32, String>) -> bool {
            let expected = n.clone();
            let f1: fn(&i32) -> i32 = |x| *x * 2;
            let f2: fn(&i32) -> i32 = |x| *x + 4;
            n.fmap(f1).fmap(f2) == expected.fmap(|x| f2(&f1(x)))
        }
    }

    mod vec {
        use crate::Functor;
        use std::convert::identity;

        #[quickcheck]
        fn covariant_identity(n: Vec<i32>) -> bool {
            let expected = n.clone();
            n.fmap(|x| identity(*x)) == expected
        }

        #[quickcheck]
        fn covariant_composition(n: Vec<i32>) -> bool {
            let expected = n.clone();
            let f1: fn(&i32) -> i32 = |x| *x * 2;
            let f2: fn(&i32) -> i32 = |x| *x + 4;
            n.fmap(f1).fmap(f2) == expected.fmap(|x| f2(&f1(x)))
        }
    }
}
