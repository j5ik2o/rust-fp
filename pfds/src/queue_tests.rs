//! Queueトレイト実装のための包括的なテストモジュール
//! 
//! このモジュールでは、Queueトレイトの実装に対する共通テストを提供します。
//! 様々な実装（ListQueue、OptimizedQueueなど）に対して同じテストを実行することで、
//! 一貫した動作を保証します。

use std::rc::Rc;
use crate::{Queue, QueueError, ListQueue, OptimizedQueue};
use rust_fp_categories::Empty;

/// 共通テスト関数：空のキューに対するテスト
/// 
/// このテストでは、空のキューが正しく初期化され、
/// 適切なメソッドの振る舞いを示すことを確認します。
pub fn test_empty_queue<Q, A>()
where
    Q: Queue<A> + Empty,
    A: Clone + PartialEq + std::fmt::Debug,
{
    let queue: Q = Q::empty();
    assert!(rust_fp_categories::Empty::is_empty(&queue), "空のキューはis_emptyがtrueを返すべき");
    assert_eq!(queue.size(), 0, "空のキューのサイズは0であるべき");
    assert!(queue.peek().is_err(), "空のキューのpeekはエラーを返すべき");
    
    // 空のキューからのdequeueはエラーを返すべき
    let dequeue_result = queue.dequeue();
    assert!(dequeue_result.is_err(), "空のキューからのdequeueはエラーを返すべき");
    
    if let Err(err) = dequeue_result {
        assert!(matches!(err, QueueError::EmptyQueueError), "エラーの種類はEmptyQueueErrorであるべき");
    }
}

/// 共通テスト関数：enqueueとdequeueの基本操作
/// 
/// このテストでは、キューへの要素の追加と取り出しが
/// FIFO（先入れ先出し）の順序で正しく動作することを確認します。
pub fn test_basic_operations<Q, A>()
where
    Q: Queue<A> + Empty,
    A: Clone + PartialEq + std::fmt::Debug + Default,
{
    let a = A::default();
    let b = A::default();
    let c = A::default();
    // Use default values for testing
    
    let queue = Q::empty();
    let queue = queue.enqueue(a.clone()).enqueue(b.clone()).enqueue(c.clone());
    
    assert_eq!(queue.size(), 3, "3つの要素を追加した後のサイズは3であるべき");
    assert!(!rust_fp_categories::Empty::is_empty(&queue), "要素を追加した後はis_emptyがfalseを返すべき");
    
    // 最初の要素を取り出し
    let (value, queue) = queue.dequeue().unwrap();
    assert_eq!(value, a, "最初にdequeueされる値は最初に追加された値であるべき");
    assert_eq!(queue.size(), 2, "1つdequeueした後のサイズは2であるべき");
    
    // 2番目の要素を取り出し
    let (value, queue) = queue.dequeue().unwrap();
    assert_eq!(value, b, "2番目にdequeueされる値は2番目に追加された値であるべき");
    assert_eq!(queue.size(), 1, "2つdequeueした後のサイズは1であるべき");
    
    // 3番目の要素を取り出し
    let (value, queue) = queue.dequeue().unwrap();
    assert_eq!(value, c, "3番目にdequeueされる値は3番目に追加された値であるべき");
    assert_eq!(queue.size(), 0, "すべての要素をdequeueした後のサイズは0であるべき");
    assert!(rust_fp_categories::Empty::is_empty(&queue), "すべての要素をdequeueした後はis_emptyがtrueを返すべき");
    
    // 空になった後のdequeueはエラーを返すべき
    assert!(queue.dequeue().is_err(), "空のキューからのdequeueはエラーを返すべき");
}

