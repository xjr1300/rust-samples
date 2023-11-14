/// ```
/// pub fn iter_mut(&mut self) -> IterMut<'_, T>
/// ```
///
/// Returns a mutable iterator over the possibly contained value.
///
/// 含まれた可能性のある値に対する可変イテレーターを返却する。
///
/// The iterator yields one value if the result is Result::Ok, otherwise none.
///
/// そのイテレーターは`Result::Ok`の場合は1つの値を返却して、そうでない場合は`None`を返却する。
///
/// ```rust
/// let mut x: Result<u32, &str> = Ok(7);
/// match x.iter_mut().next() {
///     Some(v) => *v = 40,
///     None => {},
/// }
/// assert_eq!(x, Ok(40));
///
/// let mut x: Result<u32, &str> = Err("nothing!");
/// assert_eq!(x.iter_mut().next(), None);
/// ```
fn main() {
    let mut x: Result<u32, &str> = Ok(41);
    match x.iter_mut().next() {
        Some(v) => *v = 42,
        None => {}
    }
    assert_eq!(x, Ok(42));

    let mut x: Result<u32, &str> = Err("spam");
    assert_eq!(x.iter_mut().next(), None);
}
