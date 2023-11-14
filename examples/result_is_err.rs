/// ```
/// pub const fn is_err(&self) -> bool
/// ```
///
/// Returns true if the result is Err.
///
/// `Result`が`Err`の場合は`true`を返却する。
///
/// ```
/// let x: Result<i32, &str> = Ok(-3);
/// assert_eq!(x.is_err(), false);
///
/// let x: Result<i32, &str> = Err("Some error message");
/// assert_eq!(x.is_err(), true);
/// ```
fn main() {
    let x: Result<i32, &str> = Ok(41);
    assert!(!x.is_err());
    let x: Result<i32, &str> = Err("Some error message");
    assert!(x.is_err());
}