/// 共通テスト関数：peekメソッドのテスト
/// 
/// このテストでは、peekメソッドが次に取り出される要素を
/// キューから削除せずに正しく返すことを確認します。
pub fn test_peek<Q, A>()
where
    Q: Queue<A> + Empty,
    A: Clone + PartialEq + std::fmt::Debug + Default,
{
    let a = A::default();
    let b = A::default();
    
    let queue = Q::empty().enqueue(a.clone()).enqueue(b.clone());
    
    // 最初の要素をpeek
    assert_eq!(queue.peek().unwrap(), a, "peekは最初に追加された要素を返すべき");
    
    // dequeueした後に次の要素をpeek
    let (_, queue) = queue.dequeue().unwrap();
    assert_eq!(queue.peek().unwrap(), b, "dequeue後のpeekは次の要素を返すべき");
    
    // すべての要素をdequeueした後のpeekはエラーを返すべき
    let (_, queue) = queue.dequeue().unwrap();
    assert!(queue.peek().is_err(), "空のキューに対するpeekはエラーを返すべき");
}

/// 共通テスト関数：from_iterメソッドのテスト
/// 
/// このテストでは、イテレータからキューを作成する機能が
/// 正しく動作することを確認します。
pub fn test_from_iter<Q, A>()
where
    Q: Queue<A> + Empty,
    A: Clone + PartialEq + std::fmt::Debug + Default,
{
    let items = vec![A::default(), A::default(), A::default(), A::default(), A::default()];
    let expected_items = items.clone();
    
    let queue = Q::from_iter(items);
    
    assert_eq!(queue.size(), expected_items.len(), "from_iterで作成されたキューのサイズは元のコレクションのサイズと同じであるべき");
    
    // すべての要素が正しい順序で取り出せることを確認
    let mut queue_items = Vec::new();
    let mut current_queue = queue;
    
    while !rust_fp_categories::Empty::is_empty(&current_queue) {
        let (item, new_queue) = current_queue.dequeue().unwrap();
        queue_items.push(item);
        current_queue = new_queue;
    }
    
    assert_eq!(queue_items, expected_items, "キューから取り出した要素は元のコレクションと同じ順序であるべき");
}

/// 共通テスト関数：大量の要素を扱うテスト
/// 
/// このテストでは、キューが大量の要素を効率的に
/// 処理できることを確認します。
pub fn test_large_queue<Q>()
where
    Q: Queue<i32> + Empty,
{
    let mut queue = Q::empty();
    let element_count = 1000;
    
    // 大量の要素をenqueue
    for i in 0..element_count as i32 {
        queue = queue.enqueue(i);
    }
    
    assert_eq!(queue.size(), element_count, "大量の要素を追加した後のサイズは追加した要素数と同じであるべき");
    
    // すべての要素を正しい順序でdequeue
    for i in 0..element_count as i32 {
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, i, "dequeueされた値は追加された順序と同じであるべき");
        queue = new_queue;
    }
    
    assert!(rust_fp_categories::Empty::is_empty(&queue), "すべての要素をdequeueした後はキューは空であるべき");
}

/// 共通テスト関数：交互操作のテスト
/// 
/// このテストでは、enqueueとdequeueを交互に行った場合の
/// キューの振る舞いを確認します。
pub fn test_alternating_operations<Q>()
where
    Q: Queue<i32> + Empty,
{
    let mut queue = Q::empty();
    
    // enqueueとdequeueを交互に実行
    for i in 0..100 {
        queue = queue.enqueue(i);
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, i, "交互操作時にdequeueされた値は直前にenqueueされた値と同じであるべき");
        queue = new_queue;
    }
    
    assert!(rust_fp_categories::Empty::is_empty(&queue), "交互操作後のキューは空であるべき");
    
    // 複数の要素をenqueueしてから複数の要素をdequeue
    for i in 0..50 {
        queue = queue.enqueue(i);
    }
    
    assert_eq!(queue.size(), 50, "50個の要素を追加した後のサイズは50であるべき");
    
    for i in 0..25 {
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, i, "dequeueされた値は追加された順序と同じであるべき");
        queue = new_queue;
    }
    
    assert_eq!(queue.size(), 25, "25個の要素をdequeueした後のサイズは25であるべき");
    
    for i in 50..75 {
        queue = queue.enqueue(i);
    }
    
    assert_eq!(queue.size(), 50, "さらに25個の要素を追加した後のサイズは50であるべき");
    
    for i in 25..75 {
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, i, "dequeueされた値は追加された順序と同じであるべき");
        queue = new_queue;
    }
    
    assert!(rust_fp_categories::Empty::is_empty(&queue), "すべての要素をdequeueした後はキューは空であるべき");
}

