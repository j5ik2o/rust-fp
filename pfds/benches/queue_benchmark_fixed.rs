use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_categories::Empty;
use rust_fp_pfds::{ArrayQueue, ListQueue, OptimizedQueue, Queue};

fn enqueue_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_enqueue");

    group.bench_function("ArrayQueue", |b| {
        b.iter(|| {
            let mut queue = ArrayQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(black_box(i));
            }
            black_box(queue);
        })
    });

    group.bench_function("ListQueue", |b| {
        b.iter(|| {
            let mut queue = ListQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(black_box(i));
            }
            black_box(queue);
        })
    });

    group.bench_function("OptimizedQueue", |b| {
        b.iter(|| {
            let mut queue = OptimizedQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(black_box(i));
            }
            black_box(queue);
        })
    });

    group.finish();
}

fn dequeue_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_dequeue");

    group.bench_function("ArrayQueue", |b| {
        b.iter(|| {
            // Create a new queue for each iteration
            let mut queue = ArrayQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(i);
            }

            // Process all elements
            let mut result = 0;
            let mut current_queue = queue;
            while let Ok((value, new_queue)) = current_queue.dequeue() {
                result += value;
                current_queue = new_queue;
            }
            black_box(result);
        })
    });

    group.bench_function("ListQueue", |b| {
        b.iter(|| {
            // Create a new queue for each iteration
            let mut queue = ListQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(i);
            }

            // Process all elements
            let mut result = 0;
            let mut current_queue = queue;
            while let Ok((value, new_queue)) = current_queue.dequeue() {
                result += value;
                current_queue = new_queue;
            }
            black_box(result);
        })
    });

    group.bench_function("OptimizedQueue", |b| {
        b.iter(|| {
            // Create a new queue for each iteration
            let mut queue = OptimizedQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(i);
            }

            // Process all elements
            let mut result = 0;
            let mut current_queue = queue;
            while let Ok((value, new_queue)) = current_queue.dequeue() {
                result += value;
                current_queue = new_queue;
            }
            black_box(result);
        })
    });

    group.finish();
}

fn peek_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_peek");

    // Create queues with 100 elements
    let mut array_queue = ArrayQueue::<i32>::new();
    let mut list_queue = ListQueue::<i32>::new();
    let mut optimized_queue = OptimizedQueue::<i32>::new();

    for i in 0..100 {
        array_queue = array_queue.enqueue(i);
        list_queue = list_queue.enqueue(i);
        optimized_queue = optimized_queue.enqueue(i);
    }

    group.bench_function("ArrayQueue", |b| {
        b.iter(|| {
            let result = array_queue.peek();
            black_box(result);
        })
    });

    group.bench_function("ListQueue", |b| {
        b.iter(|| {
            let result = list_queue.peek();
            black_box(result);
        })
    });

    group.bench_function("OptimizedQueue", |b| {
        b.iter(|| {
            let result = optimized_queue.peek();
            black_box(result);
        })
    });

    group.finish();
}

fn mixed_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_mixed_operations");

    group.bench_function("ArrayQueue", |b| {
        b.iter(|| {
            let mut queue = ArrayQueue::<i32>::new();

            // Enqueue elements
            for i in 0..50 {
                queue = queue.enqueue(i);
            }

            // Peek and dequeue alternately
            let mut result = 0;
            for _ in 0..25 {
                if let Ok(value) = queue.peek() {
                    result += value;
                }

                // Clone the queue before dequeuing
                let queue_clone = queue.clone();
                if let Ok((value, new_queue)) = queue_clone.dequeue() {
                    result += value;
                    queue = new_queue;
                }
            }

            // Enqueue more elements
            for i in 50..75 {
                queue = queue.enqueue(i);
            }

            // Dequeue remaining elements
            let mut current_queue = queue;
            while let Ok((value, new_queue)) = current_queue.dequeue() {
                result += value;
                current_queue = new_queue;
            }

            black_box(result);
        })
    });

    group.bench_function("ListQueue", |b| {
        b.iter(|| {
            let mut queue = ListQueue::<i32>::new();

            // Enqueue elements
            for i in 0..50 {
                queue = queue.enqueue(i);
            }

            // Peek and dequeue alternately
            let mut result = 0;
            for _ in 0..25 {
                if let Ok(value) = queue.peek() {
                    result += value;
                }

                // Clone the queue before dequeuing
                let queue_clone = queue.clone();
                if let Ok((value, new_queue)) = queue_clone.dequeue() {
                    result += value;
                    queue = new_queue;
                }
            }

            // Enqueue more elements
            for i in 50..75 {
                queue = queue.enqueue(i);
            }

            // Dequeue remaining elements
            let mut current_queue = queue;
            while let Ok((value, new_queue)) = current_queue.dequeue() {
                result += value;
                current_queue = new_queue;
            }

            black_box(result);
        })
    });

    group.bench_function("OptimizedQueue", |b| {
        b.iter(|| {
            let mut queue = OptimizedQueue::<i32>::new();

            // Enqueue elements
            for i in 0..50 {
                queue = queue.enqueue(i);
            }

            // Peek and dequeue alternately
            let mut result = 0;
            for _ in 0..25 {
                if let Ok(value) = queue.peek() {
                    result += value;
                }

                // Clone the queue before dequeuing
                let queue_clone = queue.clone();
                if let Ok((value, new_queue)) = queue_clone.dequeue() {
                    result += value;
                    queue = new_queue;
                }
            }

            // Enqueue more elements
            for i in 50..75 {
                queue = queue.enqueue(i);
            }

            // Dequeue remaining elements
            let mut current_queue = queue;
            while let Ok((value, new_queue)) = current_queue.dequeue() {
                result += value;
                current_queue = new_queue;
            }

            black_box(result);
        })
    });

    group.finish();
}

fn from_iter_benchmark(c: &mut Criterion) {
    let vec = (0..100).collect::<Vec<i32>>();

    let mut group = c.benchmark_group("queue_from_iter");

    group.bench_function("ArrayQueue", |b| {
        b.iter(|| {
            let queue = ArrayQueue::<i32>::from_iter(black_box(vec.clone()));
            black_box(queue);
        })
    });

    group.bench_function("ListQueue", |b| {
        b.iter(|| {
            let queue = ListQueue::<i32>::from_iter(black_box(vec.clone()));
            black_box(queue);
        })
    });

    group.bench_function("OptimizedQueue", |b| {
        b.iter(|| {
            let queue = OptimizedQueue::<i32>::from_iter(black_box(vec.clone()));
            black_box(queue);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    enqueue_benchmark,
    dequeue_benchmark,
    peek_benchmark,
    mixed_operations_benchmark,
    from_iter_benchmark
);
criterion_main!(benches);
