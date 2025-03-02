use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_pfds::{ArrayStack, PersistentStack, Stack};

fn stack_cons_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_cons");

    group.bench_function("ArrayStack::cons", |b| {
        b.iter(|| {
            let mut stack = ArrayStack::<i32>::new();
            for i in 0..100 {
                stack = stack.cons(black_box(i));
            }
            black_box(stack);
        })
    });

    group.bench_function("PersistentStack::cons", |b| {
        b.iter(|| {
            let mut stack = PersistentStack::<i32>::new();
            for i in 0..100 {
                stack = stack.cons(black_box(i));
            }
            black_box(stack);
        })
    });

    group.finish();
}

fn stack_uncons_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_uncons");

    group.bench_function("ArrayStack::uncons", |b| {
        b.iter(|| {
            let mut stack = ArrayStack::<i32>::new();
            for i in 0..100 {
                stack = stack.cons(i);
            }

            let mut result = 0;
            while let Ok((value, new_stack)) = stack.uncons() {
                result += value;
                stack = new_stack;
            }
            black_box(result);
        })
    });

    group.bench_function("PersistentStack::uncons", |b| {
        b.iter(|| {
            let mut stack = PersistentStack::<i32>::new();
            for i in 0..100 {
                stack = stack.cons(i);
            }

            let mut result = 0;
            while let Ok((value, new_stack)) = stack.uncons() {
                result += value;
                stack = new_stack;
            }
            black_box(result);
        })
    });

    group.finish();
}

criterion_group!(benches, stack_cons_benchmark, stack_uncons_benchmark);
criterion_main!(benches);