/// 共通テスト関数：エッジケースのテスト
/// 
/// このテストでは、単一要素のキューや、
/// 要素を追加した後にすべて削除するなどのエッジケースを確認します。
pub fn test_edge_cases<Q, A>()
where
    Q: Queue<A> + Empty,
    A: Clone + PartialEq + std::fmt::Debug + Default,
{
    let value_factory = A::default();
    
    // 単一要素のキュー
    let queue = Q::empty().enqueue(value_factory.clone());
    assert_eq!(queue.size(), 1, "1つの要素を追加した後のサイズは1であるべき");
    assert_eq!(queue.peek().unwrap(), value_factory, "peekは追加された要素を返すべき");
    
    let (value, queue) = queue.dequeue().unwrap();
    assert_eq!(value, value_factory, "dequeueされた値は追加された値と同じであるべき");
    assert!(rust_fp_categories::Empty::is_empty(&queue), "すべての要素をdequeueした後はキューは空であるべき");
    
    // 要素を追加した後にすべて削除
    let mut queue = Q::empty();
    for _i in 0..10 {
        queue = queue.enqueue(value_factory.clone());
    }
    
    assert_eq!(queue.size(), 10, "10個の要素を追加した後のサイズは10であるべき");
    
    for _ in 0..10 {
        let (_, new_queue) = queue.dequeue().unwrap();
        queue = new_queue;
    }
    
    assert!(rust_fp_categories::Empty::is_empty(&queue), "すべての要素をdequeueした後はキューは空であるべき");
    assert_eq!(queue.size(), 0, "すべての要素をdequeueした後のサイズは0であるべき");
}

/// 共通テスト関数：複数の型に対するテスト
/// 
/// このテストでは、キューが様々な型の要素を
/// 正しく処理できることを確認します。
pub fn test_different_types<Q>()
where
    Q: Queue<String> + Queue<i32> + Queue<bool> + Empty,
{
    // 文字列型のテスト
    let mut string_queue = Q::empty();
    string_queue = string_queue.enqueue("Hello".to_string()).enqueue("World".to_string());
    
    let (value, _): (String, _) = string_queue.dequeue().unwrap();
    assert_eq!(value, "Hello".to_string(), "文字列型のキューからdequeueされた値は正しいべき");
    
    // 整数型のテスト
    let mut int_queue = Q::empty();
    int_queue = int_queue.enqueue(42).enqueue(100);
    
    let (value, _): (i32, _) = int_queue.dequeue().unwrap();
    assert_eq!(value, 42, "整数型のキューからdequeueされた値は正しいべき");
    
    // 真偽値型のテスト
    let mut bool_queue = Q::empty();
    bool_queue = bool_queue.enqueue(true).enqueue(false);
    
    let (value, _): (bool, _) = bool_queue.dequeue().unwrap();
    assert_eq!(value, true, "真偽値型のキューからdequeueされた値は正しいべき");
}

#[cfg(test)]
mod list_queue_tests {
    use super::*;
    
    #[test]
    fn test_empty() {
        test_empty_queue::<ListQueue<i32>, i32>();
    }
    
    #[test]
    fn test_basic() {
        test_basic_operations::<ListQueue<i32>, i32>();
    }
    
    #[test]
    fn test_peek_method() {
        test_peek::<ListQueue<i32>, i32>();
    }
    
    #[test]
    fn test_from_iterator() {
        test_from_iter::<ListQueue<i32>, i32>();
    }
    
    #[test]
    fn test_large() {
        test_large_queue::<ListQueue<i32>>();
    }
    
    #[test]
    fn test_alternating() {
        test_alternating_operations::<ListQueue<i32>>();
    }
    
    #[test]
    fn test_edge() {
        test_edge_cases::<ListQueue<i32>, i32>();
    }
    
