use crate::{AsyncDeque, TokioDeque};
use rust_fp_categories::r#async::{
    AsyncApply, AsyncBind, AsyncEmpty, AsyncFoldable, AsyncFunctor, AsyncMonoid, AsyncPure,
    AsyncSemigroup,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_functor() {
        // Create deque with async operations
        let empty_deque =
            <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let deque1 = empty_deque.push_back(1).await;
        let deque2 = deque1.push_back(2).await;
        let deque = deque2.push_back(3).await;

        // Test fmap
        let mapped_deque = deque.fmap(|x| x * 2).await;

        // Verify the result
        let mut values = Vec::new();
        let mut current_deque = mapped_deque;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_deque).await {
            match current_deque.pop_front().await {
                Ok((value, new_deque)) => {
                    values.push(value);
                    current_deque = new_deque;
                }
                Err(_) => break,
            }
        }

        assert_eq!(values, vec![2, 4, 6]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_pure() {
        // Test pure
        let deque = TokioDeque::<i32>::pure(42).await;

        // Verify the deque contains only the pure value
        match deque.pop_front().await {
            Ok((value, new_deque)) => {
                assert_eq!(value, 42);
                assert!(AsyncEmpty::is_empty(&new_deque).await);
            }
            Err(_) => panic!("Expected a value in the deque"),
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_apply() {
        // Create deque with async operations
        let empty_deque =
            <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let deque1 = empty_deque.push_back(1).await;
        let deque2 = deque1.push_back(2).await;
        let deque = deque2.push_back(3).await;

        // Define functions to be used in the test
        fn double(x: &i32) -> i32 {
            x * 2
        }
        fn add_ten(x: &i32) -> i32 {
            x + 10
        }

        // Create a deque with function pointers
        let mut functions =
            <TokioDeque<fn(&i32) -> i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        functions = functions.push_back(double as fn(&i32) -> i32).await;
        functions = functions.push_back(add_ten as fn(&i32) -> i32).await;

        // Test ap
        let result_deque = deque.ap(&functions).await;

        // Verify the result deque contains the expected values
        let mut values = Vec::new();
        let mut current_deque = result_deque;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_deque).await {
            match current_deque.pop_front().await {
                Ok((value, new_deque)) => {
                    values.push(value);
                    current_deque = new_deque;
                }
                Err(_) => break,
            }
        }

        // Expected: [1*2, 2*2, 3*2, 1+10, 2+10, 3+10]
        assert_eq!(values, vec![2, 4, 6, 11, 12, 13]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_bind() {
        // Create deque with async operations
        let empty_deque =
            <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let deque1 = empty_deque.push_back(1).await;
        let deque2 = deque1.push_back(2).await;
        let deque = deque2.push_back(3).await;

        // Test bind
        let result_deque = deque
            .bind(|x: &i32| {
                let x_clone = *x;
                Box::pin(async move {
                    let empty_deque =
                        <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
                    let deque = TokioDeque::pure(x_clone * 2).await;
                    deque
                })
            })
            .await;

        // Verify the result deque contains the expected values
        let mut values = Vec::new();
        let mut current_deque = result_deque;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_deque).await {
            match current_deque.pop_front().await {
                Ok((value, new_deque)) => {
                    values.push(value);
                    current_deque = new_deque;
                }
                Err(_) => break,
            }
        }

        // Expected: [1*2, 2*2, 3*2]
        assert_eq!(values, vec![2, 4, 6]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_fold_left() {
        // Create deque with async operations
        let empty_deque =
            <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let deque1 = empty_deque.push_back(1).await;
        let deque2 = deque1.push_back(2).await;
        let deque = deque2.push_back(3).await;

        // Test fold_left
        let sum = deque
            .fold_left(0, |acc, x: &i32| {
                let x_clone = *x;
                Box::pin(async move { acc + x_clone })
            })
            .await;

        assert_eq!(sum, 6);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_fold_right() {
        // Create deque with async operations
        let empty_deque =
            <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let deque1 = empty_deque.push_back(1).await;
        let deque2 = deque1.push_back(2).await;
        let deque = deque2.push_back(3).await;

        // Test fold_right
        let sum = deque
            .fold_right(0, |x: &i32, acc| {
                let x_clone = *x;
                Box::pin(async move { x_clone + acc })
            })
            .await;

        assert_eq!(sum, 6);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_complex_async_operations() {
        // Create deque with async operations
        let empty_deque =
            <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let deque1 = empty_deque.push_back(1).await;
        let deque2 = deque1.push_back(2).await;
        let deque = deque2.push_back(3).await;

        // Combine multiple async operations
        // 1. Map each element to its double
        // 2. Bind each element to a deque containing the element and its square
        let result_deque = async {
            let mapped_deque = deque.fmap(|x| x * 2).await;

            mapped_deque
                .bind(|x: &i32| {
                    let x_clone = *x;
                    Box::pin(async move {
                        let empty_deque =
                            <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty()
                                .await;
                        let deque1 = empty_deque.push_back(x_clone).await;
                        let deque2 = deque1.push_back(x_clone * x_clone).await;
                        deque2
                    })
                })
                .await
        }
        .await;

        // Verify the result deque contains the expected values
        let mut values = Vec::new();
        let mut current_deque = result_deque;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_deque).await {
            match current_deque.pop_front().await {
                Ok((value, new_deque)) => {
                    values.push(value);
                    current_deque = new_deque;
                }
                Err(_) => break,
            }
        }

        // Expected: [2, 4, 4, 16, 6, 36]
        assert_eq!(values, vec![2, 4, 4, 16, 6, 36]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_semigroup() {
        // Create two deques
        let empty_deque =
            <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let deque1 = empty_deque.push_back(1).await;
        let deque1 = deque1.push_back(2).await;
        let deque1 = deque1.push_back(3).await;

        let deque2 = empty_deque.push_back(4).await;
        let deque2 = deque2.push_back(5).await;
        let deque2 = deque2.push_back(6).await;

        // Combine the deques
        let combined_deque = deque1.combine(&deque2).await;

        // Verify the result
        let mut values = Vec::new();
        let mut current_deque = combined_deque;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_deque).await {
            match current_deque.pop_front().await {
                Ok((value, new_deque)) => {
                    values.push(value);
                    current_deque = new_deque;
                }
                Err(_) => break,
            }
        }

        assert_eq!(values, vec![1, 2, 3, 4, 5, 6]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_monoid_identity() {
        // Create a deque
        let empty_deque =
            <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let deque = empty_deque.push_back(1).await;
        let deque = deque.push_back(2).await;
        let deque = deque.push_back(3).await;

        // Test left identity: empty.combine(deque) == deque
        let left_combined = empty_deque.combine(&deque).await;

        // Test right identity: deque.combine(empty) == deque
        let right_combined = deque.combine(&empty_deque).await;

        // Verify the results
        let mut left_values = Vec::new();
        let mut current_deque = left_combined;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_deque).await {
            match current_deque.pop_front().await {
                Ok((value, new_deque)) => {
                    left_values.push(value);
                    current_deque = new_deque;
                }
                Err(_) => break,
            }
        }

        let mut right_values = Vec::new();
        let mut current_deque = right_combined;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_deque).await {
            match current_deque.pop_front().await {
                Ok((value, new_deque)) => {
                    right_values.push(value);
                    current_deque = new_deque;
                }
                Err(_) => break,
            }
        }

        assert_eq!(left_values, vec![1, 2, 3]);
        assert_eq!(right_values, vec![1, 2, 3]);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_semigroup_associativity() {
        // Create three deques
        let empty_deque =
            <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let deque1 = empty_deque.push_back(1).await;
        let deque2 = empty_deque.push_back(2).await;
        let deque3 = empty_deque.push_back(3).await;

        // Test associativity: (deque1.combine(deque2)).combine(deque3) == deque1.combine(deque2.combine(deque3))
        let combined1 = deque1.combine(&deque2).await;
        let left_combined = combined1.combine(&deque3).await;

        let combined2 = deque2.combine(&deque3).await;
        let right_combined = deque1.combine(&combined2).await;

        // Verify the results
        let mut left_values = Vec::new();
        let mut current_deque = left_combined;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_deque).await {
            match current_deque.pop_front().await {
                Ok((value, new_deque)) => {
                    left_values.push(value);
                    current_deque = new_deque;
                }
                Err(_) => break,
            }
        }

        let mut right_values = Vec::new();
        let mut current_deque = right_combined;

        while !rust_fp_categories::r#async::AsyncEmpty::is_empty(&current_deque).await {
            match current_deque.pop_front().await {
                Ok((value, new_deque)) => {
                    right_values.push(value);
                    current_deque = new_deque;
                }
                Err(_) => break,
            }
        }

        assert_eq!(left_values, right_values);
        assert_eq!(left_values, vec![1, 2, 3]);
    }
}
