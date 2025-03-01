use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_pfds::{ArrayDeque, OptimizedDeque, Deque};
use rust_fp_categories::Empty;

fn push_front_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_front");
    
    group.bench_function("ArrayDeque", |b| {
        b.iter(|| {
            let mut deque = ArrayDeque::<i32>::new();
            for i in 0..100 {
                deque = deque.push_front(black_box(i));
            }
        })
    });

    group.bench_function("OptimizedDeque", |b| {
        b.iter(|| {
            let mut deque = OptimizedDeque::<i32>::new();
            for i in 0..100 {
                deque = deque.push_front(black_box(i));
            }
        })
    });

    // TokioDeque is not included in push_front benchmark because it's an async wrapper
    // and would require a runtime, making the comparison unfair

    group.finish();
}

fn push_back_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("push_back");
    
    group.bench_function("ArrayDeque", |b| {
        b.iter(|| {
            let mut deque = ArrayDeque::<i32>::new();
            for i in 0..100 {
                deque = deque.push_back(black_box(i));
            }
        })
    });

    group.bench_function("OptimizedDeque", |b| {
        b.iter(|| {
            let mut deque = OptimizedDeque::<i32>::new();
            for i in 0..100 {
                deque = deque.push_back(black_box(i));
            }
        })
    });

    // TokioDeque is not included for the same reason as above

    group.finish();
}

fn pop_front_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_front");
    
    group.bench_function("ArrayDeque", |b| {
        b.iter(|| {
            let mut deque = ArrayDeque::<i32>::new();
            for i in 0..100 {
                deque = deque.push_back(i);
            }
            
            let mut result = 0;
            while let Ok((value, new_deque)) = deque.pop_front() {
                result += value;
                deque = new_deque;
            }
            black_box(result);
        })
    });

    group.bench_function("OptimizedDeque", |b| {
        b.iter(|| {
            let mut deque = OptimizedDeque::<i32>::new();
            for i in 0..100 {
                deque = deque.push_back(i);
            }
            
            let mut result = 0;
            while let Ok((value, new_deque)) = deque.pop_front() {
                result += value;
                deque = new_deque;
            }
            black_box(result);
        })
    });

    group.finish();
}

fn pop_back_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_back");
    
    group.bench_function("ArrayDeque", |b| {
        b.iter(|| {
            let mut deque = ArrayDeque::<i32>::new();
            for i in 0..100 {
                deque = deque.push_front(i);
            }
            
            let mut result = 0;
            while let Ok((value, new_deque)) = deque.pop_back() {
                result += value;
                deque = new_deque;
            }
            black_box(result);
        })
    });

    group.bench_function("OptimizedDeque", |b| {
        b.iter(|| {
            let mut deque = OptimizedDeque::<i32>::new();
            for i in 0..100 {
                deque = deque.push_front(i);
            }
            
            let mut result = 0;
            while let Ok((value, new_deque)) = deque.pop_back() {
                result += value;
                deque = new_deque;
            }
            black_box(result);
        })
    });

    group.finish();
}

fn mixed_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_operations");
    
    group.bench_function("ArrayDeque", |b| {
        b.iter(|| {
            let mut deque = ArrayDeque::<i32>::new();
            
            // Push from both ends
            for i in 0..50 {
                deque = deque.push_front(i);
                deque = deque.push_back(i);
            }
            
            // Pop from both ends
            let mut result = 0;
            for _ in 0..25 {
                if let Ok((value, new_deque)) = deque.clone().pop_front() {
                    result += value;
                    deque = new_deque;
                }
                
                if let Ok((value, new_deque)) = deque.clone().pop_back() {
                    result += value;
                    deque = new_deque;
                }
            }
            
            black_box(result);
        })
    });

    group.bench_function("OptimizedDeque", |b| {
        b.iter(|| {
            let mut deque = OptimizedDeque::<i32>::new();
            
            // Push from both ends
            for i in 0..50 {
                deque = deque.push_front(i);
                deque = deque.push_back(i);
            }
            
            // Pop from both ends
            let mut result = 0;
            for _ in 0..25 {
                if let Ok((value, new_deque)) = deque.clone().pop_front() {
                    result += value;
                    deque = new_deque;
                }
                
                if let Ok((value, new_deque)) = deque.clone().pop_back() {
                    result += value;
                    deque = new_deque;
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
    
    group.bench_function("ArrayDeque", |b| {
        b.iter(|| {
            let deque = ArrayDeque::<i32>::from_iter(black_box(vec.clone()));
            black_box(deque);
        })
    });

    group.bench_function("OptimizedDeque", |b| {
        b.iter(|| {
            let deque = OptimizedDeque::<i32>::from_iter(black_box(vec.clone()));
            black_box(deque);
        })
    });

    group.finish();
}

criterion_group!(benches, 
    push_front_benchmark, 
    push_back_benchmark, 
    pop_front_benchmark, 
    pop_back_benchmark, 
    mixed_operations_benchmark, 
    from_iter_benchmark
);
criterion_main!(benches);
