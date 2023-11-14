/// ```
/// pub fn is_err_and(self, f: impl FnOnce(E) -> bool) -> bool
/// ```
///
/// Returns true if the result is Err and the value inside of it matches a predicate.
///
/// `Result`が`Err`であり、その中身の値が述語と一致する場合は`true`を返却する。
///
/// このメソッドは自身を消費する。
///
/// ```rust
/// use std::io::{Error, ErrorKind};
///
/// let x: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "!"));
/// assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), true);
///
/// let x: Result<u32, Error> = Err(Error::new(ErrorKind::PermissionDenied, "!"));
/// assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);
///
/// let x: Result<u32, Error> = Ok(123);
/// assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);
/// ```
use std::io::{Error, ErrorKind};

fn main() {
    let x: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "!"));
    let result = x.is_err_and(|x| x.kind() == ErrorKind::NotFound);
    assert!(result);
    let x: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "!"));
    let result = x.is_err_and(|x| x.kind() == ErrorKind::PermissionDenied);
    assert!(!result);
    let x: Result<u32, Error> = Ok(41);
    let result = x.is_err_and(|x| x.kind() == ErrorKind::NotFound);
    assert!(!result);
}
