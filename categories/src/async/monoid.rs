use crate::r#async::{AsyncEmpty, AsyncSemigroup};

/// AsyncMonoidは、非同期的な単位元と結合操作を提供する型クラスです。
///
/// # 型クラス階層における位置
///
/// AsyncMonoidは非同期カテゴリの基本となる型クラスで、AsyncEmptyとAsyncSemigroupを組み合わせたものです。
///
/// # トレイト階層
///
/// ```text
///     AsyncEmpty   AsyncSemigroup
///         \           /
///          \         /
///           \       /
///          AsyncMonoid
/// ```
pub trait AsyncMonoid: AsyncEmpty + AsyncSemigroup {}
