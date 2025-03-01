use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_pfds::{List, ListOptimized};

fn from_vec_benchmark(c: &mut Criterion) {
    let vec = (0..1000).collect::<Vec<i32>>();

    c.bench_function("List::from_vec", |b| {
        b.iter(|| {
            let _list: List<i32> = List::from(black_box(vec.clone()));
        })
    });

    c.bench_function("ListOptimized::from_vec", |b| {
        b.iter(|| {
            let _list: ListOptimized<i32> = ListOptimized::from(black_box(vec.clone()));
        })
    });
}

fn into_vec_benchmark(c: &mut Criterion) {
    let list: List<i32> = List::from((0..1000).collect::<Vec<i32>>());
    let list_opt: ListOptimized<i32> = ListOptimized::from((0..1000).collect::<Vec<i32>>());

    c.bench_function("List::into_vec", |b| {
        b.iter(|| {
            let _vec: Vec<i32> = black_box(list.clone()).into();
        })
    });

    c.bench_function("ListOptimized::into_vec", |b| {
        b.iter(|| {
            let _vec: Vec<i32> = black_box(list_opt.clone()).into();
        })
    });
}

fn reverse_benchmark(c: &mut Criterion) {
    let list: List<i32> = List::from((0..1000).collect::<Vec<i32>>());
    let list_opt: ListOptimized<i32> = ListOptimized::from((0..1000).collect::<Vec<i32>>());

    c.bench_function("List::reverse", |b| {
        b.iter(|| {
            let _reversed = black_box(list.clone()).reverse();
        })
    });

    c.bench_function("ListOptimized::reverse", |b| {
        b.iter(|| {
            let _reversed = black_box(list_opt.clone()).reverse();
        })
    });
}

criterion_group!(
    benches,
    from_vec_benchmark,
    into_vec_benchmark,
    reverse_benchmark
);
criterion_main!(benches);