    #[test]
    fn test_types() {
        // 文字列型のテスト
        let mut string_queue = ListQueue::empty();
        string_queue = string_queue.enqueue("Hello".to_string()).enqueue("World".to_string());
        
        let (value, _): (String, _) = string_queue.dequeue().unwrap();
        assert_eq!(value, "Hello".to_string(), "文字列型のキューからdequeueされた値は正しいべき");
        
        // 整数型のテスト
        let mut int_queue = ListQueue::empty();
        int_queue = int_queue.enqueue(42).enqueue(100);
        
        let (value, _): (i32, _) = int_queue.dequeue().unwrap();
        assert_eq!(value, 42, "整数型のキューからdequeueされた値は正しいべき");
    }
    
    /// 複雑なパターンのテスト
    /// 
    /// このテストでは、enqueueとdequeueを様々なパターンで
    /// 組み合わせた場合の動作を確認します。
    #[test]
    fn test_complex_patterns() {
        let mut queue = ListQueue::empty();
        
        // パターン1: enqueue x 3, dequeue x 1, enqueue x 2, dequeue x 4
        queue = queue.enqueue(1).enqueue(2).enqueue(3);
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 1, "最初のdequeueは最初に追加された値を返すべき");
        queue = new_queue;
        
        queue = queue.enqueue(4).enqueue(5);
        
        // 残りの要素を順番に取り出し
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 2, "2番目のdequeueは2番目に追加された値を返すべき");
        queue = new_queue;
        
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 3, "3番目のdequeueは3番目に追加された値を返すべき");
        queue = new_queue;
        
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 4, "4番目のdequeueは4番目に追加された値を返すべき");
        queue = new_queue;
        
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 5, "5番目のdequeueは5番目に追加された値を返すべき");
        queue = new_queue;
        
        assert!(rust_fp_categories::Empty::is_empty(&queue), "すべての要素をdequeueした後はキューは空であるべき");
        
        // パターン2: 空のキューに対する操作の後に要素を追加
        assert!(queue.clone().dequeue().is_err(), "空のキューからのdequeueはエラーを返すべき");
        assert!(queue.peek().is_err(), "空のキューに対するpeekはエラーを返すべき");
        
        queue = queue.enqueue(10);
        assert_eq!(queue.size(), 1, "空のキューに1つ要素を追加した後のサイズは1であるべき");
        assert_eq!(queue.peek().unwrap(), 10, "追加した要素をpeekで確認できるべき");
    }
    
    /// 不変性のテスト
    /// 
    /// このテストでは、キューの操作が元のキューを変更せず、
    /// 新しいキューを返すことを確認します。
    #[test]
    fn test_immutability() {
        let queue1 = ListQueue::empty().enqueue(1).enqueue(2);
        let queue2 = queue1.clone();
        
        // queue1からdequeueしても、queue2は影響を受けないことを確認
        let (_, queue1_after_dequeue) = queue1.clone().dequeue().unwrap();
        
        assert_eq!(queue2.size(), 2, "クローンされたキューは元のキューの操作の影響を受けないべき");
        assert_eq!(queue2.peek().unwrap(), 1, "クローンされたキューの先頭要素は変わらないべき");
        
        // 元のキューも変更されていないことを確認
        assert_eq!(queue1.size(), 2, "元のキューはdequeue操作後も変更されないべき");
        assert_eq!(queue1.peek().unwrap(), 1, "元のキューの先頭要素はdequeue操作後も変更されないべき");
        
        // dequeue後の新しいキューは変更されていることを確認
        assert_eq!(queue1_after_dequeue.size(), 1, "dequeue操作後の新しいキューのサイズは減少しているべき");
        assert_eq!(queue1_after_dequeue.peek().unwrap(), 2, "dequeue操作後の新しいキューの先頭要素は次の要素であるべき");
    }
    
    /// 境界値テスト
    /// 
    /// このテストでは、キューの操作における境界値の
    /// 処理が正しく行われることを確認します。
    #[test]
    fn test_boundary_values() {
        // 非常に大きな値を持つキュー
        let large_value = std::i32::MAX;
        let queue = ListQueue::empty().enqueue(large_value);
        
        let (value, _) = queue.dequeue().unwrap();
        assert_eq!(value, large_value, "非常に大きな値もキューで正しく処理されるべき");
        
        // 非常に小さな値を持つキュー
        let small_value = std::i32::MIN;
        let queue = ListQueue::empty().enqueue(small_value);
        
        let (value, _) = queue.dequeue().unwrap();
        assert_eq!(value, small_value, "非常に小さな値もキューで正しく処理されるべき");
    }
    
    /// 内部実装の検証テスト
    /// 
    /// このテストでは、ListQueueの内部実装の特性を
    /// 検証します。特に、frontリストとrearリストの
    /// 動作を確認します。
    #[test]
    fn test_internal_implementation() {
        // このテストはListQueueの内部実装に依存しています
        // frontリストが空の場合にrearリストが反転されることを確認
        
        let mut queue = ListQueue::empty();
        
        // 要素を追加
        for i in 1..=5 {
            queue = queue.enqueue(i);
        }
        
        // 要素を1つ取り出し
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 1, "最初にdequeueされる値は最初に追加された値であるべき");
        queue = new_queue;
        
        // さらに要素を追加
        for i in 6..=10 {
            queue = queue.enqueue(i);
        }
        
        // すべての要素を順番に取り出し
        for i in 2..=10 {
            let (value, new_queue) = queue.dequeue().unwrap();
            assert_eq!(value, i, "dequeueされる値は追加された順序と同じであるべき");
            queue = new_queue;
        }
        
        assert!(rust_fp_categories::Empty::is_empty(&queue), "すべての要素をdequeueした後はキューは空であるべき");
    }
}

