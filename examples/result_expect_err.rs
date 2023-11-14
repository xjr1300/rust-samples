/// ```
/// pub fn expect_err(self, msg: &str) -> E
/// where
///     T: Debug,
/// ```
///
/// Returns the contained Err value, consuming the self value.
///
/// 含まれている`Err`の値を返却して、自身の値を消費する。
///
/// ## Panics
///
/// Panics if the value is an Ok, with a panic message including the passed message, and the content of the Ok.
///
/// もし、値が｀Ok`の場合は、渡したメッセージと`Ok`の内容を含むパニックメッセージと一緒にパニックする。
///
/// ```rust
/// let x: Result<u32, &str> = Ok(10);
/// x.expect_err("Testing expect_err"); // panics with `Testing expect_err: 10`
/// ```
fn main() {
    let x: Result<&str, i32> = Ok("ok");
    x.expect_err("Testing expect_err"); // panics with `Testing expect_err: "ok"`
}
