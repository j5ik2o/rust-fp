use std::fmt::Display;
#[allow(dead_code)]
use std::rc::Rc;

/// Show型クラスは、値を文字列表現に変換するための型クラスです。
///
/// Show型クラスは、様々な型の値を一貫した方法で文字列に変換する機能を提供します。
/// これにより、デバッグ出力やログ出力、ユーザーインターフェースでの表示などが容易になります。
///
/// # 型クラス階層における位置
///
/// Show型クラスは独立した型クラスであり、他の型クラスとの直接的な階層関係はありません。
/// しかし、他の多くの型クラスと組み合わせて使用することができます。
///
/// # Show型クラスの法則
///
/// Show型クラスは以下の法則を満たすことが望ましいです：
///
/// 1. 一貫性の法則：同じ値に対するshow呼び出しは常に同じ文字列を返すべきです
///    ```rust,ignore
///    x.show() == x.show()
///    ```
///
/// 2. 可読性の法則：返される文字列は人間が読みやすい形式であるべきです
///
/// # 型パラメータ
///
/// * `Elm` - 文字列に変換される値の型
///
/// # メソッド
///
/// * `show` - 値を文字列表現に変換する
///
/// # 注意
///
/// `show`メソッドは、Rustの標準ライブラリの`Display`トレイトと似ていますが、
/// 型クラスとしての一貫性と拡張性を提供します。
pub trait Show {
    type Elm;

    fn show(self) -> String;
}

use crate::impl_show_for_numeric;

impl_show_for_numeric!();

/// `Rc<A>`に対するShow型クラスの実装
///
/// この実装により、参照カウント型のコンテナ内の値を文字列表現に変換することができます。
///
/// # 例
///
/// ```
/// use std::rc::Rc;
/// use rust_fp_categories::Show;
///
/// let value = Rc::new(5);
/// let result = value.show();
/// assert_eq!(result, "5");
/// ```
impl<A: Display> Show for Rc<A> {
    type Elm = A;

    fn show(self) -> String {
        crate::common::show::rc::show(self)
    }
}

/// `Box<A>`に対するShow型クラスの実装
///
/// この実装により、ヒープ割り当て型のコンテナ内の値を文字列表現に変換することができます。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Show;
///
/// let value = Box::new(5);
/// let result = value.show();
/// assert_eq!(result, "5");
/// ```
impl<A: Display> Show for Box<A> {
    type Elm = A;

    fn show(self) -> String {
        crate::common::show::boxed::show(self)
    }
}

/// `Option<A>`に対するShow型クラスの実装
///
/// この実装により、Optionコンテナ内の値を文字列表現に変換することができます。
/// Noneの場合は"None"という文字列を返します。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Show;
///
/// let some_value = Some(5);
/// let result = some_value.show();
/// assert_eq!(result, "Some(5)");
///
/// let none_value: Option<i32> = None;
/// let result = none_value.show();
/// assert_eq!(result, "None");
/// ```
impl<A: Display> Show for Option<A> {
    type Elm = A;

    fn show(self) -> String {
        crate::common::show::option::show(self)
    }
}

/// `Result<A, E>`に対するShow型クラスの実装
///
/// この実装により、Resultコンテナ内の値を文字列表現に変換することができます。
/// Errの場合はエラー値の文字列表現を返します。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Show;
///
/// let ok_value: Result<i32, &str> = Ok(5);
/// let result = ok_value.show();
/// assert_eq!(result, "Ok(5)");
///
/// let err_value: Result<i32, &str> = Err("エラー");
/// let result = err_value.show();
/// assert_eq!(result, "Err(エラー)");
/// ```
impl<A: Display, E: Display> Show for Result<A, E> {
    type Elm = A;

    fn show(self) -> String {
        crate::common::show::result::show(self)
    }
}

