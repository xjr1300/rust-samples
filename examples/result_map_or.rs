/// ```
/// pub fn map_or<U, F>(self, default: U, f: F) -> U
/// where
///     F: FnOnce(T) -> U,
/// ```
///
/// Returns the provided default (if Err), or applies a function to the contained value (if Ok),
///
/// もし`Err`の場合は提供されたデフォルトを返却し、`Ok`の場合は含まれた値に対して関数を適用する。
///
/// Arguments passed to map_or are eagerly evaluated; if you are passing the result of a function call,
/// it is recommended to use map_or_else, which is lazily evaluated.
///
/// `map_or`に渡される値は即座に評価される。もし関数呼び出しの結果を渡す場合は、遅延評価される`map_or_else`を使用することを推奨する。
///
/// ```rust
/// let x: Result<_, &str> = Ok("foo");
/// assert_eq!(x.map_or(42, |v| v.len()), 3);
///
/// let x: Result<&str, _> = Err("bar");
/// assert_eq!(x.map_or(42, |v| v.len()), 42);
/// ```
fn main() {
    let x: Result<_, &str> = Ok("foo");
    assert_eq!(x.map_or(42, |v| v.len()), 3); // `x`が`Err`の場合はデフォルトの`42`を返却して、`Ok`の場合は関数を適用

    let x: Result<&str, _> = Err("bar");
    assert_eq!(x.map_or(42, |v| v.len()), 42);
}
