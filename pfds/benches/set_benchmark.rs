use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_categories::Empty;
use rust_fp_pfds::{BTreeSet, HashSet, Set, TreeOptimized};

fn set_insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_insert");

    group.bench_function("BTreeSet::insert", |b| {
        b.iter(|| {
            let mut set = BTreeSet::<i32>::empty();
            for i in 0..100 {
                set = set.insert(black_box(i));
            }
            black_box(set);
        })
    });

    group.bench_function("HashSet::insert", |b| {
        b.iter(|| {
            let mut set = HashSet::<i32>::empty();
            for i in 0..100 {
                set = set.insert(black_box(i));
            }
            black_box(set);
        })
    });

    group.bench_function("TreeOptimized::insert", |b| {
        b.iter(|| {
            let mut tree = TreeOptimized::<i32>::empty();
            for i in 0..100 {
                tree = tree.insert(black_box(i));
            }
            black_box(tree);
        })
    });

    group.finish();
}

fn set_member_benchmark(c: &mut Criterion) {
    // Create sets with 100 elements
    let mut btree_set = BTreeSet::<i32>::empty();
    let mut hash_set = HashSet::<i32>::empty();
    let mut tree_opt = TreeOptimized::<i32>::empty();

    for i in 0..100 {
        btree_set = btree_set.insert(i);
        hash_set = hash_set.insert(i);
        tree_opt = tree_opt.insert(i);
    }

    let mut group = c.benchmark_group("set_member");

    group.bench_function("BTreeSet::member", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 0..100 {
                if btree_set.member(black_box(i)) {
                    result += 1;
                }
            }
            black_box(result);
        })
    });

    group.bench_function("HashSet::member", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 0..100 {
                if hash_set.member(black_box(i)) {
                    result += 1;
                }
            }
            black_box(result);
        })
    });

    group.bench_function("TreeOptimized::member", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 0..100 {
                if tree_opt.member(black_box(i)) {
                    result += 1;
                }
            }
            black_box(result);
        })
    });

    group.finish();
}

criterion_group!(benches, set_insert_benchmark, set_member_benchmark);
criterion_main!(benches);