/// `Vec<A>`に対するShow型クラスの実装
///
/// この実装により、ベクトル内の各要素を文字列表現に変換することができます。
/// 空のベクトルの場合は"[]"という文字列を返します。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Show;
///
/// let values = vec![1, 2, 3, 4, 5];
/// let result = values.show();
/// assert_eq!(result, "[1, 2, 3, 4, 5]");
///
/// let empty: Vec<i32> = vec![];
/// let result = empty.show();
/// assert_eq!(result, "[]");
/// ```
impl<A: Display> Show for Vec<A> {
    type Elm = A;

    fn show(self) -> String {
        crate::common::show::vec::show(self)
    }
}

/// `String`に対するShow型クラスの実装
///
/// この実装により、文字列を文字列表現に変換することができます。
/// 基本的には元の文字列をそのまま返します。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Show;
///
/// let value = "Hello, world!".to_string();
/// let result = value.show();
/// assert_eq!(result, "Hello, world!");
/// ```
impl Show for String {
    type Elm = String;

    fn show(self) -> String {
        self
    }
}

/// `&str`に対するShow型クラスの実装
///
/// この実装により、文字列スライスを文字列表現に変換することができます。
/// 文字列スライスをStringに変換して返します。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Show;
///
/// let value = "Hello, world!";
/// let result = value.show();
/// assert_eq!(result, "Hello, world!");
/// ```
impl<'a> Show for &'a str {
    type Elm = &'a str;

    fn show(self) -> String {
        self.to_string()
    }
}

/// `bool`に対するShow型クラスの実装
///
/// この実装により、真偽値を文字列表現に変換することができます。
/// trueの場合は"true"、falseの場合は"false"という文字列を返します。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Show;
///
/// let value = true;
/// let result = value.show();
/// assert_eq!(result, "true");
///
/// let value = false;
/// let result = value.show();
/// assert_eq!(result, "false");
/// ```
impl Show for bool {
    type Elm = bool;

    fn show(self) -> String {
        self.to_string()
    }
}

/// `char`に対するShow型クラスの実装
///
/// この実装により、文字を文字列表現に変換することができます。
/// 文字を含む文字列を返します。
///
/// # 例
///
/// ```
/// use rust_fp_categories::Show;
///
/// let value = 'a';
/// let result = value.show();
/// assert_eq!(result, "a");
/// ```
impl Show for char {
    type Elm = char;

    fn show(self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn test_show_for_rc() {
        let value = Rc::new(5);
        assert_eq!(value.show(), "5");
    }

    #[test]
    fn test_show_for_box() {
        let value = Box::new(5);
        assert_eq!(value.show(), "5");
    }

    #[test]
    fn test_show_for_option() {
        let some_value = Some(5);
        assert_eq!(some_value.show(), "Some(5)");

        let none_value: Option<i32> = None;
        assert_eq!(none_value.show(), "None");
    }

    #[test]
    fn test_show_for_result() {
        let ok_value: Result<i32, &str> = Ok(5);
        assert_eq!(ok_value.show(), "Ok(5)");

        let err_value: Result<i32, &str> = Err("エラー");
        assert_eq!(err_value.show(), "Err(エラー)");
    }

    #[test]
    fn test_show_for_vec() {
        let values = vec![1, 2, 3, 4, 5];
        assert_eq!(values.show(), "[1, 2, 3, 4, 5]");

        let empty: Vec<i32> = vec![];
        assert_eq!(empty.show(), "[]");
    }

    #[test]
    fn test_show_for_string() {
        let value = "Hello, world!".to_string();
        assert_eq!(value.show(), "Hello, world!");
    }

    #[test]
    fn test_show_for_str() {
        let value = "Hello, world!";
        assert_eq!(value.show(), "Hello, world!");
    }

    #[test]
    fn test_show_for_bool() {
        assert_eq!(true.show(), "true");
        assert_eq!(false.show(), "false");
    }

    #[test]
    fn test_show_for_char() {
        assert_eq!('a'.show(), "a");
        assert_eq!('あ'.show(), "あ");
    }
}
