/// ```
/// pub fn iter(&self) -> Iter<'_, T>
/// ```
///
/// Returns an iterator over the possibly contained value.
///
/// 含まれた可能性のある値に対するイテレーターを返却する。
///
/// The iterator yields one value if the result is Result::Ok, otherwise none.
///
/// そのイテレーターは`Result::Ok`の場合は1つの値を返却して、そうでない場合は`None`を返却する。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(7);
/// assert_eq!(x.iter().next(), Some(&7));
///
/// let x: Result<u32, &str> = Err("nothing!");
/// assert_eq!(x.iter().next(), None);
/// ```
fn main() {
    let x: Result<u32, &str> = Ok(41);
    assert_eq!(x.iter().next(), Some(&41));
    let x: Result<u32, &str> = Err("spam");
    assert_eq!(x.iter().next(), None);
}
