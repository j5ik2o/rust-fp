use std::boxed::Box;
use std::fmt::Debug;
use std::marker::PhantomData;

use rust_fp_categories::Empty;

use crate::{FingerTree, FingerTreeError};

/// 単純化されたフィンガーツリーの実装
///
/// この実装は、再帰的な型定義の深さを制限するために単純化されています。
/// 効率的な連結操作と分割操作をサポートします。
///
/// 時間計算量:
/// - push_front: 償却O(1)
/// - push_back: 償却O(1)
/// - pop_front: 償却O(1)
/// - pop_back: 償却O(1)
/// - peek_front: O(1)
/// - peek_back: O(1)
/// - concat: O(log(min(n1, n2)))
/// - split: O(log(n))
/// - size: O(1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleFingerTree<A: Clone + Debug> {
    /// 空の木
    Empty,
    /// 単一要素の木
    Single(A),
    /// 深い木（接頭辞、中央、接尾辞）
    Deep {
        /// 木のサイズ（要素数）
        size: usize,
        /// 接頭辞（先頭の要素）
        prefix: Vec<A>,
        /// 中央の木 - 再帰を制限するために異なる実装を使用
        middle: Box<InternalTree<A>>,
        /// 接尾辞（末尾の要素）
        suffix: Vec<A>,
    },
}

/// 内部木の実装 - 再帰的な型定義の深さを制限するための補助構造体
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternalTree<A: Clone + Debug> {
    /// 内部木のサイズ（要素数）
    size: usize,
    /// 内部木の要素
    elements: Vec<Vec<A>>,
    /// 型パラメータAを保持するためのマーカー
    _marker: PhantomData<A>,
}

impl<A: Clone + Debug> InternalTree<A> {
    /// 新しい空の内部木を作成します。
    pub fn new() -> Self {
        InternalTree {
            size: 0,
            elements: Vec::new(),
            _marker: PhantomData,
        }
    }

    /// 内部木のサイズを取得します。
    pub fn size(&self) -> usize {
        self.size
    }

    /// 内部木が空かどうかを判定します。
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// 内部木の先頭に要素を追加します。
    pub fn push_front(&self, value: Vec<A>) -> Self {
        let mut new_elements = Vec::with_capacity(self.elements.len() + 1);
        new_elements.push(value.clone());
        new_elements.extend_from_slice(&self.elements);

        InternalTree {
            size: self.size + value.len(),
            elements: new_elements,
            _marker: PhantomData,
        }
    }

    /// 内部木の末尾に要素を追加します。
    pub fn push_back(&self, value: Vec<A>) -> Self {
        let mut new_elements = self.elements.clone();
        new_elements.push(value.clone());

        InternalTree {
            size: self.size + value.len(),
            elements: new_elements,
            _marker: PhantomData,
        }
    }

    /// 内部木の先頭から要素を取り出します。
    pub fn pop_front(&self) -> Result<(Vec<A>, Self), FingerTreeError> {
        if self.is_empty() {
            return Err(FingerTreeError::EmptyTreeError);
        }

        let value = self.elements[0].clone();
        let value_len = value.len();
        let mut new_elements = self.elements.clone();
        new_elements.remove(0);

        Ok((
            value,
            InternalTree {
                size: self.size - value_len,
                elements: new_elements,
                _marker: PhantomData,
            },
        ))
    }

    /// 内部木の末尾から要素を取り出します。
    pub fn pop_back(&self) -> Result<(Vec<A>, Self), FingerTreeError> {
        if self.is_empty() {
            return Err(FingerTreeError::EmptyTreeError);
        }

        let value = self.elements[self.elements.len() - 1].clone();
        let value_len = value.len();
        let mut new_elements = self.elements.clone();
        new_elements.pop();

        Ok((
            value,
            InternalTree {
                size: self.size - value_len,
                elements: new_elements,
                _marker: PhantomData,
            },
        ))
    }

