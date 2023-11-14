/// ```
/// pub fn unwrap(self) -> T
/// where
///     E: Debug,
/// ```
///
/// Returns the contained Ok value, consuming the self value.
///
/// `Ok`に含まれた値を返却して、自身の値を消費する。
///
/// Because this function may panic, its use is generally discouraged.
/// Instead, prefer to use pattern matching and handle the Err case explicitly, or call unwrap_or, unwrap_or_else, or unwrap_or_default.
///
/// この関数はパニックする可能性があるため、一般的にはその使用は推奨されない。
/// 代わりに、パターンマッチングを使用して、明示的に`Err`ケースを処理するか、`unwrap_or`、`unwrap_or_else`、`unwrap_or_default`を呼び出すことを推奨する。
///
/// ## Panics
///
/// Panics if the value is an Err, with a panic message provided by the Err’s value.
///
/// もし値が`Err`の場合、`Err`の値によって提供されるパニックメッセージとともにパニックする。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(2);
/// assert_eq!(x.unwrap(), 2);
///
/// let x: Result<u32, &str> = Err("emergency failure");
/// x.unwrap(); // panics with `emergency failure`
/// ```
fn main() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);

    let x: Result<u32, &str> = Err("emergency failure");
    x.unwrap(); // panic!
}
