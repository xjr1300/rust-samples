/// ```
/// pub fn err(self) -> Option<E>
/// ```
/// Converts from Result<T, E> to Option<E>.
///
/// Converts self into an Option<E>, consuming self, and discarding the success value, if any.
///
/// `Result<T, E>`から`Option<E>`に変換する。
///
/// 自分自身を`Option<E>`に変換して、自分自身を消費して、成功した値がある場合は値を破棄する。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(2);
/// assert_eq!(x.err(), None);
///
/// let x: Result<u32, &str> = Err("Nothing here");
/// assert_eq!(x.err(), Some("Nothing here"));
/// ```
fn main() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.err(), None);
    let x: Result<u32, &str> = Err("spam");
    assert_eq!(x.err(), Some("spam"));
}