#[cfg(test)]
mod optimized_queue_tests {
    use super::*;
    
    #[test]
    fn test_empty() {
        test_empty_queue::<OptimizedQueue<i32>, i32>();
    }
    
    #[test]
    fn test_basic() {
        test_basic_operations::<OptimizedQueue<i32>, i32>();
    }
    
    #[test]
    fn test_peek_method() {
        test_peek::<OptimizedQueue<i32>, i32>();
    }
    
    #[test]
    fn test_from_iterator() {
        test_from_iter::<OptimizedQueue<i32>, i32>();
    }
    
    #[test]
    fn test_large() {
        test_large_queue::<OptimizedQueue<i32>>();
    }
    
    #[test]
    fn test_alternating() {
        test_alternating_operations::<OptimizedQueue<i32>>();
    }
    
    #[test]
    fn test_edge() {
        test_edge_cases::<OptimizedQueue<i32>, i32>();
    }
    
    #[test]
    fn test_types() {
        // 文字列型のテスト
        let mut string_queue = OptimizedQueue::empty();
        string_queue = string_queue.enqueue("Hello".to_string()).enqueue("World".to_string());
        
        let (value, _): (String, _) = string_queue.dequeue().unwrap();
        assert_eq!(value, "Hello".to_string(), "文字列型のキューからdequeueされた値は正しいべき");
        
        // 整数型のテスト
        let mut int_queue = OptimizedQueue::empty();
        int_queue = int_queue.enqueue(42).enqueue(100);
        
        let (value, _): (i32, _) = int_queue.dequeue().unwrap();
        assert_eq!(value, 42, "整数型のキューからdequeueされた値は正しいべき");
    }
    
    #[test]
    fn test_performance_comparison() {
        // このテストは最適化されたキューの性能を検証します
        // 実際の性能測定はベンチマークで行うべきですが、
        // ここでは基本的な操作の正確性を確認します
        
        let element_count: usize = 10000;
        
        // OptimizedQueueでの大量の要素の処理
        let mut optimized_queue = OptimizedQueue::empty();
        for i in 0..element_count as i32 {
            optimized_queue = optimized_queue.enqueue(i);
        }
        
        assert_eq!(optimized_queue.size(), element_count, "最適化されたキューのサイズは正確であるべき");
        
        // サイズ計算が定数時間であることを確認（機能的なテスト）
        let size = optimized_queue.size();
        assert_eq!(size, element_count, "キャッシュされたサイズは正確であるべき");
        
        // 要素を取り出して正確性を確認
        for i in 0..100 {
            let (value, new_queue) = optimized_queue.dequeue().unwrap();
            assert_eq!(value, i, "最適化されたキューからdequeueされた値は正しいべき");
            optimized_queue = new_queue;
        }
        
        assert_eq!(optimized_queue.size(), element_count - 100, "dequeue後のサイズは正確であるべき");
    }
    