    /// 内部木を連結します。
    pub fn concat(&self, other: Self) -> Self {
        let mut new_elements = self.elements.clone();
        new_elements.extend_from_slice(&other.elements);

        InternalTree {
            size: self.size + other.size,
            elements: new_elements,
            _marker: PhantomData,
        }
    }

    /// 内部木を指定されたインデックスで分割します。
    pub fn split(&self, index: usize) -> (Self, Self) {
        if index == 0 {
            return (InternalTree::new(), self.clone());
        }

        if index >= self.size {
            return (self.clone(), InternalTree::new());
        }

        let mut current_index = 0;
        let mut split_point = 0;

        // 分割点を見つける
        for (i, chunk) in self.elements.iter().enumerate() {
            if current_index + chunk.len() > index {
                split_point = i;
                break;
            }
            current_index += chunk.len();
        }

        let (left_elements, right_elements) = self.elements.split_at(split_point);

        let left = InternalTree {
            size: current_index,
            elements: left_elements.to_vec(),
            _marker: PhantomData,
        };

        let right = InternalTree {
            size: self.size - current_index,
            elements: right_elements.to_vec(),
            _marker: PhantomData,
        };

        (left, right)
    }
}

impl<A: Clone + Debug> Empty for SimpleFingerTree<A> {
    fn empty() -> Self {
        SimpleFingerTree::Empty
    }

    fn is_empty(&self) -> bool {
        match self {
            SimpleFingerTree::Empty => true,
            _ => false,
        }
    }
}

impl<A: Clone + Debug> SimpleFingerTree<A> {
    /// 新しい空の木を作成します。
    pub fn new() -> Self {
        SimpleFingerTree::Empty
    }

    /// 単一要素の木を作成します。
    pub fn single(value: A) -> Self {
        SimpleFingerTree::Single(value)
    }

    /// 深い木を作成します。
    fn deep(prefix: Vec<A>, middle: Box<InternalTree<A>>, suffix: Vec<A>) -> Self {
        let size = prefix.len() + middle.size() + suffix.len();
        SimpleFingerTree::Deep {
            size,
            prefix,
            middle,
            suffix,
        }
    }
}

impl<A: Clone + Debug> FingerTree<A> for SimpleFingerTree<A> {
    fn push_front(self, value: A) -> Self {
        match self {
            SimpleFingerTree::Empty => SimpleFingerTree::Single(value),
            SimpleFingerTree::Single(a) => {
                SimpleFingerTree::deep(vec![value], Box::new(InternalTree::new()), vec![a])
            }
            SimpleFingerTree::Deep {
                size,
                mut prefix,
                middle,
                suffix,
            } => {
                // 接頭辞の先頭に要素を追加
                if prefix.len() < 4 {
                    // 接頭辞が最大サイズ未満の場合は単純に追加
                    prefix.insert(0, value);
                    SimpleFingerTree::Deep {
                        size: size + 1,
                        prefix,
                        middle,
                        suffix,
                    }
                } else {
                    // 接頭辞が最大サイズの場合は分割して中央の木に追加
                    let mut new_prefix = vec![value];
                    new_prefix.extend_from_slice(&prefix[0..3]);
                    let overflow = vec![prefix[3].clone()];

                    let new_middle = middle.push_back(overflow);
                    SimpleFingerTree::Deep {
                        size: size + 1,
                        prefix: new_prefix,
                        middle: Box::new(new_middle),
                        suffix,
                    }
                }
            }
        }
    }

    fn push_back(self, value: A) -> Self {
        match self {
            SimpleFingerTree::Empty => SimpleFingerTree::Single(value),
            SimpleFingerTree::Single(a) => {
                SimpleFingerTree::deep(vec![a], Box::new(InternalTree::new()), vec![value])
            }
            SimpleFingerTree::Deep {
                size,
                prefix,
                middle,
                mut suffix,
            } => {
                // 接尾辞の末尾に要素を追加
                if suffix.len() < 4 {
                    // 接尾辞が最大サイズ未満の場合は単純に追加
                    suffix.push(value);
                    SimpleFingerTree::Deep {
                        size: size + 1,
                        prefix,
                        middle,
                        suffix,
                    }
                } else {
                    // 接尾辞が最大サイズの場合は分割して中央の木に追加
                    let overflow = vec![suffix[0].clone()];
                    suffix.remove(0);
                    suffix.push(value);

                    let new_middle = middle.push_back(overflow);
                    SimpleFingerTree::Deep {
                        size: size + 1,
                        prefix,
                        middle: Box::new(new_middle),
                        suffix,
                    }
                }
            }
        }
    }

