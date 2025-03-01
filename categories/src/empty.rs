pub trait Empty {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;
}

use crate::{impl_empty_for_integer, impl_empty_for_float};

impl_empty_for_integer!();
impl_empty_for_float!();

impl<T> Empty for Vec<T> {
    fn empty() -> Vec<T> {
        vec![]
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Empty for String {
    fn empty() -> String {
        "".to_string()
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