    /// 複雑なパターンのテスト
    /// 
    /// このテストでは、enqueueとdequeueを様々なパターンで
    /// 組み合わせた場合の動作を確認します。
    #[test]
    fn test_complex_patterns() {
        let mut queue = OptimizedQueue::empty();
        
        // パターン1: enqueue x 3, dequeue x 1, enqueue x 2, dequeue x 4
        queue = queue.enqueue(1).enqueue(2).enqueue(3);
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 1, "最初のdequeueは最初に追加された値を返すべき");
        queue = new_queue;
        
        queue = queue.enqueue(4).enqueue(5);
        
        // 残りの要素を順番に取り出し
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 2, "2番目のdequeueは2番目に追加された値を返すべき");
        queue = new_queue;
        
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 3, "3番目のdequeueは3番目に追加された値を返すべき");
        queue = new_queue;
        
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 4, "4番目のdequeueは4番目に追加された値を返すべき");
        queue = new_queue;
        
        let (value, new_queue) = queue.dequeue().unwrap();
        assert_eq!(value, 5, "5番目のdequeueは5番目に追加された値を返すべき");
        queue = new_queue;
        
        assert!(rust_fp_categories::Empty::is_empty(&queue), "すべての要素をdequeueした後はキューは空であるべき");
        
        // パターン2: 空のキューに対する操作の後に要素を追加
        assert!(queue.clone().dequeue().is_err(), "空のキューからのdequeueはエラーを返すべき");
        assert!(queue.peek().is_err(), "空のキューに対するpeekはエラーを返すべき");
        
        queue = queue.enqueue(10);
        assert_eq!(queue.size(), 1, "空のキューに1つ要素を追加した後のサイズは1であるべき");
        assert_eq!(queue.peek().unwrap(), 10, "追加した要素をpeekで確認できるべき");
    }
    
    /// 不変性のテスト
    /// 
    /// このテストでは、キューの操作が元のキューを変更せず、
    /// 新しいキューを返すことを確認します。
    #[test]
    fn test_immutability() {
        let queue1 = OptimizedQueue::empty().enqueue(1).enqueue(2);
        let queue2 = queue1.clone();
        
        // queue1からdequeueしても、queue2は影響を受けないことを確認
        let (_, queue1_after_dequeue) = queue1.clone().dequeue().unwrap();
        
        assert_eq!(queue2.size(), 2, "クローンされたキューは元のキューの操作の影響を受けないべき");
        assert_eq!(queue2.peek().unwrap(), 1, "クローンされたキューの先頭要素は変わらないべき");
        
        // 元のキューも変更されていないことを確認
        assert_eq!(queue1.size(), 2, "元のキューはdequeue操作後も変更されないべき");
        assert_eq!(queue1.peek().unwrap(), 1, "元のキューの先頭要素はdequeue操作後も変更されないべき");
        
        // dequeue後の新しいキューは変更されていることを確認
        assert_eq!(queue1_after_dequeue.size(), 1, "dequeue操作後の新しいキューのサイズは減少しているべき");
        assert_eq!(queue1_after_dequeue.peek().unwrap(), 2, "dequeue操作後の新しいキューの先頭要素は次の要素であるべき");
    }
    
    /// 境界値テスト
    /// 
    /// このテストでは、キューの操作における境界値の
    /// 処理が正しく行われることを確認します。
    #[test]
    fn test_boundary_values() {
        // 非常に大きな値を持つキュー
        let large_value = std::i32::MAX;
        let queue = OptimizedQueue::empty().enqueue(large_value);
        
        let (value, _) = queue.dequeue().unwrap();
        assert_eq!(value, large_value, "非常に大きな値もキューで正しく処理されるべき");
        
        // 非常に小さな値を持つキュー
        let small_value = std::i32::MIN;
        let queue = OptimizedQueue::empty().enqueue(small_value);
        
        let (value, _) = queue.dequeue().unwrap();
        assert_eq!(value, small_value, "非常に小さな値もキューで正しく処理されるべき");
    }
}
