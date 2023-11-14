/// ```
/// pub fn or_else<F, O>(self, op: O) -> Result<T, F>
/// where
///     O: FnOnce(E) -> Result<T, F>,
/// ```
///
/// Calls op if the result is Err, otherwise returns the Ok value of self.
///
/// もし`Result`が`Err`の場合は`op`を呼び出し、そうでない場合は自身の`Ok`の値を返却する。
///
/// This function can be used for control flow based on result values.
///
/// この関数は、`Result`の値に基づいたコントロールフローに使用できる。
///
/// ```rust
/// fn sq(x: u32) -> Result<u32, u32> { Ok(x * x) }
/// fn err(x: u32) -> Result<u32, u32> { Err(x) }
///
/// assert_eq!(Ok(2).or_else(sq).or_else(sq), Ok(2));
/// assert_eq!(Ok(2).or_else(err).or_else(sq), Ok(2));
/// assert_eq!(Err(3).or_else(sq).or_else(err), Ok(9));
/// assert_eq!(Err(3).or_else(err).or_else(err), Err(3));
/// ```
fn sq(x: u32) -> Result<u32, u32> {
    Ok(x * x)
}

fn err(x: u32) -> Result<u32, u32> {
    Err(x)
}

fn main() {
    assert_eq!(Ok(2).or_else(sq), Ok(2));
    assert_eq!(Err(2).or_else(sq), Ok(4));
    assert_eq!(Ok(2).or_else(err), Ok(2));
    assert_eq!(Err(2).or_else(err), Err(2));
}
