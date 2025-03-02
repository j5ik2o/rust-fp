use criterion::{black_box, criterion_group, criterion_main, Criterion};
use im;
use rpds;
use rust_fp_pfds::{ArrayStack, PersistentStack, Stack};

fn stack_push_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_push_comparison");

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

    // im::Vector as stack
    group.bench_function("im::Vector::push_back", |b| {
        b.iter(|| {
            let mut vec = im::Vector::<i32>::new();
            for i in 0..100 {
                vec.push_back(black_box(i));
            }
            black_box(vec);
        })
    });

    // rpds::Stack
    group.bench_function("rpds::Stack::push", |b| {
        b.iter(|| {
            let mut stack = rpds::Stack::<i32>::new();
            for i in 0..100 {
                stack = stack.push(black_box(i));
            }
            black_box(stack);
        })
    });

    group.finish();
}

fn stack_pop_benchmark(c: &mut Criterion) {
    // Create stacks with 100 elements
    let mut array_stack = ArrayStack::<i32>::new();
    let mut persistent_stack = PersistentStack::<i32>::new();
    let mut im_vec = im::Vector::<i32>::new();
    let mut rpds_stack = rpds::Stack::<i32>::new();

    for i in 0..100 {
        array_stack = array_stack.cons(i);
        persistent_stack = persistent_stack.cons(i);
        im_vec.push_back(i);
        rpds_stack = rpds_stack.push(i);
    }

    let mut group = c.benchmark_group("stack_pop_comparison");

    group.bench_function("ArrayStack::uncons", |b| {
        b.iter(|| {
            let mut result = 0;
            let mut current_stack = array_stack.clone();
            while let Ok((value, new_stack)) = current_stack.uncons() {
                result += value;
                current_stack = new_stack;
            }
            black_box(result);
        })
    });

    group.bench_function("PersistentStack::uncons", |b| {
        b.iter(|| {
            let mut result = 0;
            let mut current_stack = persistent_stack.clone();
            while let Ok((value, new_stack)) = current_stack.uncons() {
                result += value;
                current_stack = new_stack;
            }
            black_box(result);
        })
    });

    // im::Vector as stack
    group.bench_function("im::Vector::pop_back", |b| {
        b.iter(|| {
            let mut result = 0;
            let mut current_vec = im_vec.clone();
            while !current_vec.is_empty() {
                if let Some(value) = current_vec.pop_back() {
                    result += value;
                }
            }
            black_box(result);
        })
    });

    // rpds::Stack
    group.bench_function("rpds::Stack::pop", |b| {
        b.iter(|| {
            let mut result = 0;
            let mut current_stack = rpds_stack.clone();
            while let Some(new_stack) = current_stack.pop() {
                if let Some(value) = current_stack.peek() {
                    result += *value;
                }
                current_stack = new_stack;
            }
            black_box(result);
        })
    });

    group.finish();
}

criterion_group!(benches, stack_push_benchmark, stack_pop_benchmark);
criterion_main!(benches);
