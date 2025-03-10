use std::rc::Rc;

use crate::{Applicative, Bind};

/// Monadは、ApplicativeとBindを組み合わせた型クラスです。
///
/// # 型クラス階層における位置
///
/// Monadは型クラス階層の頂点に位置し、最も表現力の高い型クラスです：
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
/// # 特徴
///
/// Monadは以下の機能を組み合わせます：
/// - 値をコンテナにリフトする（Pure）
/// - 関数を含むコンテナを適用する（Apply）
/// - コンテナ内の値に関数を適用し、その結果を平坦化する（Bind）
pub trait Monad: Bind + Applicative {}

use crate::impl_marker_trait_for_numeric;

impl_marker_trait_for_numeric!(Monad);

impl<A: Clone> Monad for Rc<A> {}
impl<A: Clone> Monad for Box<A> {}

impl<A: Clone> Monad for Option<A> {}
impl<A: Clone, E> Monad for Result<A, E> {}
impl<A: Clone> Monad for Vec<A> {}

#[cfg(test)]
mod laws {
    use crate::{Bind, Pure};

    #[quickcheck]
    fn monad_left_identity_law(n: i64) {
        assert_eq!(Option::pure(n).bind(|x| Option::pure(*x)), Option::pure(n))
    }

    #[quickcheck]
    fn monad_right_identity_law(n: i64) {
        assert_eq!(
            Option::pure(n)
                .bind(|x| Option::pure(*x))
                .bind(|y| Option::pure(*y)),
            Option::pure(n).bind(|x| Option::pure(*x).bind(|y| Option::pure(*y)))
        )
    }
}
