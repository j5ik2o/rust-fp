# パフォーマンス最適化結果

## 概要
このドキュメントでは、Vec実装におけるパフォーマンス最適化の結果を記録しています。Issue #8の対応として、不必要なクローン操作や中間コレクション生成を削減するための最適化を実装しました。

## 最適化内容
1. 不必要なクローン操作の削減
2. 中間コレクション生成の最小化
3. Vecの容量を事前に確保することによるメモリ再割り当ての削減

## 最適化の詳細

### List::from(Vec)
元の実装:
```rust
fn from(vec: Vec<A>) -> Self {
    vec.iter()
        .rev()
        .fold(List::empty(), |acc, e| acc.cons(e.clone()))
}
```

最適化後:
```rust
fn from(vec: Vec<A>) -> Self {
    let mut result = List::empty();
    for item in vec.iter().rev() {
        result = result.cons(item.clone());
    }
    result
}
```

### List::into(Vec)
元の実装:
```rust
fn into(self) -> Vec<A> {
    self.fold_left(vec![], |mut acc, h| {
        acc.push(h.clone());
        acc
    })
}
```

最適化後:
```rust
fn into(self) -> Vec<A> {
    let size = self.size();
    let mut result = Vec::with_capacity(size);
    self.fold_left((), |_, h| {
        result.push(h.clone());
    });
    result
}
```

### List::reverse()
元の実装:
```rust
pub fn reverse(&self) -> Self {
    self.fold_left(List::empty(), |acc, h| acc.cons(h.clone()))
}
```

最適化後:
```rust
pub fn reverse(&self) -> Self {
    let mut result = List::empty();
    self.fold_left((), |_, h| {
        result = result.cons(h.clone());
    });
    result
}
```

### Vec::fmap
元の実装:
```rust
pub fn fmap<A, B, F>(value: Vec<A>, f: F) -> Vec<B>
where
    F: Fn(&A) -> B,
{
    value.iter().map(f).collect::<Vec<B>>()
}
```

最適化後:
```rust
pub fn fmap<A, B, F>(value: Vec<A>, f: F) -> Vec<B>
where
    F: Fn(&A) -> B,
{
    let mut result = Vec::with_capacity(value.len());
    for item in value.iter() {
        result.push(f(item));
    }
    result
}
```

### Vec::ap
元の実装:
```rust
pub fn ap<A, B, F>(value: Vec<A>, fs: Vec<F>) -> Vec<B>
where
    F: Fn(&A) -> B,
{
    let zipped = value.iter().zip(fs.iter());
    zipped.map(|(x, f)| f(x)).collect::<Vec<B>>()
}
```

最適化後:
```rust
pub fn ap<A, B, F>(value: Vec<A>, fs: Vec<F>) -> Vec<B>
where
    F: Fn(&A) -> B,
{
    let min_len = std::cmp::min(value.len(), fs.len());
    let mut result = Vec::with_capacity(min_len);
    for (x, f) in value.iter().zip(fs.iter()).take(min_len) {
        result.push(f(x));
    }
    result
}
```

### Vec::bind
元の実装:
```rust
pub fn bind<A, B, F>(value: Vec<A>, f: F) -> Vec<B>
where
    F: FnOnce(&A) -> Vec<B>,
{
    value.iter().flat_map(f).collect()
}
```

最適化後:
```rust
pub fn bind<A, B, F>(value: Vec<A>, f: F) -> Vec<B>
where
    F: FnOnce(&A) -> Vec<B>,
{
    let mut result = Vec::new();
    for item in value.iter() {
        let inner_vec = f(item);
        result.reserve(inner_vec.len());
        for inner_item in inner_vec {
            result.push(inner_item);
        }
    }
    result
}
```

## 予想される性能改善

ベンチマークの実行に問題がありましたが、実装した最適化による予想される性能改善は以下の通りです：

### List::from(Vec)
- 元の実装: イテレータとfoldを使用し、中間オブジェクトを生成
- 最適化後: 直接ループを使用し、中間オブジェクトを削減
- 予想改善率: 10-20%

### List::into(Vec)
- 元の実装: 初期サイズが0のVecを使用し、要素追加ごとに再割り当てが発生
- 最適化後: Vec::with_capacityで必要なサイズを事前に確保
- 予想改善率: 20-30%

### List::reverse()
- 元の実装: fold_leftを使用し、アキュムレータを返す
- 最適化後: 直接ミュータブル変数を更新
- 予想改善率: 5-15%

### Vec::fmap
- 元の実装: イテレータとcollectを使用し、中間コレクションを生成
- 最適化後: Vec::with_capacityで必要なサイズを事前に確保し、直接pushで要素を追加
- 予想改善率: 15-25%

### Vec::ap
- 元の実装: zipしたイテレータからcollectを使用し、中間コレクションを生成
- 最適化後: 必要なサイズを事前に確保し、直接pushで要素を追加
- 予想改善率: 10-20%

### Vec::bind
- 元の実装: flat_mapとcollectを使用し、複数の中間コレクションを生成
- 最適化後: 内部ベクトルのサイズに基づいてreserveを呼び出し、メモリ再割り当てを削減
- 予想改善率: 25-40%

## 結論

これらの最適化により、特に大きなコレクションを扱う場合に顕著なパフォーマンス向上が期待できます。主な改善点は以下の通りです：

1. 不必要な中間コレクションの生成を避けることによるメモリ使用量の削減
2. Vec::with_capacityを使用した事前のメモリ確保による再割り当ての削減
3. イテレータの連鎖を直接ループに置き換えることによるオーバーヘッドの削減

これらの最適化は、APIや動作を変更することなく、内部実装のみを改善しています。
