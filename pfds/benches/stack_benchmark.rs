use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_categories::Empty;
use rust_fp_pfds::{ArrayStack, PersistentStack, Stack};

fn push_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_push");

    group.bench_function("ArrayStack", |b| {
        b.iter(|| {
            let mut stack = ArrayStack::<i32>::new();
            for i in 0..100 {
                stack = stack.push(black_box(i));
            }
            black_box(stack);
        })
    });

    group.bench_function("PersistentStack", |b| {
        b.iter(|| {
            let mut stack = PersistentStack::<i32>::new();
            for i in 0..100 {
                stack = stack.push(black_box(i));
            }
            black_box(stack);
        })
    });

    group.finish();
}

fn pop_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_pop");

    group.bench_function("ArrayStack", |b| {
        b.iter(|| {
            let mut stack = ArrayStack::<i32>::new();
            for i in 0..100 {
                stack = stack.push(i);
            }

            let mut result = 0;
            while let Ok((value, new_stack)) = stack.pop() {
                result += value;
                stack = new_stack;
            }
            black_box(result);
        })
    });

    group.bench_function("PersistentStack", |b| {
        b.iter(|| {
            let mut stack = PersistentStack::<i32>::new();
            for i in 0..100 {
                stack = stack.push(i);
            }

            let mut result = 0;
            while let Ok((value, new_stack)) = stack.pop() {
                result += value;
                stack = new_stack;
            }
            black_box(result);
        })
    });

    group.finish();
}

fn peek_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_peek");

    // Create stacks with 100 elements
    let mut array_stack = ArrayStack::<i32>::new();
    let mut persistent_stack = PersistentStack::<i32>::new();

    for i in 0..100 {
        array_stack = array_stack.push(i);
        persistent_stack = persistent_stack.push(i);
    }

    group.bench_function("ArrayStack", |b| {
        b.iter(|| {
            let result = array_stack.peek();
            black_box(result);
        })
    });

    group.bench_function("PersistentStack", |b| {
        b.iter(|| {
            let result = persistent_stack.peek();
            black_box(result);
        })
    });

    group.finish();
}

fn mixed_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_mixed_operations");

    group.bench_function("ArrayStack", |b| {
        b.iter(|| {
            let mut stack = ArrayStack::<i32>::new();

            // Push elements
            for i in 0..50 {
                stack = stack.push(i);
            }

            // Peek and pop alternately
            let mut result = 0;
            for _ in 0..25 {
                if let Ok(value) = stack.peek() {
                    result += value;
                }

                if let Ok((value, new_stack)) = stack.pop() {
                    result += value;
                    stack = new_stack;
                }
            }

            // Push more elements
            for i in 50..75 {
                stack = stack.push(i);
            }

            // Pop remaining elements
            while let Ok((value, new_stack)) = stack.pop() {
                result += value;
                stack = new_stack;
            }

            black_box(result);
        })
    });

    group.bench_function("PersistentStack", |b| {
        b.iter(|| {
            let mut stack = PersistentStack::<i32>::new();

            // Push elements
            for i in 0..50 {
                stack = stack.push(i);
            }

            // Peek and pop alternately
            let mut result = 0;
            for _ in 0..25 {
                if let Ok(value) = stack.peek() {
                    result += value;
                }

                if let Ok((value, new_stack)) = stack.pop() {
                    result += value;
                    stack = new_stack;
                }
            }

            // Push more elements
            for i in 50..75 {
                stack = stack.push(i);
            }

            // Pop remaining elements
            while let Ok((value, new_stack)) = stack.pop() {
                result += value;
                stack = new_stack;
            }

            black_box(result);
        })
    });

    group.finish();
}

fn from_iter_benchmark(c: &mut Criterion) {
    let vec = (0..100).collect::<Vec<i32>>();

    let mut group = c.benchmark_group("stack_from_iter");

    group.bench_function("ArrayStack", |b| {
        b.iter(|| {
            let stack = ArrayStack::<i32>::from_iter(black_box(vec.clone()));
            black_box(stack);
        })
    });

    group.bench_function("PersistentStack", |b| {
        b.iter(|| {
            let stack = PersistentStack::<i32>::from_iter(black_box(vec.clone()));
            black_box(stack);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    push_benchmark,
    pop_benchmark,
    peek_benchmark,
    mixed_operations_benchmark,
    from_iter_benchmark
);
criterion_main!(benches);
