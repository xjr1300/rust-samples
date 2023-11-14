/// ```
/// pub fn unwrap_or(self, default: T) -> T
/// ```
///
/// Returns the contained Ok value or a provided default.
///
/// `Ok`に含まれた値が、提供されたデフォルト値を返却する。
///
/// Arguments passed to unwrap_or are eagerly evaluated; if you are passing the result of a function call,
/// it is recommended to use unwrap_or_else, which is lazily evaluated.
///
/// `unwrap_or`に渡された引数は即座に評価される。もし関数呼び出しの結果を渡している場合は、遅延評価される`unwrap_or_else`を使用することが推奨される。
///
/// ```rust
/// let default = 2;
/// let x: Result<u32, &str> = Ok(9);
/// assert_eq!(x.unwrap_or(default), 9);
///
/// let x: Result<u32, &str> = Err("error");
/// assert_eq!(x.unwrap_or(default), default);
/// ```
fn main() {
    let default = 2;
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or(default), 9);

    let x: Result<u32, &str> = Err("error");
    assert_eq!(x.unwrap_or(default), 2);
}
