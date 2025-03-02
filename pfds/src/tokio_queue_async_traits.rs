use std::future::Future;
use std::pin::Pin;

use rust_fp_categories::r#async::{
    AsyncApplicative, AsyncApply, AsyncBind, AsyncFoldable, AsyncFunctor, AsyncMonad, AsyncPure,
};
use rust_fp_categories::Empty;

use crate::{AsyncQueue, TokioQueue};

impl<A: Clone + Send + Sync + 'static> AsyncFunctor for TokioQueue<A> {
    type Elm = A;
    type M<B: Clone + Send + Sync + 'static> = TokioQueue<B>;

    fn fmap<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        f: F,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> B + Send + Sync + 'a,
    {
        Box::pin(async move {
            let mut result = TokioQueue::empty();
            let mut current_queue = self.clone();

            while !Empty::is_empty(&current_queue) {
                match current_queue.dequeue().await {
                    Ok((value, new_queue)) => {
                        let mapped_value = f(&value);
                        result = result.enqueue(mapped_value).await;
                        current_queue = new_queue;
                    }
                    Err(_) => break,
                }
            }

            result
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncPure for TokioQueue<A> {
    type Elm = A;

    fn pure<'a>(value: Self::Elm) -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        Self: Sized + 'a,
    {
        Box::pin(async move {
            let empty_queue = TokioQueue::empty();
            empty_queue.enqueue(value).await
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncApply for TokioQueue<A> {
    fn ap<'a, B: Clone + Send + Sync + 'static, F: Clone + Send + Sync + 'static>(
        &'a self,
        fs: &'a Self::M<F>,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> B + Send + Sync + 'a,
    {
        Box::pin(async move {
            let mut result = TokioQueue::empty();
            let mut fs_clone = fs.clone();

            while !Empty::is_empty(&fs_clone) {
                match fs_clone.dequeue().await {
                    Ok((f, new_fs)) => {
                        let mut self_clone = self.clone();
                        while !Empty::is_empty(&self_clone) {
                            match self_clone.dequeue().await {
                                Ok((a, new_self)) => {
                                    let b = f(&a);
                                    result = result.enqueue(b).await;
                                    self_clone = new_self;
                                }
                                Err(_) => break,
                            }
                        }
                        fs_clone = new_fs;
                    }
                    Err(_) => break,
                }
            }

            result
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncBind for TokioQueue<A> {
    type Elm = A;
    type M<B: Clone + Send + Sync + 'static> = TokioQueue<B>;

    fn bind<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        f: F,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>> + Send + Sync + 'a,
    {
        Box::pin(async move {
            let mut result = TokioQueue::empty();
            let mut current_queue = self.clone();

            while !Empty::is_empty(&current_queue) {
                match current_queue.dequeue().await {
                    Ok((value, new_queue)) => {
                        let mb = f(&value).await;
                        let mut mb_clone = mb.clone();

                        while !Empty::is_empty(&mb_clone) {
                            match mb_clone.dequeue().await {
                                Ok((b, new_mb)) => {
                                    result = result.enqueue(b).await;
                                    mb_clone = new_mb;
                                }
                                Err(_) => break,
                            }
                        }

                        current_queue = new_queue;
                    }
                    Err(_) => break,
                }
            }

            result
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncApplicative for TokioQueue<A> {}
impl<A: Clone + Send + Sync + 'static> AsyncMonad for TokioQueue<A> {}

impl<A: Clone + Send + Sync + 'static> AsyncFoldable for TokioQueue<A> {
    type Elm = A;

    fn fold_left<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        b: B,
        f: F,
    ) -> Pin<Box<dyn Future<Output = B> + 'a>>
    where
        F: Fn(B, &Self::Elm) -> Pin<Box<dyn Future<Output = B> + 'a>> + Send + Sync + 'a,
    {
        Box::pin(async move {
            let mut result = b;
            let mut current_queue = self.clone();

            while !Empty::is_empty(&current_queue) {
                match current_queue.dequeue().await {
                    Ok((value, new_queue)) => {
                        result = f(result, &value).await;
                        current_queue = new_queue;
                    }
                    Err(_) => break,
                }
            }

            result
        })
    }

    fn fold_right<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        b: B,
        f: F,
    ) -> Pin<Box<dyn Future<Output = B> + 'a>>
    where
        F: Fn(&Self::Elm, B) -> Pin<Box<dyn Future<Output = B> + 'a>> + Send + Sync + 'a,
    {
        Box::pin(async move {
            let mut values = Vec::new();
            let mut current_queue = self.clone();

            while !Empty::is_empty(&current_queue) {
                match current_queue.dequeue().await {
                    Ok((value, new_queue)) => {
                        values.push(value);
                        current_queue = new_queue;
                    }
                    Err(_) => break,
                }
            }

            let mut result = b;
            for value in values.iter().rev() {
                result = f(value, result).await;
            }

            result
        })
    }
}
