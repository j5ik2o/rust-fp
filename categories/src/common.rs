//! 共通の型クラス実装のためのユーティリティ関数

use std::rc::Rc;
use std::fmt::Display;

/// Rc<A>型に対する共通の実装パターン
pub mod rc {
    use std::rc::Rc;

    /// Rc<A>に対するfmap実装のためのヘルパー関数
    pub fn fmap<A, B, F>(value: Rc<A>, f: F) -> Rc<B>
    where
        F: Fn(&A) -> B,
    {
        Rc::new(f(&value))
    }

    /// Rc<A>に対するap実装のためのヘルパー関数
    pub fn ap<A, B, F>(value: Rc<A>, fs: Rc<F>) -> Rc<B>
    where
        F: Fn(&A) -> B,
    {
        Rc::new(fs(&value))
    }

    /// Rc<A>に対するbind実装のためのヘルパー関数
    pub fn bind<A, B, F>(value: Rc<A>, f: F) -> Rc<B>
    where
        F: FnOnce(&A) -> Rc<B>,
    {
        f(&value)
    }

    /// Rc<A>に対するpure実装のためのヘルパー関数
    pub fn pure<A>(value: A) -> Rc<A> {
        Rc::new(value)
    }

    /// Rc<A>に対するunit実装のためのヘルパー関数
    pub fn unit() -> Rc<()> {
        Rc::new(())
    }
}

/// Box<A>型に対する共通の実装パターン
pub mod boxed {
    /// Box<A>に対するfmap実装のためのヘルパー関数
    pub fn fmap<A, B, F>(value: Box<A>, f: F) -> Box<B>
    where
        F: Fn(&A) -> B,
    {
        Box::new(f(&value))
    }

    /// Box<A>に対するap実装のためのヘルパー関数
    pub fn ap<A, B, F>(value: Box<A>, fs: Box<F>) -> Box<B>
    where
        F: Fn(&A) -> B,
    {
        Box::new(fs(&value))
    }

    /// Box<A>に対するbind実装のためのヘルパー関数
    pub fn bind<A, B, F>(value: Box<A>, f: F) -> Box<B>
    where
        F: FnOnce(&A) -> Box<B>,
    {
        f(&value)
    }

    /// Box<A>に対するpure実装のためのヘルパー関数
    pub fn pure<A>(value: A) -> Box<A> {
        Box::new(value)
    }

    /// Box<A>に対するunit実装のためのヘルパー関数
    pub fn unit() -> Box<()> {
        Box::new(())
    }
}

/// Option<A>型に対する共通の実装パターン
pub mod option {
    /// Option<A>に対するfmap実装のためのヘルパー関数
    pub fn fmap<A, B, F>(value: Option<A>, f: F) -> Option<B>
    where
        F: Fn(&A) -> B,
    {
        match value {
            Some(ref v) => Some(f(v)),
            None => None,
        }
    }

    /// Option<A>に対するap実装のためのヘルパー関数
    pub fn ap<A, B, F>(value: Option<A>, fs: Option<F>) -> Option<B>
    where
        F: Fn(&A) -> B,
    {
        Some(fs?(&value?))
    }

    /// Option<A>に対するbind実装のためのヘルパー関数
    pub fn bind<A, B, F>(value: Option<A>, f: F) -> Option<B>
    where
        F: FnOnce(&A) -> Option<B>,
    {
        value.and_then(|e| f(&e))
    }

    /// Option<A>に対するpure実装のためのヘルパー関数
    pub fn pure<A>(value: A) -> Option<A> {
        Some(value)
    }

    /// Option<A>に対するunit実装のためのヘルパー関数
    pub fn unit() -> Option<()> {
        Some(())
    }
}

/// Result<A, E>型に対する共通の実装パターン
pub mod result {
    /// Result<A, E>に対するfmap実装のためのヘルパー関数
    pub fn fmap<A, B, E, F>(value: Result<A, E>, f: F) -> Result<B, E>
    where
        F: Fn(&A) -> B,
    {
        match value {
            Ok(v) => Ok(f(&v)),
            Err(e) => Err(e),
        }
    }

    /// Result<A, E>に対するap実装のためのヘルパー関数
    pub fn ap<A, B, E, F>(value: Result<A, E>, fs: Result<F, E>) -> Result<B, E>
    where
        F: Fn(&A) -> B,
    {
        let x = value?;
        let fs = fs?;
        Ok(fs(&x))
    }

    /// Result<A, E>に対するbind実装のためのヘルパー関数
    pub fn bind<A, B, E, F>(value: Result<A, E>, f: F) -> Result<B, E>
    where
        F: FnOnce(&A) -> Result<B, E>,
    {
        value.and_then(|e| f(&e))
    }