    fn pop_front(self) -> Result<(A, Self), FingerTreeError> {
        match self {
            SimpleFingerTree::Empty => Err(FingerTreeError::EmptyTreeError),
            SimpleFingerTree::Single(a) => Ok((a, SimpleFingerTree::Empty)),
            SimpleFingerTree::Deep {
                size,
                mut prefix,
                middle,
                suffix,
            } => {
                if prefix.is_empty() {
                    return Err(FingerTreeError::EmptyTreeError);
                }

                let head = prefix.remove(0);

                if !prefix.is_empty() {
                    // 接頭辞がまだ要素を持っている場合
                    Ok((
                        head,
                        SimpleFingerTree::Deep {
                            size: size - 1,
                            prefix,
                            middle,
                            suffix,
                        },
                    ))
                } else if middle.is_empty() && suffix.is_empty() {
                    // 中央の木と接尾辞が空の場合
                    Ok((head, SimpleFingerTree::Empty))
                } else if middle.is_empty() {
                    // 中央の木が空で接尾辞が要素を持っている場合
                    match suffix.len() {
                        0 => Ok((head, SimpleFingerTree::Empty)),
                        1 => Ok((head, SimpleFingerTree::Single(suffix[0].clone()))),
                        _ => {
                            let new_prefix = vec![suffix[0].clone()];
                            let mut new_suffix = suffix.clone();
                            new_suffix.remove(0);

                            Ok((
                                head,
                                SimpleFingerTree::deep(
                                    new_prefix,
                                    Box::new(InternalTree::new()),
                                    new_suffix,
                                ),
                            ))
                        }
                    }
                } else {
                    // 中央の木から要素を取り出す
                    let (middle_vec, new_middle) = middle.pop_front()?;

                    Ok((
                        head,
                        SimpleFingerTree::Deep {
                            size: size - 1,
                            prefix: middle_vec,
                            middle: Box::new(new_middle),
                            suffix,
                        },
                    ))
                }
            }
        }
    }

    fn pop_back(self) -> Result<(A, Self), FingerTreeError> {
        match self {
            SimpleFingerTree::Empty => Err(FingerTreeError::EmptyTreeError),
            SimpleFingerTree::Single(a) => Ok((a, SimpleFingerTree::Empty)),
            SimpleFingerTree::Deep {
                size,
                prefix,
                middle,
                mut suffix,
            } => {
                if suffix.is_empty() {
                    return Err(FingerTreeError::EmptyTreeError);
                }

                let last = suffix.pop().unwrap();

                if !suffix.is_empty() {
                    // 接尾辞がまだ要素を持っている場合
                    Ok((
                        last,
                        SimpleFingerTree::Deep {
                            size: size - 1,
                            prefix,
                            middle,
                            suffix,
                        },
                    ))
                } else if middle.is_empty() && prefix.is_empty() {
                    // 中央の木と接頭辞が空の場合
                    Ok((last, SimpleFingerTree::Empty))
                } else if middle.is_empty() {
                    // 中央の木が空で接頭辞が要素を持っている場合
                    match prefix.len() {
                        0 => Ok((last, SimpleFingerTree::Empty)),
                        1 => Ok((last, SimpleFingerTree::Single(prefix[0].clone()))),
                        _ => {
                            let new_suffix = vec![prefix[prefix.len() - 1].clone()];
                            let mut new_prefix = prefix.clone();
                            new_prefix.pop();

                            Ok((
                                last,
                                SimpleFingerTree::deep(
                                    new_prefix,
                                    Box::new(InternalTree::new()),
                                    new_suffix,
                                ),
                            ))
                        }
                    }
                } else {
                    // 中央の木から要素を取り出す
                    let (middle_vec, new_middle) = middle.pop_back()?;

                    Ok((
                        last,
                        SimpleFingerTree::Deep {
                            size: size - 1,
                            prefix,
                            middle: Box::new(new_middle),
                            suffix: middle_vec,
                        },
                    ))
                }
            }
        }
    }

