use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_fp_pfds::{ArrayQueue, ListQueue, OptimizedQueue, Queue};

fn queue_enqueue_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_enqueue");

    group.bench_function("ArrayQueue::enqueue", |b| {
        b.iter(|| {
            let mut queue = ArrayQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(black_box(i));
            }
            black_box(queue);
        })
    });

    group.bench_function("ListQueue::enqueue", |b| {
        b.iter(|| {
            let mut queue = ListQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(black_box(i));
            }
            black_box(queue);
        })
    });

    group.bench_function("OptimizedQueue::enqueue", |b| {
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

fn queue_dequeue_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_dequeue");

    group.bench_function("ArrayQueue::dequeue", |b| {
        b.iter(|| {
            let mut queue = ArrayQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(i);
            }

            let mut result = 0;
            while let Ok((value, new_queue)) = queue.dequeue() {
                result += value;
                queue = new_queue;
            }
            black_box(result);
        })
    });

    group.bench_function("ListQueue::dequeue", |b| {
        b.iter(|| {
            let mut queue = ListQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(i);
            }

            let mut result = 0;
            while let Ok((value, new_queue)) = queue.dequeue() {
                result += value;
                queue = new_queue;
            }
            black_box(result);
        })
    });

    group.bench_function("OptimizedQueue::dequeue", |b| {
        b.iter(|| {
            let mut queue = OptimizedQueue::<i32>::new();
            for i in 0..100 {
                queue = queue.enqueue(i);
            }

            let mut result = 0;
            while let Ok((value, new_queue)) = queue.dequeue() {
                result += value;
                queue = new_queue;
            }
            black_box(result);
        })
    });

    group.finish();
}

criterion_group!(benches, queue_enqueue_benchmark, queue_dequeue_benchmark);
criterion_main!(benches);
