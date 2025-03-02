use super::applicative::AsyncApplicative;
use super::bind::AsyncBind;

/// AsyncMonadは、AsyncApplicativeとAsyncBindの機能を組み合わせた型クラスです。
///
/// # 型クラス階層における位置
///
/// AsyncMonadはAsyncApplicativeとAsyncBindを組み合わせた型クラスです：
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
pub trait AsyncMonad: AsyncApplicative + AsyncBind {}