    /// Result<A, E>に対するpure実装のためのヘルパー関数
    pub fn pure<A, E>(value: A) -> Result<A, E> {
        Ok(value)
    }

    /// Result<A, E>に対するunit実装のためのヘルパー関数
    pub fn unit<E>() -> Result<(), E> {
        Ok(())
    }
}

/// Vec<A>型に対する共通の実装パターン
pub mod vec {
    /// Vec<A>に対するfmap実装のためのヘルパー関数
    pub fn fmap<A, B, F>(value: Vec<A>, f: F) -> Vec<B>
    where
        F: Fn(&A) -> B,
    {
        value.iter().map(f).collect::<Vec<B>>()
    }

    /// Vec<A>に対するap実装のためのヘルパー関数
    pub fn ap<A, B, F>(value: Vec<A>, fs: Vec<F>) -> Vec<B>
    where
        F: Fn(&A) -> B,
    {
        let zipped = value.iter().zip(fs.iter());
        zipped.map(|(x, f)| f(x)).collect::<Vec<B>>()
    }

    /// Vec<A>に対するbind実装のためのヘルパー関数
    pub fn bind<A, B, F>(value: Vec<A>, f: F) -> Vec<B>
    where
        F: FnMut(&A) -> Vec<B>,
    {
        value.iter().flat_map(f).collect()
    }

    /// Vec<A>に対するpure実装のためのヘルパー関数
    pub fn pure<A>(value: A) -> Vec<A> {
        vec![value]
    }

    /// Vec<A>に対するunit実装のためのヘルパー関数
    pub fn unit() -> Vec<()> {
        vec![()]
    }
}

/// 数値型に対する共通の実装パターン
pub mod numeric {
    /// 数値型に対するfmap実装のためのヘルパー関数
    pub fn fmap<A, B, F>(value: A, f: F) -> B
    where
        F: Fn(&A) -> B,
    {
        f(&value)
    }

    /// 数値型に対するap実装のためのヘルパー関数
    pub fn ap<A, B, F>(value: A, fs: F) -> B
    where
        F: Fn(&A) -> B,
    {
        fs(&value)
    }

    /// 数値型に対するbind実装のためのヘルパー関数
    pub fn bind<A, B, F>(value: A, f: F) -> B
    where
        F: FnOnce(&A) -> B,
    {
        f(&value)
    }

    /// 数値型に対するpure実装のためのヘルパー関数
    pub fn pure<A>(value: A) -> A {
        value
    }

    /// 数値型に対するunit実装のためのヘルパー関数
    pub fn unit() -> () {
        ()
    }

    /// 数値型に対するshow実装のためのヘルパー関数
    pub fn show<A: std::fmt::Display>(value: A) -> String {
        value.to_string()
    }
}

/// Show型クラスに対する共通の実装パターン
pub mod show {
    use std::fmt::Display;
    use std::rc::Rc;

    /// Rc<A>型に対するshow実装のためのヘルパー関数
    pub mod rc {
        use std::fmt::Display;
        use std::rc::Rc;

        /// Rc<A>に対するshow実装のためのヘルパー関数
        pub fn show<A: Display>(value: Rc<A>) -> String {
            value.to_string()
        }
    }

    /// Box<A>型に対するshow実装のためのヘルパー関数
    pub mod boxed {
        use std::fmt::Display;

        /// Box<A>に対するshow実装のためのヘルパー関数
        pub fn show<A: Display>(value: Box<A>) -> String {
            value.to_string()
        }
    }

    /// Option<A>型に対するshow実装のためのヘルパー関数
    pub mod option {
        use std::fmt::Display;

        /// Option<A>に対するshow実装のためのヘルパー関数
        pub fn show<A: Display>(value: Option<A>) -> String {
            match value {
                Some(v) => format!("Some({})", v),
                None => "None".to_string(),
            }
        }
    }

    /// Result<A, E>型に対するshow実装のためのヘルパー関数
    pub mod result {
        use std::fmt::Display;

        /// Result<A, E>に対するshow実装のためのヘルパー関数
        pub fn show<A: Display, E: Display>(value: Result<A, E>) -> String {
            match value {
                Ok(v) => format!("Ok({})", v),
                Err(e) => format!("Err({})", e),
            }
        }
    }

    /// Vec<A>型に対するshow実装のためのヘルパー関数
    pub mod vec {
        use std::fmt::Display;

        /// Vec<A>に対するshow実装のためのヘルパー関数
        pub fn show<A: Display>(value: Vec<A>) -> String {
            let items = value
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            format!("[{}]", items)
        }
    }
}