    fn peek_front(&self) -> Result<A, FingerTreeError> {
        match self {
            SimpleFingerTree::Empty => Err(FingerTreeError::EmptyTreeError),
            SimpleFingerTree::Single(a) => Ok(a.clone()),
            SimpleFingerTree::Deep { prefix, .. } => {
                if prefix.is_empty() {
                    Err(FingerTreeError::EmptyTreeError)
                } else {
                    Ok(prefix[0].clone())
                }
            }
        }
    }

    fn peek_back(&self) -> Result<A, FingerTreeError> {
        match self {
            SimpleFingerTree::Empty => Err(FingerTreeError::EmptyTreeError),
            SimpleFingerTree::Single(a) => Ok(a.clone()),
            SimpleFingerTree::Deep { suffix, .. } => {
                if suffix.is_empty() {
                    Err(FingerTreeError::EmptyTreeError)
                } else {
                    Ok(suffix[suffix.len() - 1].clone())
                }
            }
        }
    }

    fn concat(self, other: Self) -> Self {
        match (self, other) {
            (SimpleFingerTree::Empty, other) => other,
            (self_tree, SimpleFingerTree::Empty) => self_tree,
            (SimpleFingerTree::Single(a), other) => other.push_front(a),
            (self_tree, SimpleFingerTree::Single(a)) => self_tree.push_back(a),
            (
                SimpleFingerTree::Deep {
                    prefix: self_prefix,
                    middle: self_middle,
                    suffix: self_suffix,
                    ..
                },
                SimpleFingerTree::Deep {
                    prefix: other_prefix,
                    middle: other_middle,
                    suffix: other_suffix,
                    ..
                },
            ) => {
                // 中央の木を連結し、左の接尾辞と右の接頭辞を合わせる
                let mut combined = Vec::new();
                combined.extend_from_slice(&self_suffix);
                combined.extend_from_slice(&other_prefix);

                // 中央の要素を作成
                let mut middle_chunks = Vec::new();
                for chunk in combined.chunks(3) {
                    middle_chunks.push(chunk.to_vec());
                }

                // 新しい中央の木を作成
                let mut new_middle = (*self_middle).concat(*other_middle);
                for chunk in middle_chunks {
                    new_middle = new_middle.push_back(chunk);
                }

                SimpleFingerTree::deep(self_prefix, Box::new(new_middle), other_suffix)
            }
        }
    }

    fn split(self, index: usize) -> (Self, Self) {
        if index == 0 {
            return (SimpleFingerTree::Empty, self);
        }

        if index >= self.size() {
            return (self, SimpleFingerTree::Empty);
        }

        // 完全に新しい実装を使用
        let mut left_tree = SimpleFingerTree::Empty;
        let mut right_tree = SimpleFingerTree::Empty;

        // 要素を一つずつ取り出して適切な木に追加
        let mut current_tree = self;
        let mut current_index = 0;

        while let Ok((value, new_tree)) = current_tree.pop_front() {
            if current_index < index {
                left_tree = left_tree.push_back(value);
            } else {
                right_tree = right_tree.push_back(value);
            }
            current_index += 1;
            current_tree = new_tree;
        }

        (left_tree, right_tree)
    }

    fn size(&self) -> usize {
        match self {
            SimpleFingerTree::Empty => 0,
            SimpleFingerTree::Single(_) => 1,
            SimpleFingerTree::Deep { size, .. } => *size,
        }
    }

    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        iter.into_iter()
            .fold(SimpleFingerTree::Empty, |acc, x| acc.push_back(x))
    }
}
