//! for_yield マクロは、Scalaのfor/yield構文やHaskellのdo記法に似た、
//! flat_map/mapのネスト解消するマクロです。
//!
//! # 概要
//!
//! このマクロは、複雑なモナド計算を簡潔に記述できるようにするためのものです。
//! ネストされたbind（flat_map）とfmap（map）の呼び出しを、より読みやすく書けるようにします。
//!
//! # 例
//!
//! ```
//! use rust_fp_categories::{Bind, Functor, for_yield};
//!
//! let result = for_yield! {
//!     bind a = Some(1);
//!     bind b = Some(2);
//!     let c = a + b;
//!     yield Some(c * 2)
//! };
//! assert_eq!(result, Some(6));
//! ```
//!
//! 上記のコードは、以下のコードと等価です：
//!
//! ```
//! use rust_fp_categories::{Bind, Functor};
//!
//! let result = Some(1).bind(|a| {
//!     Some(2).bind(|b| {
//!         let c = a + b;
//!         Some(c * 2)
//!     })
//! });
//! assert_eq!(result, Some(6));
//! ```
//!
//! # 構文
//!
//! for_yield! マクロは以下の構文をサポートしています：
//!
//! - `bind a = expr;` - bind操作（flat_map）。exprはBindを実装している必要があります。
//! - `let a = expr;` - 変数への代入。
//! - `yield expr` - 最後の式（結果を返す）。
//!
//! # 制限事項
//!
//! - マクロ内の各式は、セミコロンで区切る必要があります（最後の式を除く）。
//! - bind操作を使用する場合、左辺は単一の識別子である必要があります。
//! - 最後の式は、セミコロンを付けてはいけません。

/// for_yield マクロは、Scalaのfor/yield構文やHaskellのdo記法に似た、
/// flat_map/mapのネスト解消するマクロです。
///
/// # 例
///
/// ```
/// use rust_fp_categories::{Bind, Functor, for_yield};
///
/// let result = for_yield! {
///     bind a = Some(1);
///     bind b = Some(2);
///     let c = a + b;
///     yield Some(c * 2)
/// };
/// assert_eq!(result, Some(6));
/// ```
#[macro_export]
macro_rules! for_yield {
    // 最後の式（結果を返す）
    (yield $e:expr) => {
        $e
    };

    // 最後の式（自動的にコンテナに包む）
    ($e:expr) => {
        $e
    };

    // bind操作
    (bind $i:ident = $e:expr; $($rest:tt)*) => {
        $e.bind(|$i| for_yield!($($rest)*))
    };

    // 変数への代入
    (let $i:ident = $e:expr; $($rest:tt)*) => {
        {
            let $i = $e;
            for_yield!($($rest)*)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{Bind, Functor};

    #[test]
    fn test_for_yield_with_option() {
        let result = for_yield! {
            bind a = Some(1);
            bind b = Some(2);
            let c = a + b;
            yield Some(c * 2)
        };
        assert_eq!(result, Some(6));

        // Noneの場合
        let result = for_yield! {
            bind a = Some(1);
            bind b = None::<i32>;
            let c = a + b;
            yield Some(c * 2)
        };
        assert_eq!(result, None);
    }

    #[test]
    fn test_for_yield_with_result() {
        let result: Result<i32, &str> = for_yield! {
            bind a = Ok(1);
            bind b = Ok(2);
            let c = a + b;
            yield Ok(c * 2)
        };
        assert_eq!(result, Ok(6));

        // Errの場合
        let result: Result<i32, &str> = for_yield! {
            bind a = Ok(1);
            bind b = Err("エラー");
            let c = a + b;
            yield Ok(c * 2)
        };
        assert_eq!(result, Err("エラー"));
    }

    #[test]
    fn test_for_yield_with_vec() {
        let result = for_yield! {
            bind a = vec![1, 2];
            bind b = vec![10, 20];
            let c = a + b;
            yield vec![c * 2]
        };
        assert_eq!(result, vec![22, 42, 24, 44]);
    }

    #[test]
    fn test_for_yield_nested() {
        let result = for_yield! {
            bind a = Some(1);
            bind b = for_yield! {
                bind x = Some(2);
                bind y = Some(3);
                yield Some(x + y)
            };
            let c = a + b;
            yield Some(c * 2)
        };
        assert_eq!(result, Some(12));
    }
}
