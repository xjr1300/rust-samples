/// ```
/// pub unsafe fn unwrap_err_unchecked(self) -> E
/// ```
///
/// Returns the contained Err value, consuming the self value, without checking that the value is not an Ok.
///
/// `Err`に含まれた値を返却して、自身の値を消費するが、値が`Ok`でないことをチェックしない。
///
/// ## Safety
///
/// Calling this method on an Ok is undefined behavior.
///
/// `Ok`に対してこのメソッドの呼び出しは、未定義な動作になる。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(2);
/// unsafe { x.unwrap_err_unchecked() }; // Undefined behavior!
///
/// let x: Result<u32, &str> = Err("emergency failure");
/// assert_eq!(unsafe { x.unwrap_err_unchecked() }, "emergency failure");
/// ```
fn main() {
    let x: Result<u32, &str> = Err("emergency failure");
    assert_eq!(unsafe { x.unwrap_err_unchecked() }, "emergency failure");

    let x: Result<u32, &str> = Ok(2);
    unsafe { x.unwrap_err_unchecked() }; // 未定義な動作
}
