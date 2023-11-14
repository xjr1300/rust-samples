/// ```
/// pub fn unwrap_err(self) -> E
/// where
///     T: Debug,
/// ```
///
/// Returns the contained Err value, consuming the self value.
///
/// `Err`に含まれた値を返却して、自身の値を消費する。
///
/// ## Panics
///
/// Panics if the value is an Ok, with a custom panic message provided by the Ok’s value.
///
/// もし値が`Ok`の場合、`Ok`の値によって提供されるカスタムパニックメッセージと一緒にパニックする。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(2);
/// x.unwrap_err(); // panics with `2`
///
/// let x: Result<u32, &str> = Err("emergency failure");
/// assert_eq!(x.unwrap_err(), "emergency failure");
/// ```
fn main() {
    let x: Result<u32, &str> = Err("emergency failure");
    assert_eq!(x.unwrap_err(), "emergence failure");

    let x: Result<u32, &str> = Ok(2);
    x.unwrap_err(); // panic with `2`
}
