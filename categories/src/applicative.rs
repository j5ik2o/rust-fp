use std::rc::Rc;

use crate::{Apply, Pure};

/// Applicativeは、ApplyとPureを組み合わせた型クラスです。
///
/// # 型クラス階層における位置
///
/// ApplicativeはApplyとPureを組み合わせた型クラスで、Monadの基礎となります：
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
/// Applicativeは以下の機能を組み合わせます：
/// - 値をコンテナにリフトする（Pure）
/// - 関数を含むコンテナを適用する（Apply）
pub trait Applicative: Apply + Pure {}

use crate::impl_marker_trait_for_numeric;

impl_marker_trait_for_numeric!(Applicative);

impl<A: Clone> Applicative for Rc<A> {}
impl<A: Clone> Applicative for Box<A> {}

impl<A: Clone> Applicative for Option<A> {}
impl<A: Clone, E> Applicative for Result<A, E> {}
impl<A: Clone> Applicative for Vec<A> {}
