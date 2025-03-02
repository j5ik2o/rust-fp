use crate::{AsyncDeque, TokioDeque};
use rust_fp_categories::r#async::{
    AsyncApply, AsyncBind, AsyncEmpty, AsyncFoldable, AsyncFunctor, AsyncPure,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_async_functor() {
        // Create deque with async operations
        let empty_deque = <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
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
        let empty_deque = <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
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
        let mut functions = <TokioDeque<fn(&i32) -> i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
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
        let empty_deque = <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
        let deque1 = empty_deque.push_back(1).await;
        let deque2 = deque1.push_back(2).await;
        let deque = deque2.push_back(3).await;

        // Test bind
        let result_deque = deque
            .bind(|x: &i32| {
                let x_clone = *x;
                Box::pin(async move {
                    let empty_deque = <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
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
        let empty_deque = <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
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
        let empty_deque = <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
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
        let empty_deque = <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
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
                        let empty_deque = <TokioDeque<i32> as rust_fp_categories::r#async::AsyncEmpty>::empty().await;
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
}
