/// ```
/// pub fn or<F>(self, res: Result<T, F>) -> Result<T, F>
/// ```
///
/// Returns res if the result is Err, otherwise returns the Ok value of self.
///
/// もし`Result`が`Err`の場合は`res`を返却し、そうでない場合は自身の`Ok`の値を返却する。
///
/// Arguments passed to or are eagerly evaluated; if you are passing the result of a function call,
/// it is recommended to use or_else, which is lazily evaluated.
///
/// `or`に渡された引数は即時に評価される。もし、関数呼び出しの結果を渡す場合は、遅延評価される`or_else`を使用することを推奨する。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(2);
/// let y: Result<u32, &str> = Err("late error");
/// assert_eq!(x.or(y), Ok(2));
///
/// let x: Result<u32, &str> = Err("early error");
/// let y: Result<u32, &str> = Ok(2);
/// assert_eq!(x.or(y), Ok(2));
///
/// let x: Result<u32, &str> = Err("not a 2");
/// let y: Result<u32, &str> = Err("late error");
/// assert_eq!(x.or(y), Err("late error"));
///
/// let x: Result<u32, &str> = Ok(2);
/// let y: Result<u32, &str> = Ok(100);
/// assert_eq!(x.or(y), Ok(2));
/// ```
fn main() {
    let x: Result<u32, &str> = Ok(41);
    let y: Result<u32, &str> = Err("late error");
    assert_eq!(x.or(y), Ok(41));

    let x: Result<u32, &str> = Err("early error");
    let y: Result<u32, &str> = Ok(41);
    assert_eq!(x.or(y), Ok(41));

    let x: Result<u32, &str> = Err("early error");
    let y: Result<u32, &str> = Err("late error");
    assert_eq!(x.or(y), Err("late error"));

    let x: Result<u32, u32> = Ok(41);
    let y: Result<u32, u32> = Ok(42);
    assert_eq!(x.or(y), Ok(41));
}
