use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_categories::{common, common_opt, Functor};

fn vec_fmap_benchmark(c: &mut Criterion) {
    let vec = (0..1000).collect::<Vec<i32>>();
    
    c.bench_function("Vec::fmap_original", |b| {
        b.iter(|| {
            let _result = black_box(vec.clone()).fmap(|x| x * 2);
        })
    });
    
    c.bench_function("Vec::fmap_optimized", |b| {
        b.iter(|| {
            let _result = common_opt::vec::fmap(black_box(vec.clone()), |x| x * 2);
        })
    });
}

fn vec_ap_benchmark(c: &mut Criterion) {
    let vec = (0..1000).collect::<Vec<i32>>();
    let fs = (0..1000).map(|_| |x: &i32| x * 2).collect::<Vec<_>>();
    
    c.bench_function("Vec::ap_original", |b| {
        b.iter(|| {
            let _result = common::vec::ap(black_box(vec.clone()), black_box(fs.clone()));
        })
    });
    
    c.bench_function("Vec::ap_optimized", |b| {
        b.iter(|| {
            let _result = common_opt::vec::ap(black_box(vec.clone()), black_box(fs.clone()));
        })
    });
}

fn vec_bind_benchmark(c: &mut Criterion) {
    let vec = (0..100).collect::<Vec<i32>>();
    
    c.bench_function("Vec::bind_original", |b| {
        b.iter(|| {
            let _result = common::vec::bind(black_box(vec.clone()), |x| vec![x * 2, x * 3]);
        })
    });
    
    c.bench_function("Vec::bind_optimized", |b| {
        b.iter(|| {
            let _result = common_opt::vec::bind(black_box(vec.clone()), |x| vec![x * 2, x * 3]);
        })
    });
}

criterion_group!(benches, vec_fmap_benchmark, vec_ap_benchmark, vec_bind_benchmark);
criterion_main!(benches);
