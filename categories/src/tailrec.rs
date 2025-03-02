//! TailRec型クラスは、末尾再帰最適化を提供する型クラスです。
//!
//! # 概要
//!
//! TailRec型クラスは、再帰関数のスタックオーバーフローを防ぎ、効率的な再帰処理を実現します。
//! 通常の再帰関数では、再帰の深さが深くなるとスタックオーバーフローが発生する可能性がありますが、
//! TailRec型クラスを使用することで、再帰をループに変換し、スタックオーバーフローを防ぐことができます。
//!
//! # 使用例
//!
//! ```
//! use rust_fp_categories::{RecursionState, TailRec};
//!
//! // 10から0までカウントダウンする再帰関数
//! let result = 10.rec(|x| match x {
//!     0 => RecursionState::Done(()),
//!     k => RecursionState::Continue(k - 1)
//! });
//!
//! assert_eq!(result, ());
//! ```
//!
//! # スタンドアロン関数
//!
//! TailRec型クラスを使わずに直接使用できる`tail_rec`関数も提供されています。
//!
//! ```
//! use rust_fp_categories::{RecursionState, tail_rec};
//!
//! let result = tail_rec(10, |x| match x {
//!     0 => RecursionState::Done(()),
//!     k => RecursionState::Continue(k - 1)
//! });
//!
//! assert_eq!(result, ());
//! ```

/// 再帰状態を表す列挙型です。
///
/// この列挙型は、再帰処理の状態を表現するために使用されます。
/// `Continue`は再帰を続行することを、`Done`は再帰を終了することを示します。
///
/// # 型パラメータ
///
/// * `Done` - 再帰処理の結果の型
/// * `Cont` - 再帰処理の中間状態の型
///
/// # 例
///
/// ```
/// use rust_fp_categories::{RecursionState, TailRec};
///
/// let result = 10.rec(|x| match x {
///     0 => RecursionState::Done(()),
///     k => RecursionState::Continue(k - 1)
/// });
///
/// assert_eq!(result, ());
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RecursionState<Done, Cont> {
    /// 再帰を続行することを示します。
    Continue(Cont),
    /// 再帰を終了し、結果を返すことを示します。
    Done(Done),
}

/// TailRecは、末尾再帰最適化を提供する型クラスです。
///
/// # 型クラス階層における位置
///
/// TailRecは独立した型クラスであり、他の型クラスとの直接的な階層関係はありません。
///
/// # 型パラメータ
///
/// * `Output` - 再帰処理の結果の型
///
/// # メソッド
///
/// * `rec` - 末尾再帰関数を最適化して実行する
/// * `rec_ref` - 参照を使用して末尾再帰関数を最適化して実行する
///
/// # 例
///
/// ```
/// use rust_fp_categories::{RecursionState, TailRec};
///
/// // 10から0までカウントダウンする再帰関数
/// let result = 10.rec(|x| match x {
///     0 => RecursionState::Done(()),
///     k => RecursionState::Continue(k - 1)
/// });
///
/// assert_eq!(result, ());
/// ```
pub trait TailRec<Output> {
    /// 末尾再帰関数を最適化して実行します。
    ///
    /// この関数は、再帰関数をループに変換することで、スタックオーバーフローを防ぎます。
    ///
    /// # 引数
    ///
    /// * `iterate` - 再帰処理を行う関数
    ///
    /// # 戻り値
    ///
    /// 再帰処理の結果
    ///
    /// # 例
    ///
    /// ```
    /// use rust_fp_categories::{RecursionState, TailRec};
    ///
    /// let result = 10.rec(|x| match x {
    ///     0 => RecursionState::Done(()),
    ///     k => RecursionState::Continue(k - 1)
    /// });
    ///
    /// assert_eq!(result, ());
    /// ```
    #[inline]
    fn rec<F>(self, iterate: F) -> Output
    where
        Self: Sized,
        F: Fn(Self) -> RecursionState<Output, Self>,
    {
        let mut state = iterate(self);

        loop {
            match state {
                RecursionState::Done(output) => return output,
                RecursionState::Continue(more) => state = iterate(more),
            }
        }
    }

