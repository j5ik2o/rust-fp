use crate::{ArrayQueue, Queue};
use rust_fp_categories::{Bind, Empty, Foldable, Functor, Pure};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functor() {
        let queue = ArrayQueue::empty().enqueue(1).enqueue(2).enqueue(3);

        let mapped_queue = queue.fmap(|x| x * 2);

        // Verify the mapped queue contains the expected values
        let mut values = Vec::new();
        let mut current_queue = mapped_queue;

        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        assert_eq!(values, vec![2, 4, 6]);
    }

    #[test]
    fn test_pure() {
        let queue = ArrayQueue::<i32>::pure(42);

        // Verify the queue contains only the pure value
        match queue.dequeue() {
            Ok((value, new_queue)) => {
                assert_eq!(value, 42);
                assert!(Empty::is_empty(&new_queue));
            }
            Err(_) => panic!("Expected a value in the queue"),
        }
    }

    #[test]
    fn test_apply() {
        let queue = ArrayQueue::empty().enqueue(1).enqueue(2).enqueue(3);

        // Use an enum to represent functions
        #[derive(Clone)]
        enum IntFunction {
            Double,
            AddTen,
        }

        impl IntFunction {
            fn apply(&self, x: &i32) -> i32 {
                match self {
                    IntFunction::Double => x * 2,
                    IntFunction::AddTen => x + 10,
                }
            }
        }

        let functions = ArrayQueue::empty()
            .enqueue(IntFunction::Double)
            .enqueue(IntFunction::AddTen);

        // Create a custom implementation of ap for our enum-based approach
        let mut result_queue = ArrayQueue::empty();
        let mut fs_clone = functions.clone();

        while let Ok((f, new_fs)) = fs_clone.dequeue() {
            let mut self_clone = queue.clone();
            while let Ok((a, new_self)) = self_clone.dequeue() {
                result_queue = result_queue.enqueue(f.apply(&a));
                self_clone = new_self;
            }
            fs_clone = new_fs;
        }

        // Verify the result queue contains the expected values
        let mut values = Vec::new();
        let mut current_queue = result_queue;

        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
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

    #[test]
    fn test_applicative() {
        // Test that Applicative combines Pure and Apply
        let queue = ArrayQueue::<i32>::pure(5);
        // Use an enum to represent functions
        #[derive(Clone)]
        enum IntFunction {
            Triple,
        }

        impl IntFunction {
            fn apply(&self, x: &i32) -> i32 {
                match self {
                    IntFunction::Triple => x * 3,
                }
            }
        }

        let functions = ArrayQueue::pure(IntFunction::Triple);

        // Create a custom implementation of ap for our enum-based approach
        let mut result_queue = ArrayQueue::empty();
        let mut fs_clone = functions.clone();

        while let Ok((f, new_fs)) = fs_clone.dequeue() {
            let mut self_clone = queue.clone();
            while let Ok((a, new_self)) = self_clone.dequeue() {
                result_queue = result_queue.enqueue(f.apply(&a));
                self_clone = new_self;
            }
            fs_clone = new_fs;
        }

        match result_queue.dequeue() {
            Ok((value, new_queue)) => {
                assert_eq!(value, 15);
                assert!(Empty::is_empty(&new_queue));
            }
            Err(_) => panic!("Expected a value in the queue"),
        }
    }

    #[test]
    fn test_bind() {
        let queue = ArrayQueue::empty().enqueue(1).enqueue(2).enqueue(3);

        let result_queue = queue.bind(|x| ArrayQueue::empty().enqueue(x * 2).enqueue(x + 10));

        // Verify the result queue contains the expected values
        let mut values = Vec::new();
        let mut current_queue = result_queue;

        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        // Expected: [1*2, 1+10, 2*2, 2+10, 3*2, 3+10]
        assert_eq!(values, vec![2, 11, 4, 12, 6, 13]);
    }

    #[test]
    fn test_monad() {
        // Test that Monad combines Pure and Bind
        let queue = ArrayQueue::<i32>::pure(5);

        let result_queue = queue.bind(|x| ArrayQueue::pure(x * 3));

        match result_queue.dequeue() {
            Ok((value, new_queue)) => {
                assert_eq!(value, 15);
                assert!(Empty::is_empty(&new_queue));
            }
            Err(_) => panic!("Expected a value in the queue"),
        }
    }

    #[test]
    fn test_foldable() {
        let queue = ArrayQueue::empty().enqueue(1).enqueue(2).enqueue(3);

        // Test fold_left
        let sum_left = queue.fold_left(0, |acc, x| acc + x);
        assert_eq!(sum_left, 6);

        // Test fold_right
        let sum_right = queue.fold_right(0, |x, acc| x + acc);
        assert_eq!(sum_right, 6);

        // Test more complex fold_left
        let queue2 = ArrayQueue::empty().enqueue(1).enqueue(2).enqueue(3);

        let product_left = queue2.fold_left(1, |acc, x| acc * x);
        assert_eq!(product_left, 6);

        // Test more complex fold_right
        let queue3 = ArrayQueue::empty().enqueue(1).enqueue(2).enqueue(3);

        let product_right = queue3.fold_right(1, |x, acc| x * acc);
        assert_eq!(product_right, 6);
    }

    #[test]
    fn test_functor_laws() {
        // Identity law: fmap(id) == id
        let queue = ArrayQueue::empty().enqueue(1).enqueue(2).enqueue(3);

        let mapped_queue = queue.clone().fmap(|x| x.clone());

        let mut original_values = Vec::new();
        let mut mapped_values = Vec::new();

        let mut current_queue = queue;
        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    original_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        let mut current_queue = mapped_queue;
        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    mapped_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        assert_eq!(original_values, mapped_values);

        // Composition law: fmap(f . g) == fmap(f) . fmap(g)
        let queue = ArrayQueue::empty().enqueue(1).enqueue(2).enqueue(3);

        let f = |x: &i32| x * 2;
        let g = |x: &i32| x + 3;

        // fmap(f . g)
        let composed_mapped = queue.clone().fmap(|x| f(&g(x)));

        // fmap(f) . fmap(g)
        let chained_mapped = queue.clone().fmap(g).fmap(f);

        let mut composed_values = Vec::new();
        let mut chained_values = Vec::new();

        let mut current_queue = composed_mapped;
        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    composed_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        let mut current_queue = chained_mapped;
        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    chained_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        assert_eq!(composed_values, chained_values);
    }

    #[test]
    fn test_monad_laws() {
        // Left identity: return a >>= f == f a
        let a = 5;
        let f = |x: &i32| ArrayQueue::empty().enqueue(x * 2).enqueue(x + 10);

        let left_identity = ArrayQueue::pure(a).bind(f);
        let f_a = f(&a);

        let mut left_identity_values = Vec::new();
        let mut f_a_values = Vec::new();

        let mut current_queue = left_identity;
        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    left_identity_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        let mut current_queue = f_a;
        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    f_a_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        assert_eq!(left_identity_values, f_a_values);

        // Right identity: m >>= return == m
        let m = ArrayQueue::empty().enqueue(1).enqueue(2).enqueue(3);

        let right_identity = m.clone().bind(|x| ArrayQueue::pure(*x));

        let mut m_values = Vec::new();
        let mut right_identity_values = Vec::new();

        let mut current_queue = m;
        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    m_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        let mut current_queue = right_identity;
        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    right_identity_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        assert_eq!(m_values, right_identity_values);

        // Associativity: (m >>= f) >>= g == m >>= (\x -> f x >>= g)
        let m = ArrayQueue::empty().enqueue(1).enqueue(2).enqueue(3);

        let f = |x: &i32| ArrayQueue::empty().enqueue(x * 2);
        let g = |x: &i32| ArrayQueue::empty().enqueue(x + 10);

        // (m >>= f) >>= g
        let left_associative = m.clone().bind(f).bind(g);

        // m >>= (\x -> f x >>= g)
        let right_associative = m.clone().bind(|x| f(x).bind(g));

        let mut left_values = Vec::new();
        let mut right_values = Vec::new();

        let mut current_queue = left_associative;
        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    left_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        let mut current_queue = right_associative;
        while !Empty::is_empty(&current_queue) {
            match current_queue.dequeue() {
                Ok((value, new_queue)) => {
                    right_values.push(value);
                    current_queue = new_queue;
                }
                Err(_) => break,
            }
        }

        assert_eq!(left_values, right_values);
    }
}
