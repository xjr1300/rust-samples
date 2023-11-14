/// ```
/// pub const fn is_ok(&self) -> bool
/// ```
///
/// Returns true if the result is Ok.
///
/// もし`Result`が`Ok`の場合は`true`を返却する。
///
/// ```rust
/// let x: Result<i32, &str> = Ok(-3);
/// assert_eq!(x.is_ok(), true);
///
/// let x: Result<i32, &str> = Err("Some error message");
/// assert_eq!(x.is_ok(), false);
/// ```
fn main() {
    let x: Result<i32, &str> = Ok(41);
    assert!(x.is_ok());
    let x: Result<i32, &str> = Err("Some error message");
    assert!(!x.is_ok());
}
