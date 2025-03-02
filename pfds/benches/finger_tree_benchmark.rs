use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_categories::Empty;
use rust_fp_pfds::{FingerTree, SimpleFingerTree};

fn push_front_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_front");

    group.bench_function("SimpleFingerTree", |b| {
        b.iter(|| {
            let mut tree = SimpleFingerTree::<i32>::new();
            for i in 0..100 {
                tree = tree.push_front(black_box(i));
            }
        })
    });

    group.finish();
}

fn push_back_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_back");

    group.bench_function("SimpleFingerTree", |b| {
        b.iter(|| {
            let mut tree = SimpleFingerTree::<i32>::new();
            for i in 0..100 {
                tree = tree.push_back(black_box(i));
            }
        })
    });

    group.finish();
}

fn pop_front_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_front");

    group.bench_function("SimpleFingerTree", |b| {
        b.iter(|| {
            let mut tree = SimpleFingerTree::<i32>::new();
            for i in 0..100 {
                tree = tree.push_back(i);
            }

            let mut result = 0;
            while let Ok((value, new_tree)) = tree.pop_front() {
                result += value;
                tree = new_tree;
            }
            black_box(result);
        })
    });

    group.finish();
}

fn pop_back_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_back");

    group.bench_function("SimpleFingerTree", |b| {
        b.iter(|| {
            let mut tree = SimpleFingerTree::<i32>::new();
            for i in 0..100 {
                tree = tree.push_front(i);
            }

            let mut result = 0;
            while let Ok((value, new_tree)) = tree.pop_back() {
                result += value;
                tree = new_tree;
            }
            black_box(result);
        })
    });

    group.finish();
}

fn concat_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("concat");

    group.bench_function("SimpleFingerTree", |b| {
        b.iter(|| {
            let mut tree1 = SimpleFingerTree::<i32>::new();
            let mut tree2 = SimpleFingerTree::<i32>::new();

            for i in 0..50 {
                tree1 = tree1.push_back(i);
                tree2 = tree2.push_back(i + 50);
            }

            let result = tree1.concat(tree2);
            black_box(result);
        })
    });

    group.finish();
}

fn split_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("split");

    group.bench_function("SimpleFingerTree", |b| {
        b.iter(|| {
            let mut tree = SimpleFingerTree::<i32>::new();

            for i in 0..100 {
                tree = tree.push_back(i);
            }

            let (left, right) = tree.split(50);
            black_box((left, right));
        })
    });

    group.finish();
}

fn mixed_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_operations");

    group.bench_function("SimpleFingerTree", |b| {
        b.iter(|| {
            let mut tree = SimpleFingerTree::<i32>::new();

            // Push from both ends
            for i in 0..25 {
                tree = tree.push_front(i);
                tree = tree.push_back(i + 25);
            }

            // Concat with another tree
            let mut other_tree = SimpleFingerTree::<i32>::new();
            for i in 0..10 {
                other_tree = other_tree.push_back(i + 50);
            }

            tree = tree.concat(other_tree);

            // Split and recombine
            let (left, right) = tree.split(30);
            tree = left.concat(right);

            // Pop from both ends
            let mut result = 0;
            for _ in 0..5 {
                if let Ok((value, new_tree)) = tree.clone().pop_front() {
                    result += value;
                    tree = new_tree;
                }

                if let Ok((value, new_tree)) = tree.clone().pop_back() {
                    result += value;
                    tree = new_tree;
                }
            }

            black_box(result);
        })
    });

    group.finish();
}

fn from_iter_benchmark(c: &mut Criterion) {
    let vec = (0..100).collect::<Vec<i32>>();

    let mut group = c.benchmark_group("from_iter");

    group.bench_function("SimpleFingerTree", |b| {
        b.iter(|| {
            let tree = SimpleFingerTree::<i32>::from_iter(black_box(vec.clone()));
            black_box(tree);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    push_front_benchmark,
    push_back_benchmark,
    pop_front_benchmark,
    pop_back_benchmark,
    concat_benchmark,
    split_benchmark,
    mixed_operations_benchmark,
    from_iter_benchmark
);
criterion_main!(benches);
