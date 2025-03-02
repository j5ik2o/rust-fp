# rust-fp

rust-fp is a library for functional programming in Rust.

## Install

```toml
[dependencies]
rust-fp-categories = "0.0.1"
rust-fp-pfds = "0.0.1"
```

## Type-classes for categories

|type-class|j5ik2o/rust-fp|[JasonShin/fp-core.rs](https://github.com/JasonShin/fp-core.rs)|[kitfre/Kinder](https://github.com/kitfre/Kinder)|[14427/hkt.rs](https://gist.github.com/14427/af90a21b917d2892eace)|[aoprisan/func](https://github.com/aoprisan/func)|
|:-----------------------|:------|:------|:------|:------|:------|
|Functor                 |✓|✓|✓|✓|-|
|Pure                    |✓|✓|-|-|-|
|Apply                   |✓|✓|-|-|-|
|Applicativie(Pure+Apply)|✓|✓|✓|✓|-|
|Bind                    |✓|✓|-|-|-|
|Monad(Applicative+Bind) |✓|✓|✓|✓|-|
|Empty                   |✓|✓|-|-|-|
|Semigroup               |✓|✓|-|-|-|
|Monoid(Empty+Semigroup) |✓|✓|✓|✓|-|
|Foldable                |✓|✓|✓|-|-|
|Show                    |✓|-|-|-|✓|
|HList                   |✓|-|-|-|✓|
|TailRec                 |-|-|-|-|✓|


## Purely functional data structures

|data name|j5ik2o/rust-fp|[aoprisan/func](https://github.com/aoprisan/func)|
|:---------|:------|:------|
|Stack|✓|-|
|Set|✓|-|
|Lazy|-|✓|
|IO|-|✓|
|Free|-|✓|
|Computation|-|✓|
