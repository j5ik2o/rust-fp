use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_categories::Empty;
use rust_fp_pfds::{BTreeSet, HashSet, Set};

fn insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_insert");

    group.bench_function("BTreeSet", |b| {
        b.iter(|| {
            let mut set = BTreeSet::<i32>::empty();
            for i in 0..100 {
                set = set.insert(black_box(i));
            }
            black_box(set);
        })
    });

    group.bench_function("HashSet", |b| {
        b.iter(|| {
            let mut set = HashSet::<i32>::empty();
            for i in 0..100 {
                set = set.insert(black_box(i));
            }
            black_box(set);
        })
    });

    group.finish();
}

fn member_benchmark(c: &mut Criterion) {
    // Create sets with 100 elements
    let mut btree_set = BTreeSet::<i32>::empty();
    let mut hash_set = HashSet::<i32>::empty();

    for i in 0..100 {
        btree_set = btree_set.insert(i);
        hash_set = hash_set.insert(i);
    }

    let mut group = c.benchmark_group("set_member");

    // Benchmark checking membership for existing elements
    group.bench_function("BTreeSet::member_existing", |b| {
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

    group.bench_function("HashSet::member_existing", |b| {
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

    // Benchmark checking membership for non-existing elements
    group.bench_function("BTreeSet::member_nonexisting", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 100..200 {
                if btree_set.member(black_box(i)) {
                    result += 1;
                }
            }
            black_box(result);
        })
    });

    group.bench_function("HashSet::member_nonexisting", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 100..200 {
                if hash_set.member(black_box(i)) {
                    result += 1;
                }
            }
            black_box(result);
        })
    });

    group.finish();
}

fn union_benchmark(c: &mut Criterion) {
    // Create two sets with different elements
    let mut btree_set1 = BTreeSet::<i32>::empty();
    let mut btree_set2 = BTreeSet::<i32>::empty();
    let mut hash_set1 = HashSet::<i32>::empty();
    let mut hash_set2 = HashSet::<i32>::empty();

    for i in 0..50 {
        btree_set1 = btree_set1.insert(i);
        hash_set1 = hash_set1.insert(i);
    }

    for i in 25..75 {
        btree_set2 = btree_set2.insert(i);
        hash_set2 = hash_set2.insert(i);
    }

    let mut group = c.benchmark_group("set_union");

    group.bench_function("BTreeSet::union", |b| {
        b.iter(|| {
            let result = btree_set1.clone().union(btree_set2.clone());
            black_box(result);
        })
    });

    group.bench_function("HashSet::union", |b| {
        b.iter(|| {
            let result = hash_set1.clone().union(hash_set2.clone());
            black_box(result);
        })
    });

    group.finish();
}

fn intersection_benchmark(c: &mut Criterion) {
    // Create two sets with overlapping elements
    let mut btree_set1 = BTreeSet::<i32>::empty();
    let mut btree_set2 = BTreeSet::<i32>::empty();
    let mut hash_set1 = HashSet::<i32>::empty();
    let mut hash_set2 = HashSet::<i32>::empty();

    for i in 0..50 {
        btree_set1 = btree_set1.insert(i);
        hash_set1 = hash_set1.insert(i);
    }

    for i in 25..75 {
        btree_set2 = btree_set2.insert(i);
        hash_set2 = hash_set2.insert(i);
    }

    let mut group = c.benchmark_group("set_intersection");

    group.bench_function("BTreeSet::intersection", |b| {
        b.iter(|| {
            let result = btree_set1.clone().intersection(btree_set2.clone());
            black_box(result);
        })
    });

    group.bench_function("HashSet::intersection", |b| {
        b.iter(|| {
            let result = hash_set1.clone().intersection(hash_set2.clone());
            black_box(result);
        })
    });

    group.finish();
}

fn difference_benchmark(c: &mut Criterion) {
    // Create two sets with overlapping elements
    let mut btree_set1 = BTreeSet::<i32>::empty();
    let mut btree_set2 = BTreeSet::<i32>::empty();
    let mut hash_set1 = HashSet::<i32>::empty();
    let mut hash_set2 = HashSet::<i32>::empty();

    for i in 0..50 {
        btree_set1 = btree_set1.insert(i);
        hash_set1 = hash_set1.insert(i);
    }

    for i in 25..75 {
        btree_set2 = btree_set2.insert(i);
        hash_set2 = hash_set2.insert(i);
    }

    let mut group = c.benchmark_group("set_difference");

    group.bench_function("BTreeSet::difference", |b| {
        b.iter(|| {
            let result = btree_set1.clone().difference(btree_set2.clone());
            black_box(result);
        })
    });

    group.bench_function("HashSet::difference", |b| {
        b.iter(|| {
            let result = hash_set1.clone().difference(hash_set2.clone());
            black_box(result);
        })
    });

    group.finish();
}

fn is_subset_of_benchmark(c: &mut Criterion) {
    // Create a subset and superset relationship
    let mut btree_subset = BTreeSet::<i32>::empty();
    let mut btree_superset = BTreeSet::<i32>::empty();
    let mut hash_subset = HashSet::<i32>::empty();
    let mut hash_superset = HashSet::<i32>::empty();

    for i in 0..25 {
        btree_subset = btree_subset.insert(i);
        hash_subset = hash_subset.insert(i);
    }

    for i in 0..50 {
        btree_superset = btree_superset.insert(i);
        hash_superset = hash_superset.insert(i);
    }

    let mut group = c.benchmark_group("set_is_subset_of");

    group.bench_function("BTreeSet::is_subset_of", |b| {
        b.iter(|| {
            let result = btree_subset.is_subset_of(&btree_superset);
            black_box(result);
        })
    });

    group.bench_function("HashSet::is_subset_of", |b| {
        b.iter(|| {
            let result = hash_subset.is_subset_of(&hash_superset);
            black_box(result);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    insert_benchmark,
    member_benchmark,
    union_benchmark,
    intersection_benchmark,
    difference_benchmark,
    is_subset_of_benchmark
);
criterion_main!(benches);