    /// 参照を使用して末尾再帰関数を最適化して実行します。
    ///
    /// この関数は、再帰関数をループに変換することで、スタックオーバーフローを防ぎます。
    /// `rec`メソッドとは異なり、このメソッドは参照を使用します。
    ///
    /// # 引数
    ///
    /// * `iterate` - 再帰処理を行う関数
    ///
    /// # 戻り値
    ///
    /// 再帰処理の結果
    ///
    /// # 例
    ///
    /// ```
    /// use rust_fp_categories::{RecursionState, TailRec};
    ///
    /// let result = 10.rec_ref(|x| match *x {
    ///     0 => RecursionState::Done(()),
    ///     k => RecursionState::Continue(k - 1)
    /// });
    ///
    /// assert_eq!(result, ());
    /// ```
    #[inline]
    fn rec_ref<F>(&self, iterate: F) -> Output
    where
        Self: Sized + Clone,
        F: Fn(&Self) -> RecursionState<Output, Self>,
    {
        let mut state = iterate(self);

        loop {
            match state {
                RecursionState::Done(output) => return output,
                RecursionState::Continue(more) => state = iterate(&more),
            }
        }
    }
}

/// すべての型に対するTailRecの実装
///
/// この実装により、任意の型に対して末尾再帰最適化を使用することができます。
///
/// # 例
///
/// ```
/// use rust_fp_categories::{RecursionState, TailRec};
///
/// let result = 10.rec(|x| match x {
///     0 => RecursionState::Done(()),
///     k => RecursionState::Continue(k - 1)
/// });
///
/// assert_eq!(result, ());
/// ```
impl<T, Output> TailRec<Output> for T {}

/// 末尾再帰関数を最適化して実行するスタンドアロン関数です。
///
/// この関数は、TailRecトレイトを使わずに直接使用することができます。
///
/// # 引数
///
/// * `input` - 初期値
/// * `iterate` - 再帰処理を行う関数
///
/// # 戻り値
///
/// 再帰処理の結果
///
/// # 例
///
/// ```
/// use rust_fp_categories::{RecursionState, tail_rec};
///
/// let result = tail_rec(10, |x| match x {
///     0 => RecursionState::Done(()),
///     k => RecursionState::Continue(k - 1)
/// });
///
/// assert_eq!(result, ());
/// ```
#[inline]
pub fn tail_rec<Input, Output, F>(input: Input, iterate: F) -> Output
where
    Input: Sized,
    F: Fn(Input) -> RecursionState<Output, Input>,
{
    let mut state = iterate(input);

    loop {
        match state {
            RecursionState::Done(output) => return output,
            RecursionState::Continue(more) => state = iterate(more),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tail_rec() {
        let result = 10.rec(|x| match x {
            0 => RecursionState::Done(()),
            k => RecursionState::Continue(k - 1),
        });

        assert_eq!(result, ());
    }

    #[test]
    fn test_tail_rec_ref() {
        let result = 10.rec_ref(|x| match *x {
            0 => RecursionState::Done(()),
            k => RecursionState::Continue(k - 1),
        });

        assert_eq!(result, ());
    }

    #[test]
    fn test_tail_rec_as_func() {
        let result = tail_rec(10000, |x| match x {
            0 => RecursionState::Done(()),
            k => RecursionState::Continue(k - 1),
        });

        assert_eq!(result, ());
    }

    #[test]
    fn test_tail_rec_with_option() {
        let result = Some(10).rec(|opt| match opt {
            Some(0) => RecursionState::Done(Some(0)),
            Some(k) => RecursionState::Continue(Some(k - 1)),
            None => RecursionState::Done(None),
        });

        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_tail_rec_with_result() {
        let result: Result<i32, &str> = Ok(10).rec(|res| match res {
            Ok(0) => RecursionState::Done(Ok(0)),
            Ok(k) => RecursionState::Continue(Ok(k - 1)),
            Err(e) => RecursionState::Done(Err(e)),
        });

        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_tail_rec_with_large_recursion() {
        // 通常の再帰ではスタックオーバーフローを起こす深さ
        let large_number = 1_000_000;
        let result = tail_rec(large_number, |x| match x {
            0 => RecursionState::Done(0),
            k => RecursionState::Continue(k - 1),
        });

        assert_eq!(result, 0);
    }

    #[test]
    fn test_tail_rec_with_accumulator() {
        // 累積値を使った再帰関数（階乗の計算）
        let result = tail_rec((5, 1), |(n, acc)| match n {
            0 => RecursionState::Done(acc),
            _ => RecursionState::Continue((n - 1, acc * n)),
        });

        assert_eq!(result, 120); // 5! = 120
    }

    #[test]
    fn test_tail_rec_with_vec() {
        // ベクトルを使った再帰関数（フィボナッチ数列の計算）
        let result = tail_rec((10, vec![0, 1]), |(n, mut acc)| match n {
            0 => RecursionState::Done(acc),
            _ => {
                let len = acc.len();
                let next = acc[len - 1] + acc[len - 2];
                acc.push(next);
                RecursionState::Continue((n - 1, acc))
            }
        });

        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }
}
