/// ```
/// pub fn unwrap_or_else<F>(self, op: F) -> T
/// where
///     F: FnOnce(E) -> T,
/// ```
///
/// Returns the contained Ok value or computes it from a closure.
///
/// `Ok`に含まれた値か、クロージャーから返された値を返却する。
///
/// ```rust
/// fn count(x: &str) -> usize { x.len() }
///
/// assert_eq!(Ok(2).unwrap_or_else(count), 2);
/// assert_eq!(Err("foo").unwrap_or_else(count), 3);
/// ```
fn count(x: &str) -> usize {
    x.len()
}

fn main() {
    assert_eq!(Ok(2).unwrap_or_else(count), 2);
    assert_eq!(Err("spam").unwrap_or_else(count), 4);
}
