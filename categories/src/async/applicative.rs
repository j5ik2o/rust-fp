use super::apply::AsyncApply;
use super::pure::AsyncPure;

/// AsyncApplicativeは、AsyncPureとAsyncApplyの機能を組み合わせた型クラスです。
///
/// # 型クラス階層における位置
///
/// AsyncApplicativeはAsyncPureとAsyncApplyを組み合わせた型クラスで、AsyncMonadの基礎となります：
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
pub trait AsyncApplicative: AsyncPure + AsyncApply {}
