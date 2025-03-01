# 型クラス階層図

```
                  Functor
                    |
                    v
                   Apply
                  /    \
                 v      v
               Pure    Bind
                \      /
                 v    v
              Applicative
                    |
                    v
                  Monad
```

## 型クラス間の関係

### Functor
- 基本的な型クラス
- `fmap`メソッドを提供
- コンテナ内の値を変換する機能

### Apply
- Functorを拡張
- `ap`メソッドを提供
- 関数を含むコンテナを適用する機能

### Pure
- `pure`と`unit`メソッドを提供
- 値をコンテナにリフトする機能

### Bind
- Functorを拡張
- `bind`メソッドを提供
- モナド的な連鎖操作を可能にする

### Applicative
- ApplyとPureを組み合わせた型クラス
- 関数適用と値のリフトを組み合わせる

### Monad
- ApplicativeとBindを組み合わせた型クラス
- 最も表現力の高い型クラス
