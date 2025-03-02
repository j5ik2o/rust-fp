//! Common macros for implementing traits for numeric types

/// List of all integer numeric types
#[macro_export]
macro_rules! integer_types {
    () => { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 };
}

/// List of all floating-point numeric types
#[macro_export]
macro_rules! float_types {
    () => { f32 f64 };
}

/// List of all numeric types (both integer and floating-point)
#[macro_export]
macro_rules! numeric_types {
    () => { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 };
}

/// Implements a simple marker trait for all numeric types
#[macro_export]
macro_rules! impl_marker_trait_for_numeric {
    ($trait_name:ident) => {
        impl $trait_name for usize {}
        impl $trait_name for u8 {}
        impl $trait_name for u16 {}
        impl $trait_name for u32 {}
        impl $trait_name for u64 {}
        impl $trait_name for u128 {}
        impl $trait_name for isize {}
        impl $trait_name for i8 {}
        impl $trait_name for i16 {}
        impl $trait_name for i32 {}
        impl $trait_name for i64 {}
        impl $trait_name for i128 {}
        impl $trait_name for f32 {}
        impl $trait_name for f64 {}
    };
}

/// Implements a trait with a simple combine method for all numeric types
#[macro_export]
macro_rules! impl_semigroup_for_numeric {
    () => {
        impl Semigroup for usize {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for u8 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for u16 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for u32 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for u64 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for u128 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for isize {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for i8 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for i16 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for i32 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for i64 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for i128 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for f32 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
        impl Semigroup for f64 {
            fn combine(self, other: Self) -> Self {
                self + other
            }
        }
    };
}

/// Implements the Empty trait for integer numeric types
#[macro_export]
macro_rules! impl_empty_for_integer {
    () => {
        impl Empty for usize {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for u8 {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for u16 {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for u32 {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for u64 {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for u128 {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for isize {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for i8 {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for i16 {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for i32 {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for i64 {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
        impl Empty for i128 {
            fn empty() -> Self {
                0
            }
            fn is_empty(&self) -> bool {
                *self == 0
            }
        }
    };
}

/// Implements the Empty trait for floating-point numeric types
#[macro_export]
macro_rules! impl_empty_for_float {
    () => {
        impl Empty for f32 {
            fn empty() -> Self {
                0.0
            }
            fn is_empty(&self) -> bool {
                *self == 0.0
            }
        }
        impl Empty for f64 {
            fn empty() -> Self {
                0.0
            }
            fn is_empty(&self) -> bool {
                *self == 0.0
            }
        }
    };
}

/// Implements the Functor trait for all numeric types
#[macro_export]
macro_rules! impl_functor_for_numeric {
    () => {
        impl Functor for usize {
            type Elm = usize;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for u8 {
            type Elm = u8;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for u16 {
            type Elm = u16;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for u32 {
            type Elm = u32;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for u64 {
            type Elm = u64;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for u128 {
            type Elm = u128;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for isize {
            type Elm = isize;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for i8 {
            type Elm = i8;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for i16 {
            type Elm = i16;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for i32 {
            type Elm = i32;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for i64 {
            type Elm = i64;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for i128 {
            type Elm = i128;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for f32 {
            type Elm = f32;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
        impl Functor for f64 {
            type Elm = f64;
            type M<U: Clone> = U;
            fn fmap<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::fmap(self, f)
            }
        }
    };
}

/// Implements the Bind trait for all numeric types
#[macro_export]
macro_rules! impl_bind_for_numeric {
    () => {
        impl Bind for usize {
            type Elm = usize;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for u8 {
            type Elm = u8;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for u16 {
            type Elm = u16;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for u32 {
            type Elm = u32;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for u64 {
            type Elm = u64;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for u128 {
            type Elm = u128;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for isize {
            type Elm = isize;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for i8 {
            type Elm = i8;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for i16 {
            type Elm = i16;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for i32 {
            type Elm = i32;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for i64 {
            type Elm = i64;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for i128 {
            type Elm = i128;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for f32 {
            type Elm = f32;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
        impl Bind for f64 {
            type Elm = f64;
            type M<U: Clone> = U;
            fn bind<B: Clone, F>(self, f: F) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> Self::M<B>,
            {
                crate::common::numeric::bind(self, f)
            }
        }
    };
}

/// Implements the Apply trait for all numeric types
#[macro_export]
macro_rules! impl_apply_for_numeric {
    () => {
        impl Apply for usize {
            type Elm = usize;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for u8 {
            type Elm = u8;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for u16 {
            type Elm = u16;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for u32 {
            type Elm = u32;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for u64 {
            type Elm = u64;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for u128 {
            type Elm = u128;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for isize {
            type Elm = isize;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for i8 {
            type Elm = i8;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for i16 {
            type Elm = i16;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for i32 {
            type Elm = i32;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for i64 {
            type Elm = i64;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for i128 {
            type Elm = i128;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for f32 {
            type Elm = f32;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
        impl Apply for f64 {
            type Elm = f64;
            type M<U: Clone> = U;
            fn ap<B: Clone, F: Clone>(self, fs: Self::M<F>) -> Self::M<B>
            where
                F: Fn(&Self::Elm) -> B,
            {
                crate::common::numeric::ap(self, fs)
            }
        }
    };
}

/// Implements the Pure trait for all numeric types
#[macro_export]
macro_rules! impl_pure_for_numeric {
    () => {
        impl Pure for usize {
            type Elm = usize;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for u8 {
            type Elm = u8;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for u16 {
            type Elm = u16;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for u32 {
            type Elm = u32;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for u64 {
            type Elm = u64;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for u128 {
            type Elm = u128;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for isize {
            type Elm = isize;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for i8 {
            type Elm = i8;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for i16 {
            type Elm = i16;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for i32 {
            type Elm = i32;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for i64 {
            type Elm = i64;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for i128 {
            type Elm = i128;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for f32 {
            type Elm = f32;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
        impl Pure for f64 {
            type Elm = f64;
            type M<U: Clone> = U;
            fn pure(value: Self::Elm) -> Self::M<Self::Elm> {
                crate::common::numeric::pure(value)
            }
            fn unit() -> Self::M<()> {
                crate::common::numeric::unit()
            }
        }
    };
}
