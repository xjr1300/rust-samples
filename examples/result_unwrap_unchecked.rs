/// ```
/// pub unsafe fn unwrap_unchecked(self) -> T
/// ```
///
/// Returns the contained Ok value, consuming the self value, without checking that the value is not an Err.
///
/// 値が`Err`であることを確認しないで、`Ok`に含まれた値を返却して、自身を消費する。
///
/// ## Safety
///
/// Calling this method on an Err is undefined behavior.
///
/// `Err`に対するこのメソッドの呼び出しは未定義な動作になる。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(2);
/// assert_eq!(unsafe { x.unwrap_unchecked() }, 2);
///
/// let x: Result<u32, &str> = Err("emergency failure");
/// unsafe { x.unwrap_unchecked(); } // Undefined behavior!
/// ```
fn main() {
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(unsafe { x.unwrap_unchecked() }, 2);

    let x: Result<u32, &str> = Err("emergency failure");
    unsafe {
        // Undefined behavior!
        x.unwrap_unchecked();
    }
}
