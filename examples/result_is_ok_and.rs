/// ```
/// pub fn is_ok_and(self, f: impl FnOnce(T) -> bool) -> bool
/// ```
///
/// Returns true if the result is Ok and the value inside of it matches a predicate.
///
/// もし｀Result`が`Ok`であり、その中身の値が述語と一致する場合に`true`を返却する。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(2);
/// assert_eq!(x.is_ok_and(|x| x > 1), true);
///
/// let x: Result<u32, &str> = Ok(0);
/// assert_eq!(x.is_ok_and(|x| x > 1), false);
///
/// let x: Result<u32, &str> = Err("hey");
/// assert_eq!(x.is_ok_and(|x| x > 1), false);
/// ```
fn main() {
    let x: Result<u32, &str> = Ok(41);
    let result = x.is_ok_and(|x| x == 41);
    assert!(result);

    let x: Result<u32, &str> = Ok(41);
    let result = x.is_ok_and(|x| x != 41);
    assert!(!result);

    let x: Result<u32, &str> = Err("spam");
    let result = x.is_ok_and(|x| x == 41);
    assert!(!result);
}
