use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_categories::Empty;
use rust_fp_pfds::{ListOptimized, ListOptimizedV2, Set, TreeOptimized};

// This benchmark file is designed to compare our optimized implementations
// with competing libraries. Since we want to minimize external dependencies,
// this file contains placeholders for the comparisons.
//
// To enable actual comparisons, uncomment the following lines in Cargo.toml:
// [dev-dependencies]
// im = "15.1.0"
// rpds = "0.12.0"
//
// And then uncomment the relevant code in this file.

fn list_from_vec_benchmark(c: &mut Criterion) {
    let vec = (0..1000).collect::<Vec<i32>>();

    let mut group = c.benchmark_group("list_from_vec_comparison");

    group.bench_function("ListOptimized::from_vec", |b| {
        b.iter(|| {
            let _list: ListOptimized<i32> = ListOptimized::from(black_box(vec.clone()));
        })
    });

    group.bench_function("ListOptimizedV2::from_vec", |b| {
        b.iter(|| {
            let _list: ListOptimizedV2<i32> = ListOptimizedV2::from(black_box(vec.clone()));
        })
    });

    // Placeholder for im::Vector comparison
    // Uncomment when im crate is added as a dev-dependency
    /*
    group.bench_function("im::Vector::from_vec", |b| {
        b.iter(|| {
            let _vec = im::Vector::from(black_box(vec.clone()));
        })
    });
    */

    // Placeholder for rpds::List comparison
    // Uncomment when rpds crate is added as a dev-dependency
    /*
    group.bench_function("rpds::List::from_vec", |b| {
        b.iter(|| {
            let mut list = rpds::List::new();
            for i in black_box(vec.clone()).into_iter().rev() {
                list = list.push_front(i);
            }
            black_box(list);
        })
    });
    */

    group.finish();
}

fn list_into_vec_benchmark(c: &mut Criterion) {
    let list_opt: ListOptimized<i32> = ListOptimized::from((0..1000).collect::<Vec<i32>>());
    let list_opt_v2: ListOptimizedV2<i32> = ListOptimizedV2::from((0..1000).collect::<Vec<i32>>());

    let mut group = c.benchmark_group("list_into_vec_comparison");

    group.bench_function("ListOptimized::into_vec", |b| {
        b.iter(|| {
            let _vec: Vec<i32> = black_box(list_opt.clone()).into();
        })
    });

    group.bench_function("ListOptimizedV2::into_vec", |b| {
        b.iter(|| {
            let _vec: Vec<i32> = black_box(list_opt_v2.clone()).into();
        })
    });

    // Placeholder for im::Vector comparison
    // Uncomment when im crate is added as a dev-dependency
    /*
    let im_vec = im::Vector::from((0..1000).collect::<Vec<i32>>());
    group.bench_function("im::Vector::into_vec", |b| {
        b.iter(|| {
            let _vec: Vec<i32> = black_box(im_vec.clone()).into();
        })
    });
    */

    // Placeholder for rpds::List comparison
    // Uncomment when rpds crate is added as a dev-dependency
    /*
    let mut rpds_list = rpds::List::new();
    for i in (0..1000).rev() {
        rpds_list = rpds_list.push_front(i);
    }
    group.bench_function("rpds::List::into_vec", |b| {
        b.iter(|| {
            let _vec: Vec<i32> = black_box(rpds_list.clone()).iter().cloned().collect();
        })
    });
    */

    group.finish();
}

fn tree_insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_insert_comparison");

    group.bench_function("TreeOptimized::insert", |b| {
        b.iter(|| {
            let mut tree = TreeOptimized::<i32>::empty();
            for i in 0..100 {
                tree = tree.insert(black_box(i));
            }
            black_box(tree);
        })
    });

    // Placeholder for im::OrdSet comparison
    // Uncomment when im crate is added as a dev-dependency
    /*
    group.bench_function("im::OrdSet::insert", |b| {
        b.iter(|| {
            let mut set = im::OrdSet::new();
            for i in 0..100 {
                set = set.insert(black_box(i));
            }
            black_box(set);
        })
    });
    */

    group.finish();
}

fn tree_member_benchmark(c: &mut Criterion) {
    // Create a tree with 100 elements
    let mut tree_opt = TreeOptimized::<i32>::empty();
    for i in 0..100 {
        tree_opt = tree_opt.insert(i);
    }

    let mut group = c.benchmark_group("tree_member_comparison");

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

    // Placeholder for im::OrdSet comparison
    // Uncomment when im crate is added as a dev-dependency
    /*
    let mut im_set = im::OrdSet::new();
    for i in 0..100 {
        im_set = im_set.insert(i);
    }
    group.bench_function("im::OrdSet::contains", |b| {
        b.iter(|| {
            let mut result = 0;
            for i in 0..100 {
                if im_set.contains(&black_box(i)) {
                    result += 1;
                }
            }
            black_box(result);
        })
    });
    */

    group.finish();
}

criterion_group!(
    benches,
    list_from_vec_benchmark,
    list_into_vec_benchmark,
    tree_insert_benchmark,
    tree_member_benchmark
);
criterion_main!(benches);
