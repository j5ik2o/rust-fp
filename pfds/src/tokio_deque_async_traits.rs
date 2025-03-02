use std::future::Future;
use std::pin::Pin;

use rust_fp_categories::r#async::{
    AsyncApplicative, AsyncApply, AsyncBind, AsyncFoldable, AsyncFunctor, AsyncMonad, AsyncPure,
};
use rust_fp_categories::Empty;

use crate::{AsyncDeque, TokioDeque};

impl<A: Clone + Send + Sync + 'static> AsyncFunctor for TokioDeque<A> {
    type Elm = A;
    type M<B: Clone + Send + Sync + 'static> = TokioDeque<B>;

    fn fmap<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        f: F,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> B + Send + Sync + 'a,
    {
        Box::pin(async move {
            let mut result = TokioDeque::empty();
            let mut current_deque = self.clone();

            while !Empty::is_empty(&current_deque) {
                match current_deque.pop_front().await {
                    Ok((value, new_deque)) => {
                        let mapped_value = f(&value);
                        result = result.push_back(mapped_value).await;
                        current_deque = new_deque;
                    }
                    Err(_) => break,
                }
            }

            result
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncPure for TokioDeque<A> {
    type Elm = A;

    fn pure<'a>(value: Self::Elm) -> Pin<Box<dyn Future<Output = Self> + 'a>>
    where
        Self: Sized + 'a,
    {
        Box::pin(async move {
            let empty_deque = TokioDeque::empty();
            empty_deque.push_back(value).await
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncApply for TokioDeque<A> {
    fn ap<'a, B: Clone + Send + Sync + 'static, F: Clone + Send + Sync + 'static>(
        &'a self,
        fs: &'a Self::M<F>,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> B + Send + Sync + 'a,
    {
        Box::pin(async move {
            let mut result = TokioDeque::empty();
            let mut fs_clone = fs.clone();

            while !Empty::is_empty(&fs_clone) {
                match fs_clone.pop_front().await {
                    Ok((f, new_fs)) => {
                        let mut self_clone = self.clone();
                        while !Empty::is_empty(&self_clone) {
                            match self_clone.pop_front().await {
                                Ok((a, new_self)) => {
                                    let b = f(&a);
                                    result = result.push_back(b).await;
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

impl<A: Clone + Send + Sync + 'static> AsyncBind for TokioDeque<A> {
    type Elm = A;
    type M<B: Clone + Send + Sync + 'static> = TokioDeque<B>;

    fn bind<'a, B: Clone + Send + Sync + 'static, F>(
        &'a self,
        f: F,
    ) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>>
    where
        F: Fn(&Self::Elm) -> Pin<Box<dyn Future<Output = Self::M<B>> + 'a>> + Send + Sync + 'a,
    {
        Box::pin(async move {
            let mut result = TokioDeque::empty();
            let mut current_deque = self.clone();

            while !Empty::is_empty(&current_deque) {
                match current_deque.pop_front().await {
                    Ok((value, new_deque)) => {
                        let mb = f(&value).await;
                        let mut mb_clone = mb.clone();

                        while !Empty::is_empty(&mb_clone) {
                            match mb_clone.pop_front().await {
                                Ok((b, new_mb)) => {
                                    result = result.push_back(b).await;
                                    mb_clone = new_mb;
                                }
                                Err(_) => break,
                            }
                        }

                        current_deque = new_deque;
                    }
                    Err(_) => break,
                }
            }

            result
        })
    }
}

impl<A: Clone + Send + Sync + 'static> AsyncApplicative for TokioDeque<A> {}
impl<A: Clone + Send + Sync + 'static> AsyncMonad for TokioDeque<A> {}

impl<A: Clone + Send + Sync + 'static> AsyncFoldable for TokioDeque<A> {
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
            let mut current_deque = self.clone();

            while !Empty::is_empty(&current_deque) {
                match current_deque.pop_front().await {
                    Ok((value, new_deque)) => {
                        result = f(result, &value).await;
                        current_deque = new_deque;
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
            let mut current_deque = self.clone();

            while !Empty::is_empty(&current_deque) {
                match current_deque.pop_front().await {
                    Ok((value, new_deque)) => {
                        values.push(value);
                        current_deque = new_deque;
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
