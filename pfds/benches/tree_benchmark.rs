use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_categories::Empty;
use rust_fp_pfds::{Set, Tree, TreeOptimized};

fn insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_insert");

    // Benchmark inserting elements into an empty Tree
    group.bench_function("Tree::insert", |b| {
        b.iter(|| {
            let mut tree = Tree::<i32>::empty();
            for i in 0..100 {
                tree = tree.insert(black_box(i));
            }
            black_box(tree);
        })
    });

    // Benchmark inserting elements into an empty TreeOptimized
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

fn member_benchmark(c: &mut Criterion) {
    // Create trees with 100 elements
    let mut tree = Tree::<i32>::empty();
    let mut tree_opt = TreeOptimized::<i32>::empty();
    for i in 0..100 {
        tree = tree.insert(i);
        tree_opt = tree_opt.insert(i);
    }

    let mut group = c.benchmark_group("tree_member");

    // Benchmark checking membership for existing elements
    group.bench_function("Tree::member_existing", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 0..100 {
                if tree.member(black_box(i)) {
                    result += 1;
                }
            }
            black_box(result);
        })
    });

    group.bench_function("TreeOptimized::member_existing", |b| {
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

    // Benchmark checking membership for non-existing elements
    group.bench_function("Tree::member_nonexisting", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 100..200 {
                if tree.member(black_box(i)) {
                    result += 1;
                }
            }
            black_box(result);
        })
    });

    group.bench_function("TreeOptimized::member_nonexisting", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 100..200 {
                if tree_opt.member(black_box(i)) {
                    result += 1;
                }
            }
            black_box(result);
        })
    });

    group.finish();
}

fn union_benchmark(c: &mut Criterion) {
    // Create two sets of trees with different elements
    let mut tree1 = Tree::<i32>::empty();
    let mut tree2 = Tree::<i32>::empty();
    let mut tree_opt1 = TreeOptimized::<i32>::empty();
    let mut tree_opt2 = TreeOptimized::<i32>::empty();

    for i in 0..50 {
        tree1 = tree1.insert(i);
        tree_opt1 = tree_opt1.insert(i);
    }

    for i in 25..75 {
        tree2 = tree2.insert(i);
        tree_opt2 = tree_opt2.insert(i);
    }

    let mut group = c.benchmark_group("tree_union");

    group.bench_function("Tree::union", |b| {
        b.iter(|| {
            let result = tree1.clone().union(tree2.clone());
            black_box(result);
        })
    });

    group.bench_function("TreeOptimized::union", |b| {
        b.iter(|| {
            let result = tree_opt1.clone().union(tree_opt2.clone());
            black_box(result);
        })
    });

    group.finish();
}

fn intersection_benchmark(c: &mut Criterion) {
    // Create two sets of trees with overlapping elements
    let mut tree1 = Tree::<i32>::empty();
    let mut tree2 = Tree::<i32>::empty();
    let mut tree_opt1 = TreeOptimized::<i32>::empty();
    let mut tree_opt2 = TreeOptimized::<i32>::empty();

    for i in 0..50 {
        tree1 = tree1.insert(i);
        tree_opt1 = tree_opt1.insert(i);
    }

    for i in 25..75 {
        tree2 = tree2.insert(i);
        tree_opt2 = tree_opt2.insert(i);
    }

    let mut group = c.benchmark_group("tree_intersection");

    group.bench_function("Tree::intersection", |b| {
        b.iter(|| {
            let result = tree1.clone().intersection(tree2.clone());
            black_box(result);
        })
    });

    group.bench_function("TreeOptimized::intersection", |b| {
        b.iter(|| {
            let result = tree_opt1.clone().intersection(tree_opt2.clone());
            black_box(result);
        })
    });

    group.finish();
}

fn difference_benchmark(c: &mut Criterion) {
    // Create two sets of trees with overlapping elements
    let mut tree1 = Tree::<i32>::empty();
    let mut tree2 = Tree::<i32>::empty();
    let mut tree_opt1 = TreeOptimized::<i32>::empty();
    let mut tree_opt2 = TreeOptimized::<i32>::empty();

    for i in 0..50 {
        tree1 = tree1.insert(i);
        tree_opt1 = tree_opt1.insert(i);
    }

    for i in 25..75 {
        tree2 = tree2.insert(i);
        tree_opt2 = tree_opt2.insert(i);
    }

    let mut group = c.benchmark_group("tree_difference");

    group.bench_function("Tree::difference", |b| {
        b.iter(|| {
            let result = tree1.clone().difference(tree2.clone());
            black_box(result);
        })
    });

    group.bench_function("TreeOptimized::difference", |b| {
        b.iter(|| {
            let result = tree_opt1.clone().difference(tree_opt2.clone());
            black_box(result);
        })
    });

    group.finish();
}

fn is_subset_of_benchmark(c: &mut Criterion) {
    // Create a subset and superset relationship
    let mut subset = Tree::<i32>::empty();
    let mut superset = Tree::<i32>::empty();
    let mut subset_opt = TreeOptimized::<i32>::empty();
    let mut superset_opt = TreeOptimized::<i32>::empty();

    for i in 0..25 {
        subset = subset.insert(i);
        subset_opt = subset_opt.insert(i);
    }

    for i in 0..50 {
        superset = superset.insert(i);
        superset_opt = superset_opt.insert(i);
    }

    let mut group = c.benchmark_group("tree_is_subset_of");

    group.bench_function("Tree::is_subset_of", |b| {
        b.iter(|| {
            let result = subset.is_subset_of(&superset);
            black_box(result);
        })
    });

    group.bench_function("TreeOptimized::is_subset_of", |b| {
        b.iter(|| {
            let result = subset_opt.is_subset_of(&superset_opt);
            black_box(result);
        })
    });

    group.finish();
}

fn compare_with_im_benchmark(c: &mut Criterion) {
    // This benchmark would compare with the im crate's implementation
    // However, since we're not adding external dependencies, we'll just leave this as a placeholder
    // In a real implementation, we would add the im crate as a dev-dependency and benchmark against it
    let mut group = c.benchmark_group("compare_with_im");

    // Placeholder for comparison with im crate
    group.bench_function("TreeOptimized vs im::OrdSet (placeholder)", |b| {
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

criterion_group!(
    benches,
    insert_benchmark,
    member_benchmark,
    union_benchmark,
    intersection_benchmark,
    difference_benchmark,
    is_subset_of_benchmark,
    compare_with_im_benchmark
);
criterion_main!(benches);
