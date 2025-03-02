use crate::{AsyncQueue, TokioQueue};
use rust_fp_categories::r#async::{
    AsyncApply, AsyncBind, AsyncEmpty, AsyncFoldable, AsyncFunctor, AsyncMonoid, AsyncPure,
    AsyncSemigroup,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_functor() {
        // Create queue with async operations
        let empty_queue =
            <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let queue1 = empty_queue.enqueue(1).await;
        let queue2 = queue1.enqueue(2).await;
        let queue = queue2.enqueue(3).await;

        // Test fmap
        let mapped_queue = queue.fmap(|x| x * 2).await;

        // Verify the result
        let mut values = Vec::new();
        let mut current_queue = mapped_queue;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_queue).await {
            match current_queue.dequeue().await {
                Ok((value, new_queue)) => {
                    values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        assert_eq!(values, vec![2, 4, 6]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_pure() {
        // Test pure
        let queue = TokioQueue::<i32>::pure(42).await;

        // Verify the queue contains only the pure value
        match queue.dequeue().await {
            Ok((value, new_queue)) => {
                assert_eq!(value, 42);
                assert!(AsyncEmpty::is_empty(&new_queue).await);
            }
            Err(_) => panic!("Expected a value in the queue"),
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_apply() {
        // Create queue with async operations
        let empty_queue =
            <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let queue1 = empty_queue.enqueue(1).await;
        let queue2 = queue1.enqueue(2).await;
        let queue = queue2.enqueue(3).await;

        // Define functions to be used in the test
        fn double(x: &i32) -> i32 {
            x * 2
        }
        fn add_ten(x: &i32) -> i32 {
            x + 10
        }

        // Create a queue with function pointers
        let mut functions =
            <TokioQueue<fn(&i32) -> i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        functions = functions.enqueue(double as fn(&i32) -> i32).await;
        functions = functions.enqueue(add_ten as fn(&i32) -> i32).await;

        // Test ap
        let result_queue = queue.ap(&functions).await;

        // Verify the result queue contains the expected values
        let mut values = Vec::new();
        let mut current_queue = result_queue;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_queue).await {
            match current_queue.dequeue().await {
                Ok((value, new_queue)) => {
                    values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        // Expected: [1*2, 2*2, 3*2, 1+10, 2+10, 3+10]
        assert_eq!(values, vec![2, 4, 6, 11, 12, 13]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_bind() {
        // Create queue with async operations
        let empty_queue =
            <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let queue1 = empty_queue.enqueue(1).await;
        let queue2 = queue1.enqueue(2).await;
        let queue = queue2.enqueue(3).await;

        // Test bind
        let result_queue = queue
            .bind(|x: &i32| {
                let x_clone = *x;
                Box::pin(async move {
                    let empty_queue =
                        <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
                    let queue = TokioQueue::pure(x_clone * 2).await;
                    queue
                })
            })
            .await;

        // Verify the result queue contains the expected values
        let mut values = Vec::new();
        let mut current_queue = result_queue;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_queue).await {
            match current_queue.dequeue().await {
                Ok((value, new_queue)) => {
                    values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        // Expected: [1*2, 2*2, 3*2]
        assert_eq!(values, vec![2, 4, 6]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_fold_left() {
        // Create queue with async operations
        let empty_queue =
            <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let queue1 = empty_queue.enqueue(1).await;
        let queue2 = queue1.enqueue(2).await;
        let queue = queue2.enqueue(3).await;

        // Test fold_left
        let sum = queue
            .fold_left(0, |acc, x: &i32| {
                let x_clone = *x;
                Box::pin(async move { acc + x_clone })
            })
            .await;

        assert_eq!(sum, 6);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_fold_right() {
        // Create queue with async operations
        let empty_queue =
            <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let queue1 = empty_queue.enqueue(1).await;
        let queue2 = queue1.enqueue(2).await;
        let queue = queue2.enqueue(3).await;

        // Test fold_right
        let sum = queue
            .fold_right(0, |x: &i32, acc| {
                let x_clone = *x;
                Box::pin(async move { x_clone + acc })
            })
            .await;

        assert_eq!(sum, 6);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_complex_async_operations() {
        // Create queue with async operations
        let empty_queue =
            <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let queue1 = empty_queue.enqueue(1).await;
        let queue2 = queue1.enqueue(2).await;
        let queue = queue2.enqueue(3).await;

        // Combine multiple async operations
        // 1. Map each element to its double
        // 2. Bind each element to a queue containing the element and its square
        let result_queue = async {
            let mapped_queue = queue.fmap(|x| x * 2).await;

            mapped_queue
                .bind(|x: &i32| {
                    let x_clone = *x;
                    Box::pin(async move {
                        let empty_queue =
                            <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty()
                                .await;
                        let queue1 = empty_queue.enqueue(x_clone).await;
                        let queue2 = queue1.enqueue(x_clone * x_clone).await;
                        queue2
                    })
                })
                .await
        }
        .await;

        // Verify the result queue contains the expected values
        let mut values = Vec::new();
        let mut current_queue = result_queue;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_queue).await {
            match current_queue.dequeue().await {
                Ok((value, new_queue)) => {
                    values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        // Expected: [2, 4, 4, 16, 6, 36]
        assert_eq!(values, vec![2, 4, 4, 16, 6, 36]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_semigroup() {
        // Create two queues
        let empty_queue =
            <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let queue1 = empty_queue.enqueue(1).await;
        let queue1 = queue1.enqueue(2).await;
        let queue1 = queue1.enqueue(3).await;

        let queue2 = empty_queue.enqueue(4).await;
        let queue2 = queue2.enqueue(5).await;
        let queue2 = queue2.enqueue(6).await;

        // Combine the queues
        let combined_queue = queue1.combine(&queue2).await;

        // Verify the result
        let mut values = Vec::new();
        let mut current_queue = combined_queue;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_queue).await {
            match current_queue.dequeue().await {
                Ok((value, new_queue)) => {
                    values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        assert_eq!(values, vec![1, 2, 3, 4, 5, 6]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_monoid_identity() {
        // Create a queue
        let empty_queue =
            <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let queue = empty_queue.enqueue(1).await;
        let queue = queue.enqueue(2).await;
        let queue = queue.enqueue(3).await;

        // Test left identity: empty.combine(queue) == queue
        let left_combined = empty_queue.combine(&queue).await;

        // Test right identity: queue.combine(empty) == queue
        let right_combined = queue.combine(&empty_queue).await;

        // Verify the results
        let mut left_values = Vec::new();
        let mut current_queue = left_combined;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_queue).await {
            match current_queue.dequeue().await {
                Ok((value, new_queue)) => {
                    left_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        let mut right_values = Vec::new();
        let mut current_queue = right_combined;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_queue).await {
            match current_queue.dequeue().await {
                Ok((value, new_queue)) => {
                    right_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        assert_eq!(left_values, vec![1, 2, 3]);
        assert_eq!(right_values, vec![1, 2, 3]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_semigroup_associativity() {
        // Create three queues
        let empty_queue =
            <TokioQueue<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let queue1 = empty_queue.enqueue(1).await;
        let queue2 = empty_queue.enqueue(2).await;
        let queue3 = empty_queue.enqueue(3).await;

        // Test associativity: (queue1.combine(queue2)).combine(queue3) == queue1.combine(queue2.combine(queue3))
        let combined1 = queue1.combine(&queue2).await;
        let left_combined = combined1.combine(&queue3).await;

        let combined2 = queue2.combine(&queue3).await;
        let right_combined = queue1.combine(&combined2).await;

        // Verify the results
        let mut left_values = Vec::new();
        let mut current_queue = left_combined;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_queue).await {
            match current_queue.dequeue().await {
                Ok((value, new_queue)) => {
                    left_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        let mut right_values = Vec::new();
        let mut current_queue = right_combined;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_queue).await {
            match current_queue.dequeue().await {
                Ok((value, new_queue)) => {
                    right_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        assert_eq!(left_values, right_values);
        assert_eq!(left_values, vec![1, 2, 3]);
    }
}
