/// ```
/// pub fn ok(self) -> Option<T>
/// ```
///
/// Converts from Result<T, E> to Option<T>.
///
/// `Result<T, E>`を`Option<T>`に変換する。
///
/// Converts self into an Option<T>, consuming self, and discarding the error, if any.
///
/// 自身を`Option<T>`に変換して、自身を消費し、エラーがあれば破棄する。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(2);
/// assert_eq!(x.ok(), Some(2));
///
/// let x: Result<u32, &str> = Err("Nothing here");
/// assert_eq!(x.ok(), None);
/// ```
fn main() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.ok(), Some(2));

    let x: Result<u32, &str> = Err("spam");
    assert_eq!(x.ok(), None);
}
