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
|ForYield                |✓|✓|✓|-|-|
|TailRec                 |✓|-|-|-|✓|


## Purely functional data structures

|data name|j5ik2o/rust-fp|[aoprisan/func](https://github.com/aoprisan/func)|
|:---------|:------|:------|
|Stack|✓|-|
|Set|✓|-|
|Lazy|-|✓|
|IO|-|✓|
|Free|-|✓|
|Computation|-|✓|

## Performance Benchmarks

Below are benchmark results comparing rust-fp data structures with competing libraries.

### Queue Operations

| Implementation | enqueue (µs) | dequeue (µs) |
|----------------|--------------|--------------|
| ArrayQueue     | 7.07         | 3.33         |
| ListQueue      | 4.09         | 2.96         |
| OptimizedQueue | 4.77         | 2.89         |
| im::Vector     | 0.76         | 0.82         |
| rpds::Queue    | 5.09         | -            |

### Stack Operations

| Implementation   | push (ns) | pop (µs) | peek (ns) |
|------------------|-----------|----------|-----------|
| ArrayStack       | 274.52    | 1.76     | -         |
| PersistentStack  | 2345.0    | 0.15     | -         |
| im::Vector       | 744.53    | 0.63     | -         |
| rpds::Stack      | 5091.4    | 1.82     | -         |

### Set Operations

| Implementation      | insert (µs) | member (µs) |
|---------------------|-------------|-------------|
| BTreeSet            | 190.53      | 8.06        |
| HashSet             | 2.34        | 2.74        |
| TreeOptimized       | 129.28      | 16.65       |
| im::OrdSet          | 2.42        | 0.83        |
| std::BTreeSet       | 2.61        | 0.82        |
| std::HashSet        | 2.89        | 1.10        |

*Lower values indicate better performance. Benchmarks run on a standard development machine.*
